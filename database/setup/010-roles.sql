--==============================================================================
-- Access Roles
--==============================================================================
CREATE ROLE "fpa-access";
COMMENT ON ROLE "fpa-access" IS 'Rules for operation users to access the FPA Management application.';

GRANT "fpa-access" TO "fpa-user";

--==============================================================================
-- Tables
--==============================================================================

GRANT SELECT                            ON tenants_status           TO "fpa-access";
GRANT SELECT                            ON tenants_tier             TO "fpa-access";
GRANT SELECT                            ON tenants                  TO "fpa-access";
GRANT SELECT, INSERT, UPDATE            ON users                    TO "fpa-access";
GRANT SELECT, INSERT                    ON versions                 TO "fpa-access";
GRANT SELECT                            ON empiricals               TO "fpa-access";
GRANT SELECT                            ON influences               TO "fpa-access";
GRANT SELECT                            ON factors                  TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON projects                 TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON projects_empiricals      TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON projects_factors         TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON modules                  TO "fpa-access";
GRANT SELECT                            ON functions                TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON functions_datas          TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON functions_transactions   TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON alrs                     TO "fpa-access";

--==============================================================================
-- Policies (Multi-Tenant)
--==============================================================================

ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenants_policy ON tenants
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE users ENABLE ROW LEVEL SECURITY;
CREATE POLICY users_policy ON users
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
CREATE POLICY projects_policy ON projects
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE projects_empiricals ENABLE ROW LEVEL SECURITY;
CREATE POLICY projects_empiricals_policy ON projects_empiricals
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE projects_factors ENABLE ROW LEVEL SECURITY;
CREATE POLICY projects_factors_policy ON projects_factors
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE modules ENABLE ROW LEVEL SECURITY;
CREATE POLICY modules_policy ON modules
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE functions ENABLE ROW LEVEL SECURITY;
CREATE POLICY functions_policy ON functions
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE functions_datas ENABLE ROW LEVEL SECURITY;
CREATE POLICY functions_datas_policy ON functions_datas
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE functions_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY functions_transactions_policy ON functions_transactions
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE alrs ENABLE ROW LEVEL SECURITY;
CREATE POLICY alrs_policy ON alrs
USING (tenant = current_setting('/app.current_tenant')::UUID);