--==============================================================================
-- Creates the database and associates it with the users created by script 001.
-- This script must be run with the DBA user.
-- Should not be embedded in Docker.
--==============================================================================

CREATE TABLESPACE "fpa"
    OWNER "fpa-admin"
    LOCATION '/var/lib/postgresql/fpa';
COMMENT ON TABLESPACE "fpa"
    IS 'Data Area for the FPA Management application.';

CREATE DATABASE "fpa"
    WITH OWNER = "fpa-admin"
        ENCODING = 'UTF8'
        TABLESPACE = "fpa"
        CONNECTION LIMIT = -1;
COMMENT ON DATABASE "fpa"
    IS 'FPA Management application database.';
