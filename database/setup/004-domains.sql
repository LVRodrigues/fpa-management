--==============================================================================
-- Domains
--==============================================================================

CREATE DOMAIN id AS UUID NOT NULL;
COMMENT ON DOMAIN id IS 'Unique identifier for primary keys.';

CREATE DOMAIN description AS VARCHAR(255) NOT NULL;
COMMENT ON DOMAIN description IS 'Short description for the record.';

CREATE DOMAIN description_long AS TEXT;
COMMENT ON DOMAIN description_long IS 'Long description for the record.';

CREATE DOMAIN datetime AS TIMESTAMP WITH TIME ZONE;
COMMENT ON DOMAIN datetime IS 'Date and Time with time zone.';

