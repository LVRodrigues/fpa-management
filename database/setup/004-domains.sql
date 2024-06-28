--==============================================================================
-- Domains
--==============================================================================

CREATE DOMAIN id AS UUID NOT NULL;
COMMENT ON DOMAIN id IS 'Unique identifier for primary keys.';

CREATE DOMAIN brief AS VARCHAR(255) NOT NULL;
COMMENT ON DOMAIN brief IS 'Short description for the required record.';

CREATE DOMAIN description AS TEXT;
COMMENT ON DOMAIN description IS 'Long description for the record.';

CREATE DOMAIN datetime AS TIMESTAMP WITH TIME ZONE;
COMMENT ON DOMAIN datetime IS 'Date and Time with time zone.';

