CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role SMALLINT NOT NULL,
    status SMALLINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    deleted_at BIGINT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_uidx ON users (email);

COMMENT ON COLUMN users.role IS '0 - trader, 1 - admin';
COMMENT ON COLUMN users.status IS '0 - active, 1 - blocked, 2 - deleted';
