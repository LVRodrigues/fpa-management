--==============================================================================
-- Registros para auxiliar no desenvolvimento.
--==============================================================================

CREATE TABLE tests (
    test    id,
    tenant  id,
    name    description
);

ALTER TABLE tests ADD 
    CONSTRAINT pk_tentants_status
    PRIMARY KEY (test);

ALTER TABLE tests ADD
    CONSTRAINT fk_tests_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_tests_tenant ON tests (tenant);

GRANT SELECT, INSERT, UPDATE, DELETE    ON tests            TO "fpa-access";

ALTER TABLE tests ENABLE ROW LEVEL SECURITY;

CREATE POLICY tests_policy ON tests
USING (tenant = current_setting('app.current_tenant')::UUID);

INSERT INTO tenants (tenant, name, date, status, tier) 
values ('00000000-0000-0000-0000-000000000001'::UUID, 'Tenant 01', CURRENT_TIMESTAMP, 1, 1);

DO $$
BEGIN
    FOR index IN 1..100 LOOP
        INSERT INTO tests(test, tenant, name) 
        VALUES (uuid_generate_v4(), '00000000-0000-0000-0000-000000000001'::UUID, '[Tenant 01] Test ' || LPAD(CAST(index AS VARCHAR), 3, '0'));
    END LOOP;
END; $$

INSERT INTO tenants (tenant, name, date, status, tier) 
values ('00000000-0000-0000-0000-000000000002'::UUID, 'Tenant 02', CURRENT_TIMESTAMP, 1, 1);

DO $$
BEGIN
    FOR index IN 1..100 LOOP
        INSERT INTO tests(test, tenant, name) 
        VALUES (uuid_generate_v4(), '00000000-0000-0000-0000-000000000002'::UUID, '[Tenant 02] Test ' || LPAD(CAST(index AS VARCHAR), 3, '0'));
    END LOOP;
END; $$

