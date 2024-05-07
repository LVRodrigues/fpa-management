--==============================================================================
-- Creates users for the database.
-- This script must be run with the DBA user.
-- Should not be embedded in Docker.
--==============================================================================

CREATE USER "fpa-admim" WITH
	NOCREATEDB
	CREATEROLE
	ENCRYPTED PASSWORD 'pfa-pass';
COMMENT ON ROLE "fpa-admin"
	IS 'Database administrator user for the FPA Management application.';

CREATE USER "fpa-user" WITH
	NOCREATEDB
	NOCREATEROLE
	ENCRYPTED PASSWORD 'fpa-pass';
COMMENT ON ROLE "fpa-user"
	IS 'Database operation user for the FPA Management application.';
