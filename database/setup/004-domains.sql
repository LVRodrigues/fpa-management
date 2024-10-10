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

--==============================================================================
-- Types
--==============================================================================

CREATE TYPE tenant_status AS ENUM (
    'ACTIVE',
    'SUSPENDED',
    'DISABLED');
COMMENT ON TYPE tenant_status IS 'Tenant status in the system.';

CREATE TYPE tenant_tier AS ENUM (
    'GOLD',
    'SILVER',
    'BRONZE');
COMMENT ON TYPE tenant_status IS 'Tenant access level on the system.';	

CREATE TYPE influence AS ENUM (
	'ABSENT', 
	'MINIMUM', 
	'MODERATE',
    'AVERAGE',
    'SIGNIFICANT',
    'STRONG');
COMMENT ON TYPE influence IS 'Influence value for the adjustment factor.';   
    
CREATE TYPE factor AS ENUM (
	'DATA_COMMUNICATIONS',
	'DISTRIBUTED_DATA_PROCESSING',
	'PERFORMANCE',
	'HEAVILY_USED_CONFIGURATION',
	'TRANSACTION_RATE',
	'ONLINE_DATA_ENTRY',
	'END_USER_EFFICIENCY',
	'ONLINE_UPDATE',
	'COMPLEX_PROCESSING',
	'REUSABILITY',
	'INSTALLATION_EASE',
	'OPERATIONAL_EASE',
	'FACILITTE_CHANGE',
	'MULTIPLE_SITES');
COMMENT ON TYPE factor IS 'Adjustment factor for the Project.';

CREATE TYPE empirical AS ENUM (
	'PRODUCTIVITY',
    'PLANNING',
    'COODINATION',
    'TESTING',
    'DEPLOYMENT');
COMMENT ON TYPE empirical IS 'Empirical adjustment for the Project.';

CREATE TYPE function_type AS ENUM (
	'ALI',
	'AIE',
	'CE',
	'EE',
	'SE');
COMMENT ON TYPE functions_type IS 'Type os Functions.';	