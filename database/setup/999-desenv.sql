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
	factor factor_type;
	frontier uuid;
	fun_ali uuid;
	fun_aie uuid;
	fun_ce uuid;
	fun_se uuid;
	fun_ee uuid;
BEGIN
	FOR t IN SELECT * FROM tenants LOOP
		FOR i IN 1..100 LOOP
			project := uuid_generate_v4();
			RAISE NOTICE 'Tenant: % - Project: %', t.tenant, project;
			INSERT INTO projects (project, tenant, "user", name, description, time)
			VALUES (project, t.tenant, uuid_nil(), 'Project ' || TO_CHAR(i, 'fm000'), 'Long description for the Project ' || TO_CHAR(i, 'fm000'), CURRENT_TIMESTAMP);

			frontier := uuid_generate_v4();
			INSERT INTO frontiers (frontier, project, tenant, name, description) 
			VALUES (frontier, project, t.tenant, 'Test', 'Frontier for test');

			FOR factor IN SELECT unnest(enum_range(NULL::factor_type)) LOOP
				INSERT INTO factors (frontier, factor, tenant, influence)
				VALUES (frontier, factor, t.tenant, 'ABSENT');
			END LOOP;

			INSERT INTO empiricals (frontier, empirical, tenant, value) VALUES 
				(frontier, 'PRODUCTIVITY', t.tenant, 14),
				(frontier, 'PLANNING', t.tenant, 15),
				(frontier, 'COORDINATION', t.tenant, 20),
				(frontier, 'TESTING', t.tenant, 20),
				(frontier, 'DEPLOYMENT', t.tenant, 10);

			fun_ali := uuid_generate_v4();
			INSERT INTO functions_datas (function, frontier, tenant, name, description, type)
			VALUES (fun_ali, frontier, t.tenant, 'Function ALI', 'Function ALI for test', 'ALI');
			INSERT INTO rlrs (function, name, description, tenant) 
			VALUES (fun_ali, 'Local Users', null, t.tenant);
			INSERT INTO ders (function, rlr, tenant, name, description) VALUES 
				(fun_ali, 'Local Users', t.tenant, 'id', null),
				(fun_ali, 'Local Users', t.tenant, 'name', null),
				(fun_ali, 'Local Users', t.tenant, 'email', null),
				(fun_ali, 'Local Users', t.tenant, 'cpf', null),
				(fun_ali, 'Local Users', t.tenant, 'phone', null);

			fun_aie = uuid_generate_v4();
			INSERT INTO functions_datas (function, frontier, tenant, name, description, type)
			VALUES (fun_aie, frontier, t.tenant, 'Function AIE', 'Function AIE for test', 'AIE');
			INSERT INTO rlrs (function, name, description, tenant)
			VALUES (fun_aie, 'LDAP Users', null, t.tenant);
			INSERT INTO ders (function, rlr, tenant, name, description) VALUES 
				(fun_aie, 'LDAP Users', t.tenant, 'id', null),
				(fun_aie, 'LDAP Users', t.tenant, 'name', null),
				(fun_aie, 'LDAP Users', t.tenant, 'email', null);

			fun_ee = uuid_generate_v4();
			INSERT INTO functions_transactions (function, frontier, tenant, name, description, type)
			VALUES (fun_ee, frontier, t.tenant, 'Function EE', 'Function EE for test', 'EE');
			INSERT INTO alrs (function, tenant, alr) VALUES 
				(fun_ee, t.tenant, fun_ali);

			fun_ce = uuid_generate_v4();
			INSERT INTO functions_transactions (function, frontier, tenant, name, description, type)
			VALUES (fun_ce, frontier, t.tenant, 'Function CE', 'Function CE for test', 'CE');
			INSERT INTO alrs (function, tenant, alr) VALUES 
				(fun_ce, t.tenant, fun_aie);

			fun_se = uuid_generate_v4();
			INSERT INTO functions_transactions (function, frontier, tenant, name, description, type)
			VALUES (fun_se, frontier, t.tenant, 'Function SE', 'Function SE for test', 'SE');
			INSERT INTO alrs (function, tenant, alr) VALUES 
				(fun_se, t.tenant, fun_ali),
				(fun_se, t.tenant, fun_aie);
		END LOOP;
	END LOOP;
END;
$$ LANGUAGE 'plpgsql';
SELECT update_db();
DROP FUNCTION update_db();