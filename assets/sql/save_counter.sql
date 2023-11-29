-- name: init_save_counter&
-- Initialize the counter table if it does not exist.
-- Sets the initial value to 0.
CREATE TABLE IF NOT EXISTS counter (
    id INTEGER PRIMARY KEY,
    value INTEGER
);
INSERT OR IGNORE INTO counter VALUES (1, 0);
/

-- name: get_save_count->
-- Get the current value of the counter.
SELECT value FROM counter WHERE id = 1;
/

-- name: set_save_count!
-- Sets the current value of the counter.
-- param: value: i32 - The new counter value.
UPDATE counter SET value = :value WHERE id = 1;
/
