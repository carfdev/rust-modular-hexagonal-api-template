use uuid::Uuid;
use chrono::Utc;
use crate::common::{errors::AppError, config::AppConfig};
use crate::modules::users::domain::{entity::{User, NewUser}, repository::UserRepository};
use crate::modules::auth::{
    domain::{
        entity::{UserSession, NewUserSession, token::{NewEmailVerificationToken, NewPasswordResetToken}},
        repository::{SessionRepository, verification::VerificationTokenRepository},
    },
    infrastructure::password_service::PasswordService,
    application::token_service::TokenService,
};
use crate::modules::email::domain::service::{EmailService, EmailRecipient};

pub struct AuthService<U, S, V, E> 
where 
    U: UserRepository, 
    S: SessionRepository,
    V: VerificationTokenRepository,
    E: EmailService
{
    user_repo: U,
    session_repo: S,
    verification_repo: V,
    email_service: E,
    token_service: TokenService,
    config: AppConfig,
}

impl<U, S, V, E> AuthService<U, S, V, E>
where 
    U: UserRepository, 
    S: SessionRepository,
    V: VerificationTokenRepository,
    E: EmailService 
{
    pub fn new(
        user_repo: U, 
        session_repo: S, 
        verification_repo: V, 
        email_service: E, 
        token_service: TokenService, 
        config: AppConfig
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            verification_repo,
            email_service,
            token_service,
            config,
        }
    }

    pub async fn register(&self, email: String, password: String) -> Result<User, AppError> {
        if self.user_repo.find_by_email(&email)?.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        let password_hash = PasswordService::hash_password(&password)?;
        
        let new_user = NewUser {
            email: email.clone(),
            password_hash,
        };

        let user = self.user_repo.create(new_user)?;

        // Generate verification token
        let token = Uuid::new_v4().to_string(); // Use a simpler token or same logic
        let token_hash = PasswordService::hash_password(&token)?;
        
        let expiration = Utc::now().naive_utc() + chrono::Duration::hours(24);

        let new_token = NewEmailVerificationToken {
            user_id: user.id,
            token_hash,
            expires_at: expiration,
        };

        self.verification_repo.create_email_verification(new_token)?;

        // Send email
        let recipient = EmailRecipient {
            email: email.clone(),
            name: None,
        };

        self.email_service.send_verification_email(&recipient, &format!("{}:{}", user.id, token)).await?;


        Ok(user)
    }

    pub async fn login(&self, email: String, password: String, user_agent: Option<String>, ip_address: Option<String>) -> Result<(String, String), AppError> {
        let user = self.user_repo.find_by_email(&email)?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        if !PasswordService::verify_password(&password, &user.password_hash)? {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        if !user.is_active {
            return Err(AppError::Forbidden("Account is disabled".to_string()));
        }

        // Update last login timestamp
        self.user_repo.update_last_login(user.id)?;
        
        let (session, refresh_token) = self.create_session(user.id, user_agent, ip_address).await?;
        
        let roles = self.user_repo.get_roles(user.id)?;
        let access_token = self.token_service.generate_access_token(user.id, session.id, roles)?;

        // Return "session_id:refresh_token"
        let combined_refresh_token = format!("{}:{}", session.id, refresh_token);
        Ok((access_token, combined_refresh_token)) 
    }

    pub async fn verify_email(&self, token: String) -> Result<(), AppError> {
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::Unauthorized("Invalid token format".to_string()));
        }
        
        let user_id = Uuid::parse_str(parts[0]).map_err(|_| AppError::Unauthorized("Invalid token format".to_string()))?;
        let token_raw = parts[1];
        
        let verification = self.verification_repo.find_email_verification_by_user(user_id)?
            .ok_or(AppError::Unauthorized("Invalid or expired token".to_string()))?;

        if verification.used {
             return Err(AppError::ValidationError(validator::ValidationErrors::new())); 
        }

        if !PasswordService::verify_password(token_raw, &verification.token_hash)? {
             return Err(AppError::Unauthorized("Invalid token".to_string()));
        }
        
        if verification.expires_at < Utc::now().naive_utc() {
             return Err(AppError::Unauthorized("Token expired".to_string()));
        }
        
        self.user_repo.verify_user(user_id)?;
        
        self.verification_repo.mark_email_verification_as_used(verification.id)?;
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<(String, String), AppError> {
        let parts: Vec<&str> = refresh_token.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::Unauthorized("Invalid token format".to_string()));
        }

        let session_id = Uuid::parse_str(parts[0]).map_err(|_| AppError::Unauthorized("Invalid token format".to_string()))?;
        let token_raw = parts[1];

        let session = self.session_repo.find_by_id(session_id)?
            .ok_or_else(|| AppError::Unauthorized("Session not found".to_string()))?;

        if session.is_revoked {
            return Err(AppError::Unauthorized("Session revoked".to_string()));
        }

        if session.expires_at < Utc::now().naive_utc() {
            return Err(AppError::Unauthorized("Session expired".to_string()));
        }

        if !PasswordService::verify_password(token_raw, &session.refresh_token_hash)? {
            // Potential reuse/theft detection: revoke session?
             self.session_repo.revoke(session_id)?;
             return Err(AppError::Unauthorized("Invalid refresh token".to_string()));
        }

        // Generate new pair
        let new_refresh_token = self.token_service.generate_refresh_token();
        let new_hash = PasswordService::hash_password(&new_refresh_token)?;
        
        let new_expires_at = Utc::now().naive_utc() + chrono::Duration::days(self.config.jwt_refresh_expiration_days);
        
        self.session_repo.update_refresh_token(session.id, new_hash, new_expires_at)?;
        
        // Get roles for access token
        let roles = self.user_repo.get_roles(session.user_id)?;
        let access_token = self.token_service.generate_access_token(session.user_id, session.id, roles)?;
        
        // Return combined token
        let combined_refresh_token = format!("{}:{}", session.id, new_refresh_token);
        
        Ok((access_token, combined_refresh_token))
    }

    pub async fn request_email_verification(&self, email: &str) -> Result<(), AppError> {
        let user = self.user_repo.find_by_email(email)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.is_verified {
            return Err(AppError::Forbidden("Email already verified".to_string()));
        }

        // Generate new token
        let token = Uuid::new_v4().to_string();
        let token_hash = PasswordService::hash_password(&token)?;
        
        // Save to DB
        use crate::modules::auth::domain::entity::token::NewEmailVerificationToken;
        let new_token = NewEmailVerificationToken {
            user_id: user.id,
            token_hash,
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(24),
        };
        
        self.verification_repo.create_email_verification(new_token)?;
        
        use crate::modules::email::domain::service::EmailRecipient;
        // Send email
        let recipient = EmailRecipient {
            email: email.to_string(),
            name: None,
        };
        self.email_service.send_verification_email(&recipient, &format!("{}:{}", user.id, token)).await?;
        
        Ok(())
    }

    pub async fn request_password_reset(&self, email: &str) -> Result<(), AppError> {
        let user = self.user_repo.find_by_email(email)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
            
        let token = Uuid::new_v4().to_string();
        let token_hash = PasswordService::hash_password(&token)?;
        
        let reset_token = NewPasswordResetToken {
            user_id: user.id,
            token_hash,
            expires_at: Utc::now().naive_utc() + chrono::Duration::minutes(15), 
        };

        self.verification_repo.create_password_reset(reset_token)?;

        let recipient = EmailRecipient {
            email: email.to_string(),
            name: None,
        };

        self.email_service.send_password_reset_email(&recipient, &format!("{}:{}", user.id, token)).await?;

        Ok(())
    }

    pub async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), AppError> {
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::Unauthorized("Invalid token format".to_string()));
        }
        
        let user_id = Uuid::parse_str(parts[0]).map_err(|_| AppError::Unauthorized("Invalid token format".to_string()))?;
        let token_raw = parts[1];

        let reset_token = self.verification_repo.find_password_reset_by_user(user_id)?
            .ok_or(AppError::Unauthorized("Invalid or expired token".to_string()))?;

        if reset_token.used {
             return Err(AppError::ValidationError(validator::ValidationErrors::new())); 
        }

        if !PasswordService::verify_password(token_raw, &reset_token.token_hash)? {
             return Err(AppError::Unauthorized("Invalid token".to_string()));
        }

        if reset_token.expires_at < Utc::now().naive_utc() {
             return Err(AppError::ValidationError(validator::ValidationErrors::new())); 
        }

        let password_hash = PasswordService::hash_password(new_password)?;
        
        self.user_repo.update_password(user_id, &password_hash)?;
        
        self.verification_repo.mark_password_reset_as_used(reset_token.id)?;
        
        Ok(())
    }

    pub fn logout(&self, session_id: Uuid) -> Result<(), AppError> {
        self.session_repo.revoke(session_id)
    }

    pub fn revoke_all_sessions(&self, user_id: Uuid) -> Result<(), AppError> {
        self.session_repo.revoke_all_for_user(user_id)
    }

    // Helper to create session
    async fn create_session(&self, user_id: Uuid, user_agent: Option<String>, ip_address: Option<String>) -> Result<(UserSession, String), AppError> {
        let refresh_token = self.token_service.generate_refresh_token();
        let refresh_hash = PasswordService::hash_password(&refresh_token)?;

        let expires_at = Utc::now().naive_utc() + chrono::Duration::days(self.config.jwt_refresh_expiration_days);
        
        let device_name = user_agent.as_deref().map(crate::common::user_agent_parser::parse_device_name);

        let new_session = NewUserSession {
            user_id,
            refresh_token_hash: refresh_hash,
            user_agent,
            ip_address,
            device_name,
            expires_at,
        };

        let session = self.session_repo.create(new_session)?;
        
        Ok((session, refresh_token))
    }

    pub fn get_active_sessions(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError> {
        self.session_repo.find_active_by_user(user_id)
    }
}

