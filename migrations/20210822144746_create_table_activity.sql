CREATE TABLE activity (
    id BIGINT NOT NULL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    owner_account_id BIGINT,
    source_account_id BIGINT,
    target_account_id BIGINT,
    amount BIGINT
);
-- Table comments
COMMENT ON COLUMN activity.id IS 'Activity ID';
COMMENT ON COLUMN activity.timestamp IS 'Timestamp';
COMMENT ON COLUMN activity.owner_account_id IS 'Owner ID';
COMMENT ON COLUMN activity.source_account_id IS 'Source account ID';
COMMENT ON COLUMN activity.target_account_id IS 'Target account ID';
COMMENT ON COLUMN activity.amount IS 'Acitivy amount';