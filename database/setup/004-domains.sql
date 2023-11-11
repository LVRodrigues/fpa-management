--==============================================================================
-- Domains
--==============================================================================

CREATE DOMAIN id AS uuid NOT NULL;
COMMENT ON DOMAIN id IS 'Unique identifier for primary keys.';

CREATE DOMAIN description AS VARCHAR(255);
COMMENT ON DOMAIN description IS 'Record description.';

CREATE DOMAIN datetime AS time with time zone;
COMMENT ON DOMAIN datetime IS 'Date and Time with time zone.';

CREATE DOMAIN image AS text;
COMMENT ON DOMAIN image IS 'Field for storing images.';