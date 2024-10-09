--===============================================================================
-- Initializing the database.
--===============================================================================

--===============================================================================
-- Database Version
--===============================================================================

INSERT INTO versions (version, name, major, minor, build, time)
VALUES (uuid_nil(), 'Database', 1, 0, 0, CURRENT_TIMESTAMP);

--===============================================================================
-- Tenants
--===============================================================================

INSERT INTO tenants_status (status, description) VALUES
    (1, 'Active'),
    (2, 'Suspended'),
    (3, 'Disabled');

INSERT INTO tenants_tier (tier, description) VALUES
    (1, 'Gold'),
    (2, 'Silver'),
    (3, 'Bronze');

INSERT INTO tenants (tenant, name, time, status, tier) VALUES 
    (uuid_nil(), 'Default', CURRENT_TIMESTAMP, 1, 1);

--===============================================================================
-- Users
--===============================================================================

INSERT INTO users ("user", tenant, name, email, time) VALUES
    (uuid_nil(), uuid_nil(), 'Administrator', 'lvrodriguesline@gmail.com', CURRENT_TIMESTAMP);

--===============================================================================
-- Adjusts Factors
--===============================================================================

INSERT INTO influences (influence, description) VALUES 
    (0, 'Absent'),
    (1, 'Minimum'),
    (2, 'Moderate'),
    (3, 'Average'),
    (4, 'Significant'),
    (5, 'Strong');

INSERT INTO factors (factor, description) VALUES
    ( 1, 'Data communications'),
    ( 2, 'Distributed data processing'),
    ( 3, 'Performance'),
    ( 4, 'Heavily used configuration'),
    ( 5, 'Transaction rate'),
    ( 6, 'On-Line data entry'),
    ( 7, 'End-user efficiency'),
    ( 8, 'OnLine update'),
    ( 9, 'Complex processing'),
    (10, 'Reusability'),
    (11, 'Installation ease'),
    (12, 'Operational ease'),
    (13, 'Facilitte change'),
    (14, 'Multiple sites');

INSERT INTO empiricals (empirical, description) VALUES 
    (1, 'Productivity'),
    (2, 'Planning'),
    (3, 'Coodination'),
    (4, 'Testing'),
    (5, 'Deployment');
