CREATE TABLE IF NOT EXISTS auth_tokens (
    id UUID PRIMARY KEY,
    entity_type SMALLINT NOT NULL DEFAULT 0,
    entity_id UUID NOT NULL,
    value VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL,
    valid_due BIGINT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS auth_tokens_value_uidx ON auth_tokens (value);
CREATE INDEX IF NOT EXISTS auth_tokens_entity_idx
    ON auth_tokens (entity_type, entity_id);

COMMENT ON COLUMN auth_tokens.entity_type IS '0 - user';
