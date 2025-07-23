-- Create table for tracking AwesomeEvent events from KaracurtTest contract
CREATE TABLE awesome_events (
    -- Event identification
    transaction_version BIGINT NOT NULL,
    event_index BIGINT NOT NULL,
    
    -- Event data from the AwesomeEvent struct
    user_address VARCHAR(66) NOT NULL,
    random_number BIGINT NOT NULL,
    timestamp_microseconds BIGINT NOT NULL,
    message TEXT NOT NULL,
    call_count BIGINT NOT NULL,
    
    -- Blockchain metadata
    transaction_block_height BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    
    -- Indexer metadata
    indexed_at TIMESTAMP NOT NULL DEFAULT NOW(),
    
    -- Primary key
    PRIMARY KEY (transaction_version, event_index)
);

-- Create indexes for common queries
CREATE INDEX idx_awesome_events_user_address ON awesome_events(user_address);
CREATE INDEX idx_awesome_events_timestamp ON awesome_events(timestamp_microseconds);
CREATE INDEX idx_awesome_events_call_count ON awesome_events(call_count);
CREATE INDEX idx_awesome_events_block_height ON awesome_events(transaction_block_height);
