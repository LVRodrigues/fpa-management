--==============================================================================
-- Tables
--==============================================================================

--==============================================================================
-- Controle de Versões.
--==============================================================================

CREATE TABLE versions (
    version id,
    name    description NOT NULL,
    major   INTEGER     NOT NULL DEFAULT 0,
    minor   INTEGER     NOT NULL DEFAULT 0,
    build   INTEGER     NOT NULL DEFAULT 0,
    time    datetime    NOT NULL
);

COMMENT ON TABLE versions           IS 'Application update records.';
COMMENT ON COLUMN versions.version  IS 'Unique identifier of the Version record.';
COMMENT ON COLUMN versions.name     IS 'Updated module name.';
COMMENT ON COLUMN versions.major    IS 'Major Version identification number.';
COMMENT ON COLUMN versions.minor    IS 'Minor Version identification number.';
COMMENT ON COLUMN versions.build    IS 'Build Version identification.';
COMMENT ON COLUMN versions.time     IS 'Version record time.';

ALTER TABLE versions ADD 
    CONSTRAINT pk_versions
    PRIMARY KEY (version);

COMMENT ON INDEX pk_versions IS 'Primary key of the Version registry.';

CREATE UNIQUE INDEX uq_versions ON versions (name, major, minor, build);

COMMENT ON INDEX uq_versions IS 'Unique index to register a Version, consisting of the name and identifiers.';

--==============================================================================
-- Controle de Inquilinos (Multi Tenant)
--==============================================================================

CREATE TABLE tenants_status (
    status      INTEGER     NOT NULL,
    description description NOT NULL
);

COMMENT ON TABLE tenants_status                 IS 'Tenant status in the system.';
COMMENT ON COLUMN tenants_status.status         IS 'Tenant status identifier.';
COMMENT ON COLUMN tenants_status.description    IS 'Description of the Tenant status.';

ALTER TABLE tenants_status ADD  
    CONSTRAINT pk_tentants_status
    PRIMARY KEY (status);

COMMENT ON INDEX pk_tentants_status IS 'Primary key of the Tenant status.';

CREATE TABLE tenants_tier (
    tier        INTEGER     NOT NULL,
    description description NOT NULL
);

COMMENT ON TABLE tenants_tier               IS 'Tenant access level on the system.';
COMMENT ON COLUMN tenants_tier.tier         IS 'Tenant access level identifier.';
COMMENT ON COLUMN tenants_tier.description  IS 'Description of the Tenant access level.';

ALTER TABLE tenants_tier ADD
    CONSTRAINT pk_tenants_tier
    PRIMARY KEY (tier);

COMMENT ON INDEX pk_tenants_tier IS 'Primary key of the Tenant access level.';

CREATE TABLE tenants (
    tenant  id,
    name    description NOT NULL,
    time    datetime    NOT NULL,
    status  INTEGER     NOT NULL,
    tier    INTEGER     NOT NULL
);

COMMENT ON TABLE tenants            IS 'Tenant of the system.';
COMMENT ON COLUMN tenants.tenant    IS 'Unique Tenant identifier.';
COMMENT ON COLUMN tenants.name      IS 'Tenant identification name.';
COMMENT ON COLUMN tenants.time      IS 'Tenant registration time.';
COMMENT ON COLUMN tenants.status    IS 'Tenant status.';
COMMENT ON COLUMN tenants.tier      IS 'Tenant access level.';

ALTER TABLE tenants ADD
    CONSTRAINT pk_tenants
    PRIMARY KEY (tenant);

COMMENT ON INDEX pk_tenants IS 'Primary key of the Tenant.';

ALTER TABLE tenants ADD
    CONSTRAINT fk_tenant_status
    FOREIGN KEY (status)
    REFERENCES tenants_status (status);

CREATE INDEX ix_tenants_status ON tenants (status);

COMMENT ON INDEX ix_tenants_status IS 'Index to select the status of tenants.';

ALTER TABLE tenants ADD
    CONSTRAINT fk_tenants_tier
    FOREIGN KEY (tier)
    REFERENCES tenants_tier (tier);

CREATE INDEX ix_tenants_tier ON tenants (tier);

COMMENT ON INDEX ix_tenants_tier IS 'Index to select the tier of tenants.';

CREATE UNIQUE INDEX uq_tenants_tenant_name ON tenants (tenant, name);

COMMENT ON INDEX uq_tenants_tenant_name IS 'Exclusive name of Project in a Tenant.';

CREATE TABLE users (
    "user"      id,
    tenant      id,
    name        description NOT NULL,
    email       description NOT NULL,
    time        datetime    NOT NULL
);

COMMENT ON TABLE users          IS 'User of the system.';
COMMENT ON COLUMN users."user"  IS 'Unique User identifier.';
COMMENT ON COLUMN users.tenant  IS 'Tenant owner of the User.';
COMMENT ON COLUMN users.name    IS 'Name of the User.';
COMMENT ON COLUMN users.email   IS 'E-Mail of the User.';
COMMENT ON COLUMN users.time    IS 'User registration time.';

ALTER TABLE users ADD
    CONSTRAINT pk_users
    PRIMARY KEY ("user");

COMMENT ON index pk_users IS 'Primary key of the Users.';

ALTER TABLE users ADD
    CONSTRAINT fk_users_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_users_tenant ON users (tenant);

COMMENT ON INDEX ix_users_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE projects (
    project     id,
    tenant      id,
    name        description,
    time        datetime,
    "user"      id
);

COMMENT ON TABLE projects           IS 'Project information.';
COMMENT ON COLUMN projects.project  IS 'Unique Project identificatier.';
COMMENT ON COLUMN projects.tenant   IS 'Tenant owner of the Project.';
COMMENT ON COLUMN projects.name     IS 'Name of the Project.';
COMMENT ON COLUMN projects.time     IS 'Project registration time.';
COMMENT ON COLUMN projects.user     IS 'User responsible for the Project.';

ALTER TABLE projects ADD
    CONSTRAINT pk_projects
    PRIMARY KEY (project);

ALTER TABLE projects ADD
    CONSTRAINT fk_projects_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_projects_tenant ON projects (tenant);

COMMENT ON INDEX ix_projects_tenant IS 'Index to management access on tenant scope.';

ALTER TABLE projects ADD
    CONSTRAINT fk_projects_user
    FOREIGN KEY ("user")
    REFERENCES users ("user");

CREATE INDEX ix_projects_user ON projects ("user");

COMMENT ON INDEX ix_projects_user IS 'Reference index for Users.';

CREATE UNIQUE INDEX uq_projects_tenant_name ON projects(tenant, name);

COMMENT ON INDEX uq_projects_tenant_name IS 'Unique Project Name on a Tenant.';
