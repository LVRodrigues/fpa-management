--==============================================================================
-- User access permissions to the Database created by script 002.
-- This script must be run with the DBA user.
--==============================================================================

GRANT CONNECT, TEMPORARY ON DATABASE "fpa" TO "fpa-admin";
GRANT ALL ON DATABASE "fpa" TO "fpa-admin";

GRANT CONNECT, TEMPORARY ON DATABASE "fpa" TO "fpa-user";
