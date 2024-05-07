--==============================================================================
-- Access Roles
--==============================================================================
CREATE ROLE "fpa-access";
COMMENT ON ROLE "fpa-access" IS 'Rules for operation users to access the FPA Management application.';

GRANT "fpa-access" TO "fpa-user";

--==============================================================================
-- Tables
--==============================================================================

GRANT SELECT                            ON tenants_status   TO "fpa-access";
GRANT SELECT                            ON tenants_tier     TO "fpa-access";
GRANT SELECT                            ON tenants          TO "fpa-access";
GRANT SELECT, INSERT, UPDATE            ON users            TO "fpa-access";
GRANT SELECT, INSERT                    ON versions         TO "fpa-access";
GRANT SELECT, INSERT, UPDATE, DELETE    ON projects         TO "fpa-access";

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