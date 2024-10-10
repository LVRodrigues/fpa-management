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
