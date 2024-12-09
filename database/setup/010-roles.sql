--==============================================================================
-- Access Roles
--==============================================================================
CREATE ROLE "fpa-access";
COMMENT ON ROLE "fpa-access" IS 'Rules for operation users to access the FPA Management application.';

GRANT "fpa-access" TO "fpa-user";

--==============================================================================
-- Tables
--==============================================================================

GRANT SELECT                            ON tenants                  TO "fpa-access";
GRANT SELECT, INSERT, UPDATE            ON users                    TO "fpa-access";
GRANT SELECT, INSERT                    ON versions                 TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON projects                 TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON empiricals               TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON factors                  TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON modules                  TO "fpa-access";
GRANT SELECT                            ON functions                TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON functions_datas          TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON functions_transactions   TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON alrs                     TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON rlrs                     TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON ders                     TO "fpa-access";

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

ALTER TABLE empiricals ENABLE ROW LEVEL SECURITY;
CREATE POLICY empiricals_policy ON empiricals
USING (tenant = current_setting('app.current_tenant')::UUID);

ALTER TABLE factors ENABLE ROW LEVEL SECURITY;
CREATE POLICY factors_policy ON factors
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

ALTER TABLE rlrs ENABLE ROW LEVEL SECURITY;
CREATE POLICY rlrs_policy ON rlrs
USING (tenant = current_setting('/app.current_tenant')::UUID);

ALTER TABLE ders ENABLE ROW LEVEL SECURITY;
CREATE POLICY ders_policy ON ders
USING (tenant = current_setting('/app.current_tenant')::UUID);