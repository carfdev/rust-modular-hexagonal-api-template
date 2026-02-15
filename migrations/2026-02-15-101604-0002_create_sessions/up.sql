CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash VARCHAR NOT NULL,
    user_agent VARCHAR,
    ip_address VARCHAR,
    device_name VARCHAR,
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
