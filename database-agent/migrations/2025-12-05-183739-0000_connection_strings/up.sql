CREATE TABLE connection_strings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    value VARCHAR NOT NULL,
    description VARCHAR,
    source VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending',
    created_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER connection_strings_updated_at 
AFTER UPDATE on connection_strings
FOR EACH ROW
BEGIN
    UPDATE connection_strings SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- Automatically delete any existing 'pending' records before inserting a new one
CREATE TRIGGER connection_strings_pending
BEFORE INSERT on connection_strings
FOR EACH ROW
WHEN NEW.status = 'pending'
BEGIN
    DELETE FROM connection_strings WHERE status = 'pending';
END;

-- Create an event in the events table when new connection strings are added
CREATE TRIGGER connection_strings_event_created
AFTER INSERT ON connection_strings
FOR EACH ROW
BEGIN
    INSERT INTO events (event_type, aggregate_type, aggregate_id, payload)
    VALUES (
        'connection_string.created',
        'connection_string',
        CAST(NEW.id AS TEXT),
        json_object('id', NEW.id, 'value', NEW.value, 'status', NEW.status, 'source', NEW.source)
    );
END;

-- Unique Index to ensure no duplicates
CREATE UNIQUE INDEX idx_connection_strings ON connection_strings(value);