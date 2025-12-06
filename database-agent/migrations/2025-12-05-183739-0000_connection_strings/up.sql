CREATE TABLE connection_strings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    value VARCHAR NOT NULL,
    description VARCHAR,
    source VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    created_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER connection_strings_updated_at 
AFTER UPDATE on connection_strings
FOR EACH ROW
BEGIN
    UPDATE connection_strings SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- CREATE TRIGGER connection_strings_pending
-- AFTER INSERT on connection_strings
-- FOR EACH ROW
-- BEGIN
--     UPDATE connection_strings SET status = 'pending';
-- END;