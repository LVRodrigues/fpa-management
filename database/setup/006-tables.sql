--==============================================================================
-- Tables
--==============================================================================

--==============================================================================
-- Controle de Versões.
--==============================================================================

CREATE TABLE versions (
    version id,
    name    brief,
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

CREATE TABLE tenants (
    tenant  id,
    name    brief,
    time    datetime        NOT NULL,
    status  tenant_status   NOT NULL,
    tier    tenant_tier     NOT NULL
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

CREATE INDEX ix_tenants_status ON tenants (status);

COMMENT ON INDEX ix_tenants_status IS 'Index to select the status of tenants.';

CREATE INDEX ix_tenants_tier ON tenants (tier);

COMMENT ON INDEX ix_tenants_tier IS 'Index to select the tier of tenants.';

CREATE UNIQUE INDEX uq_tenants_tenant_name ON tenants (tenant, name);

COMMENT ON INDEX uq_tenants_tenant_name IS 'Exclusive name of Project in a Tenant.';

CREATE TABLE users (
    "user"      id,
    tenant      id,
    name        brief,
    email       brief,
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

COMMENT ON INDEX pk_users IS 'Primary key of the Users.';

ALTER TABLE users ADD
    CONSTRAINT fk_users_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_users_tenant ON users (tenant);

COMMENT ON INDEX ix_users_tenant IS 'Index to management access on tenant scope.';

--==============================================================================
-- Projetos de Análise por Pontos de Função (Multi Tenant)
--==============================================================================

CREATE TABLE projects (
    project     id,
    tenant      id,
    name        brief,
    description description,
    time        datetime        NOT NULL,
    "user"      id
);

COMMENT ON TABLE projects               IS 'Project information.';
COMMENT ON COLUMN projects.project      IS 'Unique Project identificatier.';
COMMENT ON COLUMN projects.tenant       IS 'Tenant owner of the Project.';
COMMENT ON COLUMN projects.name         IS 'Name of the Project.';
COMMENT ON COLUMN projects.description  IS 'Description of the Project.';
COMMENT ON COLUMN projects.time         IS 'Project registration time.';
COMMENT ON COLUMN projects.user         IS 'User responsible for the Project.';

ALTER TABLE projects ADD
    CONSTRAINT pk_projects
    PRIMARY KEY (project);

COMMENT ON INDEX pk_projects IS 'Primary key for the Project.';

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

CREATE TABLE empiricals (
    project     id,
    empirical   empirical_type NOT NULL,
    tenant      id,
    value       INTEGER NOT NULL
);

COMMENT ON TABLE empiricals            IS 'Empirical Adjusts Factors for the Project.';
COMMENT ON COLUMN empiricals.project   IS 'Project identifier.';
COMMENT ON COLUMN empiricals.empirical IS 'Empirical`s Factor.';
COMMENT ON COLUMN empiricals.tenant    IS 'Tenant owner of the Project.';
COMMENT ON COLUMN empiricals.value     IS 'Percent of influence for the Empirical`s Factor.';

ALTER TABLE empiricals ADD
    CONSTRAINT pk_empiricals
    PRIMARY KEY (project, empirical);

COMMENT ON INDEX pk_empiricals IS 'Primary key for the Empirical`s Factors on a Project.';

ALTER TABLE empiricals ADD
    CONSTRAINT fk_empiricals_project
    FOREIGN KEY (project)
    REFERENCES projects (project)
    ON DELETE CASCADE;

CREATE INDEX ix_empiricals_project ON empiricals (project);

COMMENT ON INDEX ix_empiricals_project IS 'Index to relate to Project.';

CREATE INDEX ix_empiricals_empirical ON empiricals (empirical);

COMMENT ON INDEX ix_empiricals_empirical IS 'Index to relate to Empirical`s Factors.';

ALTER TABLE empiricals ADD
    CONSTRAINT fk_empiricals_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_empiricals_tenant ON empiricals (tenant);

COMMENT ON INDEX ix_empiricals_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE frontiers (
    frontier    id,
    project     id,
    tenant      id,
    name        brief,
    description description
);

COMMENT ON TABLE frontiers                IS 'Frontier for group Functions on a Project.';
COMMENT ON COLUMN frontiers.frontier      IS 'Unique identifier for Frontier.';
COMMENT ON COLUMN frontiers.project       IS 'Project Identifier.';
COMMENT ON COLUMN frontiers.tenant        IS 'Tenant owner of the Frontier.';
COMMENT ON COLUMN frontiers.name          IS 'Frontier`s Name.';
COMMENT ON COLUMN frontiers.description   IS 'Frontier`s Description';

ALTER TABLE frontiers ADD 
    CONSTRAINT pk_frontiers
    PRIMARY KEY (frontier);

COMMENT ON INDEX pk_frontiers IS 'Primary key for Frontiers.';

ALTER TABLE frontiers ADD
    CONSTRAINT fk_frontiers_project
    FOREIGN KEY (project)
    REFERENCES projects (project);

CREATE INDEX ix_frontiers_project ON frontiers (project);

COMMENT ON INDEX ix_frontiers_project IS 'Index to relate Frontiers and Projects.';

ALTER TABLE frontiers ADD
    CONSTRAINT fk_frontiers_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_frontiers_tenant ON frontiers (tenant);

COMMENT ON INDEX ix_frontiers_tenant IS 'Index to management access on tenant scope.';

CREATE UNIQUE INDEX uq_frontiers_project_name ON frontiers (project, name);

COMMENT ON INDEX uq_frontiers_project_name IS 'Unique index for exclusive frontiers`s name on a Project.';

CREATE TABLE factors (
    frontier    id,
    factor      factor_type NOT NULL,
    tenant      id,
    influence   influence_type NOT NULL
);

COMMENT ON TABLE factors            IS 'Adjustments Factors for the Frontire.';
COMMENT ON COLUMN factors.factor    IS 'Adjustment Fator for the Frontier.';
COMMENT ON COLUMN factors.tenant    IS 'Tenant owner of the Frontier.';
COMMENT ON COLUMN factors.frontier  IS 'Frontier identifier.';
COMMENT ON COLUMN factors.influence	IS 'Influence value for the factor on this Frontier.';

ALTER TABLE factors ADD
    CONSTRAINT pk_factors
    PRIMARY KEY (frontier, factor);

COMMENT ON INDEX pk_factors IS 'Primary key for the Factor`s Types on a Frontier.';

ALTER TABLE factors ADD
    CONSTRAINT fk_factors_frontier
    FOREIGN KEY (frontier)
    REFERENCES frontiers (frontier)
    ON DELETE CASCADE;

CREATE INDEX ix_factors_frontier ON factors (frontier);

COMMENT ON INDEX ix_factors_frontier IS 'Index to relate to Frontier.';

CREATE INDEX ix_factors_factor ON factors (factor);

COMMENT ON INDEX ix_factors_factor IS 'Index to relate to Fator`s Types.';

ALTER TABLE factors ADD
    CONSTRAINT fk_factors_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_factors_tenant ON factors (tenant);

COMMENT ON INDEX ix_factors_tenant IS 'Index to management access on tenant scope.';

CREATE INDEX ix_factors_influence ON factors (influence);

COMMENT ON INDEX ix_factors_influence IS 'Influence value for the Factor`s Type on this Project.';

CREATE TABLE functions (
    function    id,
    frontier    id,
    tenant      id,
    type        function_type NOT NULL,
    name        brief,
    description description

);

COMMENT ON TABLE functions              IS 'Set of All Functions for the Frontier.';
COMMENT ON COLUMN functions.function    IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions.frontier    IS 'Identifier of the Frontier that owns the Function.';
COMMENT ON COLUMN functions.tenant      IS 'Tenant owner of the Function';
COMMENT ON COLUMN functions.type        IS 'Functions`s type.';
COMMENT ON COLUMN functions.name        IS 'Name of the Function.';
COMMENT ON COLUMN functions.description IS 'Description for the Function.';

ALTER TABLE functions ADD 
    CONSTRAINT pk_functions
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions IS 'Primary key for Functions.';

ALTER TABLE functions ADD
    CONSTRAINT fk_functions_frontier
    FOREIGN KEY (frontier)
    REFERENCES frontiers (frontier);

CREATE INDEX ix_functions_frontier ON functions (frontier);

COMMENT ON INDEX ix_functions_frontier IS 'Reference index to the function`s owning frontier.';

ALTER TABLE functions ADD
    CONSTRAINT fk_functions_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_tenant IS 'Index to management access on tenant scope.';

CREATE INDEX ix_functions_type ON functions (type);

COMMENT ON INDEX ix_functions_type IS 'Reference index to the functions`s type.';

CREATE TABLE functions_datas () INHERITS (functions);

COMMENT ON TABLE functions_datas                IS 'Set of Functions of type Data (ALI, AIE) for the Frontier.';
COMMENT ON COLUMN functions_datas.function      IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions_datas.frontier      IS 'Identifier of the Frontier that owns the Function.';
COMMENT ON COLUMN functions_datas.tenant        IS 'Tenant owner of the Function';
COMMENT ON COLUMN functions_datas.type          IS 'Functions`s type. Only for Data on type 1 and 2.';
COMMENT ON COLUMN functions_datas.name          IS 'Name of the Function.';
COMMENT ON COLUMN functions_datas.description   IS 'Description for the Function.';

ALTER TABLE functions_datas ADD 
    CONSTRAINT pk_functions_datas
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions_datas IS 'Primary key for Functions of type Data.';

ALTER TABLE functions_datas ADD
    CONSTRAINT fk_functions_datas_frontier
    FOREIGN KEY (frontier)
    REFERENCES frontiers (frontier);

CREATE INDEX ix_functions_datas_frontier ON functions (frontier);

COMMENT ON INDEX ix_functions_datas_frontier IS 'Reference index to the Functions`s owning Frontier.';

ALTER TABLE functions_datas ADD
    CONSTRAINT fk_functions_datas_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_datas_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_datas_tenant IS 'Index to management access on tenant scope.';

CREATE INDEX ix_functions_datas_type ON functions (type);

COMMENT ON INDEX ix_functions_datas_type IS 'Reference index to the functions`s type.';

ALTER TABLE functions_datas ADD 
    CONSTRAINT check_functions_datas_type
    CHECK (type IN ('ALI', 'AIE'));

CREATE TABLE functions_transactions () INHERITS (functions);

COMMENT ON TABLE functions_transactions                 IS 'Set of Functions of type Transaction (EE, CE, SE) for the Frontier.';
COMMENT ON COLUMN functions_transactions.function       IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions_transactions.frontier       IS 'Identifier of the Frontier that owns the Function.';
COMMENT ON COLUMN functions_transactions.tenant         IS 'Tenant owner of the Function.';
COMMENT ON COLUMN functions_transactions.type           IS 'Functions`s type. Only for Transactions on type 3, 4 and 5.';
COMMENT ON COLUMN functions_transactions.name           IS 'Name of the Function.';
COMMENT ON COLUMN functions_transactions.description    IS 'Description for the Function.';

ALTER TABLE functions_transactions ADD 
    CONSTRAINT pk_functions_transactions
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions_transactions IS 'Primary key for Functions of type Transaction.';

ALTER TABLE functions_transactions ADD
    CONSTRAINT fk_functions_transactions_frontier
    FOREIGN KEY (frontier)
    REFERENCES frontiers (frontier);

CREATE INDEX ix_functions_transactions_frontier ON functions (frontier);

COMMENT ON INDEX ix_functions_transactions_frontier IS 'Reference index to the Function`s owning Frontier.';

ALTER TABLE functions_transactions ADD
    CONSTRAINT fk_functions_transactions_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_transactions_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_transactions_tenant IS 'Index to management access on tenant scope.';

CREATE INDEX ix_functions_transactions_type ON functions (type);

COMMENT ON INDEX ix_functions_transactions_type IS 'Reference index to the functions`s type.';

ALTER TABLE functions_transactions ADD 
    CONSTRAINT check_functions_transactions_type
    CHECK (type IN ('CE', 'EE', 'SE'));

CREATE TABLE alrs (
    function    id,
    alr         id,
    tenant      id
);

COMMENT ON TABLE alrs           IS 'Referenced Logical Files.';
COMMENT ON COLUMN alrs.function IS 'Unique identifier for a Function of type Transaction..';
COMMENT ON COLUMN alrs.alr      IS 'Unique identifier for a Function if type Data.';
COMMENT ON COLUMN alrs.tenant   IS 'Tenant owner of the Function.';

ALTER TABLE alrs ADD
    CONSTRAINT pk_alrs
    PRIMARY KEY (function, alr);

COMMENT ON INDEX pk_alrs IS 'Index for association between Transaction and Data functions.';

ALTER TABLE alrs ADD 
    CONSTRAINT fk_alrs_function
    FOREIGN KEY (function)
    REFERENCES functions_transactions (function)
    ON DELETE CASCADE;

CREATE INDEX ix_alrs_function ON alrs (function);

COMMENT ON INDEX ix_alrs_function IS 'Reference index to the functions of type Transaction.';

ALTER TABLE alrs ADD
    CONSTRAINT fk_alrs_alr
    FOREIGN KEY (alr)
    REFERENCES functions_datas (function)
    ON DELETE CASCADE;

CREATE INDEX ix_alrs_alr ON alrs (alr);

COMMENT ON INDEX ix_alrs_alr IS 'Reference index to the functions of type Data.';

ALTER TABLE alrs ADD
    CONSTRAINT fk_alrs_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_alrs_tenant ON alrs (tenant);

COMMENT ON INDEX ix_alrs_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE rlrs (
    function    id,
    name        brief,
    description description,
    tenant      id
);

COMMENT ON TABLE rlrs               IS 'Referenced Logical Records.';
COMMENT ON COLUMN rlrs.function     IS 'Unique identifier for a Function.';
COMMENT ON COLUMN rlrs.name         IS 'Name of the Referenced Logical Record.';
COMMENT ON COLUMN rlrs.tenant       IS 'Tenant owner of the Function.';
COMMENT ON COLUMN rlrs.description  IS 'Description for the Referenced Logical Record.';

ALTER TABLE rlrs ADD
    CONSTRAINT pk_rlrs
    PRIMARY KEY (function, name);

COMMENT ON INDEX pk_rlrs IS 'Primary key for Referenced Logial Records.';

ALTER TABLE rlrs ADD 
    CONSTRAINT fk_rlrs_functions_datas
    FOREIGN KEY (function)
    REFERENCES functions_datas (function)
    ON DELETE CASCADE;

CREATE INDEX ix_rlrs_function ON rlrs (function);

COMMENT ON INDEX ix_rlrs_function IS 'Reference index to the Functions.';

ALTER TABLE rlrs ADD
    CONSTRAINT fk_rlrs_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_rlrs_tenant ON alrs (tenant);

COMMENT ON INDEX ix_rlrs_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE ders (
    function    id,
    rlr         brief,
    name        brief,
    description description,
    tenant      id
);

COMMENT ON TABLE ders               IS 'Referenced Elementary Data.';
COMMENT ON COLUMN ders.function     IS 'Unique identifier for a Referenced Elementary Data.';
COMMENT ON COLUMN ders.rlr          IS 'Identifier for a Referenced Logical Record.';
COMMENT ON COLUMN ders.name         IS 'Name of the Referenced Elementary Data.';
COMMENT ON COLUMN ders.description  IS 'Description for the Referenced Elementary Data.';
COMMENT ON COLUMN ders.tenant       IS 'Tenant owner of the Referenced Elementary Data.';

ALTER TABLE ders ADD
    CONSTRAINT pk_ders
    PRIMARY KEY (function, rlr, name);

COMMENT ON INDEX pk_ders IS 'Primary key for Referenced Rlementary Data.';

ALTER TABLE ders ADD
    CONSTRAINT fk_ders_rlrs
    FOREIGN KEY (function, rlr)
    REFERENCES rlrs (function, name)
    ON DELETE CASCADE;

CREATE INDEX ix_ders_rlr ON ders (function, rlr);

COMMENT ON INDEX ix_ders_rlr IS 'Reference index to the Referenced Logical Records.';

ALTER TABLE ders ADD
    CONSTRAINT fk_ders_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_ders_tenant ON alrs (tenant);
COMMENT ON INDEX ix_ders_tenant IS 'Index to management access on tenant scope.';