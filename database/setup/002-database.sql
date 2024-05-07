--==============================================================================
-- Creates the database and associates it with the users created by script 001.
-- This script must be run with the DBA user.
-- Should not be embedded in Docker.
--==============================================================================

CREATE TABLESPACE "fpa-management"
    OWNER "fpa-admin"
    LOCATION '/var/lib/postgresql/fpa-management';
COMMENT ON TABLESPACE "fpa-managemen"
    IS 'Data Area for the FPA Management application.';

CREATE DATABASE "fpa-management"
    WITH OWNER = "fpa-admin"
        ENCODING = 'UTF8'
        TABLESPACE = "fpa-management"
        CONNECTION LIMIT = -1;
COMMENT ON DATABASE "fpa-management"
    IS 'FPA Management application database.';
