--==============================================================================
-- Creates users for the database.
-- This script must be run with the DBA user.
-- It is built into Docker, replacing the 001-users.sql file.
--==============================================================================

CREATE USER "fpa-user" WITH
	NOCREATEDB
	NOCREATEROLE
	ENCRYPTED PASSWORD 'fpa-pass';
COMMENT ON ROLE "fpa-user"
	IS 'Database operation user for the FPA Management application.';