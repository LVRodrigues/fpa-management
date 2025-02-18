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

INSERT INTO tenants (tenant, name, time, status, tier) VALUES 
    (uuid_nil(), 'Default', CURRENT_TIMESTAMP, 'ACTIVE', 'GOLD');

--===============================================================================
-- Users
--===============================================================================

INSERT INTO users ("user", tenant, name, email, time) VALUES
    (uuid_nil(), uuid_nil(), 'Administrator', 'lvrodriguesline@gmail.com', CURRENT_TIMESTAMP);
