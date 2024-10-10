--==============================================================================
-- Registros para auxiliar no desenvolvimento.
--==============================================================================

INSERT INTO tenants (tenant, name, time, status, tier)
VALUES	('00000000-0000-0000-0000-000000000001', 'Tenant 01', CURRENT_TIMESTAMP, 'ACTIVE', 'GOLD'),
		('00000000-0000-0000-0000-000000000002', 'Tenant 02', CURRENT_TIMESTAMP, 'ACTIVE', 'SILVER'),
		('00000000-0000-0000-0000-000000000003', 'Tenant 03', CURRENT_TIMESTAMP, 'ACTIVE', 'BRONZE');

CREATE OR REPLACE FUNCTION update_db() RETURNS VOID AS $$
DECLARE
	i INTEGER;
BEGIN
	FOR i IN 1..100 LOOP
		INSERT INTO projects (project, tenant, "user", name, description, time)
		VALUES (uuid_generate_v4(), uuid_nil(), uuid_nil(), 'Project ' || TO_CHAR(i, 'fm000'), 'Long description for the Project ' || TO_CHAR(i, 'fm000'), CURRENT_TIMESTAMP);
	END LOOP;
END;
$$ LANGUAGE 'plpgsql';
SELECT update_db();
DROP FUNCTION update_db();