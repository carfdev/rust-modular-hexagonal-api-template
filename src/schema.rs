// @generated automatically by Diesel CLI.

diesel::table! {
    email_verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token_hash -> Varchar,
        expires_at -> Timestamp,
        used -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    password_reset_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token_hash -> Varchar,
        expires_at -> Timestamp,
        used -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        is_published -> Bool,
        author_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Int4,
        assigned_at -> Timestamp,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        refresh_token_hash -> Varchar,
        user_agent -> Nullable<Varchar>,
        ip_address -> Nullable<Varchar>,
        device_name -> Nullable<Varchar>,
        is_revoked -> Bool,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        is_active -> Bool,
        is_verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(posts -> users (author_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_verification_tokens,
    password_reset_tokens,
    posts,
    roles,
    user_roles,
    user_sessions,
    users,
);
