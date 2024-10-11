--==============================================================================
-- Registros para auxiliar no desenvolvimento.
--==============================================================================

INSERT INTO tenants (tenant, name, time, status, tier)
VALUES	('00000000-0000-0000-0000-000000000001', 'Tenant 01', CURRENT_TIMESTAMP, 'ACTIVE', 'GOLD'),
		('00000000-0000-0000-0000-000000000002', 'Tenant 02', CURRENT_TIMESTAMP, 'ACTIVE', 'SILVER'),
		('00000000-0000-0000-0000-000000000003', 'Tenant 03', CURRENT_TIMESTAMP, 'ACTIVE', 'BRONZE');

CREATE OR REPLACE FUNCTION update_db() RETURNS VOID AS $$
DECLARE
	t RECORD;
	i INTEGER;
	project uuid;
	factor factor;
BEGIN
	FOR t IN SELECT * FROM tenants LOOP
		FOR i IN 1..100 LOOP
			project := uuid_generate_v4();
			RAISE NOTICE 'Tenant: % - Project: %', t.tenant, project;
			INSERT INTO projects (project, tenant, "user", name, description, time)
			VALUES (project, t.tenant, uuid_nil(), 'Project ' || TO_CHAR(i, 'fm000'), 'Long description for the Project ' || TO_CHAR(i, 'fm000'), CURRENT_TIMESTAMP);

			INSERT INTO empiricals (project, empirical, tenant, value) VALUES 
				(project, 'PRODUCTIVITY', t.tenant, 14),
				(project, 'PLANNING', t.tenant, 15),
				(project, 'COODINATION', t.tenant, 20),
				(project, 'TESTING', t.tenant, 20),
				(project, 'DEPLOYMENT', t.tenant, 10);

			FOR factor IN SELECT unnest(enum_range(NULL::factor)) LOOP
				INSERT INTO factors (project, factor, tenant, influence)
				VALUES (project, factor, t.tenant, 'ABSENT');
			END LOOP;
		END LOOP;
	END LOOP;
END;
$$ LANGUAGE 'plpgsql';
SELECT update_db();
DROP FUNCTION update_db();