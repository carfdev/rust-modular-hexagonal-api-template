CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Seed basic roles
INSERT INTO roles (name, description) VALUES 
('user', 'Standard user'),
('admin', 'Administrator');
