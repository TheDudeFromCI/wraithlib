-- name: init_server_list!
-- Initialize the server list table if it does not exist.
CREATE TABLE IF NOT EXISTS server_list (
    uuid BLOB,
    name TEXT,
    address TEXT
)
/

-- name: get_server_entries?
-- Get all entries in the server list.
SELECT rowid, name, address FROM server_list
/

-- name: add_server_entry!
-- Add an entry to the server list.
-- param: name: &str - The name of the server.
-- param: address: &str - The address of the server.
INSERT OR REPLACE INTO server_list VALUES (ZEROBLOB(16), :name, :address)
/
