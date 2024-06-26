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

CREATE TABLE tenants_status (
    status      INTEGER     NOT NULL,
    description brief
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
    description brief
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
    name    brief,
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

CREATE TABLE influences (
    influence   INTEGER     NOT NULL,
    description brief
);

COMMENT ON TABLE influences                 IS 'Influence value of the factor on the project.';
COMMENT ON COLUMN influences.influence      IS 'Unique Influence identifier.';
COMMENT ON COLUMN influences.description    IS 'Description of the Influence.';

ALTER TABLE influences ADD 
    CONSTRAINT pk_influences
    PRIMARY KEY (influence);

COMMENT ON INDEX pk_influences IS 'Primary key of the Influences.';

CREATE TABLE factors (
    factor      INTEGER     NOT NULL,
    description brief
);

COMMENT ON TABLE factors                IS 'Set of possible Factor`s Types.';
COMMENT ON COLUMN factors.factor        IS 'Factor`s Type.';
COMMENT ON COLUMN factors.description   IS 'Description for the Factor`s Type.';

ALTER TABLE factors ADD
    CONSTRAINT pk_factors
    PRIMARY KEY (factor);

COMMENT ON INDEX pk_factors IS 'Primary key for the Factors`s Types.';

CREATE TABLE empiricals (
    empirical   INTEGER     NOT NULL,
    description brief
);

COMMENT ON TABLE empiricals                 IS 'Set of possible Empirical`s Factors.';
COMMENT ON COLUMN empiricals.empirical      IS 'Empirical`s Factor.';
COMMENT ON COLUMN empiricals.description    IS 'Description for the Empirical`s factors.';

ALTER TABLE empiricals ADD
    CONSTRAINT pk_empiricals
    PRIMARY KEY (empirical);

COMMENT ON INDEX pk_empiricals IS 'Primary key for the Empirical`s Factors.';

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

CREATE TABLE projects_factors (
    project     id,
    factor      INTEGER NOT NULL,
    tenant      id,
    influence   INTEGER NOT NULL
);

COMMENT ON TABLE projects_factors               IS 'Adjusts Factors for the Project.';
COMMENT ON COLUMN projects_factors.project      IS 'Project identifier.';
COMMENT ON COLUMN projects_factors.factor       IS 'Fator`s Type for the Project.';
COMMENT ON COLUMN projects_factors.tenant       IS 'Tenant owner of the Project.';
COMMENT ON COLUMN projects_factors.influence    IS 'Influence value for the factor on this project.';

ALTER TABLE projects_factors ADD
    CONSTRAINT pk_projects_factors
    PRIMARY KEY (project, factor);

COMMENT ON INDEX pk_projects_factors IS 'Primary key for the Factor`s Types on a Project.';

ALTER TABLE projects_factors ADD
    CONSTRAINT fk_projects_factors_project
    FOREIGN KEY (project)
    REFERENCES projects (project);

CREATE INDEX ix_projects_factors_project ON projects_factors (project);

COMMENT ON INDEX ix_projects_factors_project IS 'Index to relate to Project.';

ALTER TABLE projects_factors ADD
    CONSTRAINT fk_projects_factors_factor
    FOREIGN KEY (factor)
    REFERENCES factors (factor);

CREATE INDEX ix_projects_factors_factor ON projects_factors (factor);

COMMENT ON INDEX ix_projects_factors_factor IS 'Index to relate to Fator`s Types.';

ALTER TABLE projects_factors ADD
    CONSTRAINT fk_projects_factors_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_projects_factors_tenant ON projects_factors (tenant);

COMMENT ON INDEX ix_projects_factors_tenant IS 'Index to management access on tenant scope.';

ALTER TABLE projects_factors ADD
    CONSTRAINT fk_projects_factors_influence
    FOREIGN KEY (influence)
    REFERENCES influences (influence);

CREATE INDEX ix_projects_factors_influence ON projects_factors (influence);

COMMENT ON INDEX ix_projects_factors_influence IS 'Influence value for the Factor`s Type on this Project.';

CREATE TABLE projects_empiricals (
    project     id,
    empirical   INTEGER NOT NULL,
    tenant      id,
    value       INTEGER NOT NULL
);

COMMENT ON TABLE projects_empiricals            IS 'Empirical Adjusts Factors for the Project.';
COMMENT ON COLUMN projects_empiricals.project   IS 'Project identifier.';
COMMENT ON COLUMN projects_empiricals.empirical IS 'Empirical`s Factor.';
COMMENT ON COLUMN projects_empiricals.tenant    IS 'Tenant owner of the Project.';
COMMENT ON COLUMN projects_empiricals.value     IS 'Percent of influence for the Empirical`s Factor..';

ALTER TABLE projects_empiricals ADD
    CONSTRAINT pk_projects_empiricals
    PRIMARY KEY (project, empirical);

COMMENT ON INDEX pk_projects_empiricals IS 'Primary key for the Empirical`s Factors on a Project.';

ALTER TABLE projects_empiricals ADD
    CONSTRAINT fk_projects_empiricals_project
    FOREIGN KEY (project)
    REFERENCES projects (project);

CREATE INDEX ix_projects_empiricals_project ON projects_empiricals (project);

COMMENT ON INDEX ix_projects_factors_project IS 'Index to relate to Project.';

ALTER TABLE projects_empiricals ADD
    CONSTRAINT fk_projects_empiricals_empirical
    FOREIGN KEY (empirical)
    REFERENCES empiricals (empirical);

CREATE INDEX ix_projects_empiricals_empirical ON projects_empiricals (empirical);

COMMENT ON INDEX ix_projects_factors_factor IS 'Index to relate to Empirical`s Factors.';

ALTER TABLE projects_empiricals ADD
    CONSTRAINT fk_projects_empiricals_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_projects_empiricals_tenant ON projects_empiricals (tenant);

COMMENT ON INDEX ix_projects_factors_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE modules (
    module      id,
    project     id,
    tenant      id,
    name        brief,
    description description
);

COMMENT ON TABLE modules                IS 'Module for group Functions on a Project.';
COMMENT ON COLUMN modules.module        IS 'Unique identifier for Module.';
COMMENT ON COLUMN modules.project       IS 'Project Identifier.';
COMMENT ON COLUMN modules.tenant        IS 'Tenant owner of the Module.';
COMMENT ON COLUMN modules.name          IS 'Module`s Name.';
COMMENT ON COLUMN modules.description   IS 'Module`s Description';

ALTER TABLE modules ADD 
    CONSTRAINT pk_modules
    PRIMARY KEY (module);

COMMENT ON INDEX pk_modules IS 'Primary key for Modules.';

ALTER TABLE modules ADD
    CONSTRAINT fk_modules_project
    FOREIGN KEY (project)
    REFERENCES projects (project);

CREATE INDEX ix_modules_project ON modules (project);

COMMENT ON INDEX ix_modules_project IS 'Index to relate Modules and Projects.';

ALTER TABLE modules ADD
    CONSTRAINT fk_modules_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_modules_tenant ON modules (tenant);

COMMENT ON INDEX ix_modules_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE functions_types (
    type        INTEGER  NOT NULL,
    description brief
);

COMMENT ON TABLE functions_types                IS 'Set of possible Functions`s Types.';
COMMENT ON COLUMN functions_types.type          IS 'Type of the Function.';
COMMENT ON COLUMN functions_types.description   IS 'Description for the Function`s type.';

ALTER TABLE functions_types ADD
    CONSTRAINT pk_functions_types 
    PRIMARY KEY (type);

COMMENT ON INDEX pk_functions_types IS 'Primary key for Functions`s Types.';

CREATE TABLE functions (
    function    id,
    module      id,
    tenant      id,
    type        INTEGER NOT NULL,
    name        brief,
    description description

);

COMMENT ON TABLE functions              IS 'Set of All Functions for the Module.';
COMMENT ON COLUMN functions.function    IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions.module      IS 'Identifier of the module that owns the function.';
COMMENT ON COLUMN functions.tenant      IS 'Tenant owner of the Function';
COMMENT ON COLUMN functions.type        IS 'Functions`s type.';
COMMENT ON COLUMN functions.name        IS 'Name of the Function.';
COMMENT ON COLUMN functions.description IS 'Description for the Function.';

ALTER TABLE functions ADD 
    CONSTRAINT pk_functions
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions IS 'Primary key for Functions.';

ALTER TABLE functions ADD
    CONSTRAINT fk_functions_module
    FOREIGN KEY (module)
    REFERENCES modules (module);

CREATE INDEX ix_functions_module ON functions (module);

COMMENT ON INDEX ix_functions_module IS 'Reference index to the function`s owning module.';

ALTER TABLE functions ADD
    CONSTRAINT fk_functions_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_tenant IS 'Index to management access on tenant scope.';

ALTER TABLE functions ADD
    CONSTRAINT fk_functions_type
    FOREIGN KEY (type)
    REFERENCES functions_types (type);

CREATE INDEX ix_functions_type ON functions (type);

COMMENT ON INDEX ix_functions_type IS 'Reference index to the functions`s type.';

CREATE TABLE functions_datas () INHERITS (functions);

COMMENT ON TABLE functions_datas                IS 'Set of Functions of type Data (ALI, AIE) for the Module.';
COMMENT ON COLUMN functions_datas.function      IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions_datas.module        IS 'Identifier of the module that owns the function.';
COMMENT ON COLUMN functions_datas.tenant        IS 'Tenant owner of the Function';
COMMENT ON COLUMN functions_datas.type          IS 'Functions`s type. Only for Data on type 1 and 2.';
COMMENT ON COLUMN functions_datas.name          IS 'Name of the Function.';
COMMENT ON COLUMN functions_datas.description   IS 'Description for the Function.';

ALTER TABLE functions_datas ADD 
    CONSTRAINT pk_functions_datas
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions_datas IS 'Primary key for Functions of type Data.';

ALTER TABLE functions_datas ADD
    CONSTRAINT fk_functions_datas_module
    FOREIGN KEY (module)
    REFERENCES modules (module);

CREATE INDEX ix_functions_datas_module ON functions (module);

COMMENT ON INDEX ix_functions_datas_module IS 'Reference index to the function`s owning module.';

ALTER TABLE functions_datas ADD
    CONSTRAINT fk_functions_datas_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_datas_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_datas_tenant IS 'Index to management access on tenant scope.';

ALTER TABLE functions_datas ADD
    CONSTRAINT fk_functions_datas_type
    FOREIGN KEY (type)
    REFERENCES functions_types (type);

CREATE INDEX ix_functions_datas_type ON functions (type);

COMMENT ON INDEX ix_functions_datas_type IS 'Reference index to the functions`s type.';

ALTER TABLE functions_datas ADD 
    CONSTRAINT check_functions_datas_type
    CHECK (type IN (1, 2));

CREATE TABLE functions_transactions () INHERITS (functions);

COMMENT ON TABLE functions_transactions                 IS 'Set of Functions of type Transaction (EE, CE, SE) for the Module.';
COMMENT ON COLUMN functions_transactions.function       IS 'Unique identifier for Function.';
COMMENT ON COLUMN functions_transactions.module         IS 'Identifier of the module that owns the function.';
COMMENT ON COLUMN functions_transactions.tenant         IS 'Tenant owner of the Function.';
COMMENT ON COLUMN functions_transactions.type           IS 'Functions`s type. Only for Transactions on type 3, 4 and 5.';
COMMENT ON COLUMN functions_transactions.name           IS 'Name of the Function.';
COMMENT ON COLUMN functions_transactions.description    IS 'Description for the Function.';

ALTER TABLE functions_transactions ADD 
    CONSTRAINT pk_functions_transactions
    PRIMARY KEY (function);

COMMENT ON INDEX pk_functions_transactions IS 'Primary key for Functions of type Transaction.';

ALTER TABLE functions_transactions ADD
    CONSTRAINT fk_functions_transactions_module
    FOREIGN KEY (module)
    REFERENCES modules (module);

CREATE INDEX ix_functions_transactions_module ON functions (module);

COMMENT ON INDEX ix_functions_transactions_module IS 'Reference index to the function`s owning module.';

ALTER TABLE functions_transactions ADD
    CONSTRAINT fk_functions_transactions_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_functions_transactions_tenant ON functions (tenant);

COMMENT ON INDEX ix_functions_transactions_tenant IS 'Index to management access on tenant scope.';

ALTER TABLE functions_transactions ADD
    CONSTRAINT fk_functions_transactions_type
    FOREIGN KEY (type)
    REFERENCES functions_types (type);

CREATE INDEX ix_functions_transactions_type ON functions (type);

COMMENT ON INDEX ix_functions_transactions_type IS 'Reference index to the functions`s type.';

ALTER TABLE functions_transactions ADD 
    CONSTRAINT check_functions_transactions_type
    CHECK (type IN (3, 4, 5));

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
    REFERENCES functions_transactions (function);

CREATE INDEX ix_alrs_function ON alrs (function);

COMMENT ON INDEX ix_alrs_function IS 'Reference index to the functions of type Transaction.';

ALTER TABLE alrs ADD
    CONSTRAINT fk_alrs_alr
    FOREIGN KEY (alr)
    REFERENCES functions_datas (function);

CREATE INDEX ix_alrs_alr ON alrs (alr);

COMMENT ON INDEX ix_alrs_alr IS 'Reference index to the functions of type Data.';

ALTER TABLE alrs ADD
    CONSTRAINT fk_alrs_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_alrs_tenant ON alrs (tenant);

COMMENT ON INDEX ix_alrs_tenant IS 'Index to management access on tenant scope.';

CREATE TABLE rlrs (
    rlr         id,
    function    id,
    tenant      id,
    name        brief,
    description description
);

COMMENT ON TABLE rlrs               IS 'Referenced Logical Records.';
COMMENT ON COLUMN rlrs.rlr          IS 'Unique identifier for a Reference Logical Record.';
COMMENT ON COLUMN rlrs.function     IS 'unique identifier for a Function.';
COMMENT ON COLUMN rlrs.tenant       IS 'Tenant owner of the Function.';
COMMENT ON COLUMN rlrs.name         IS 'Name of the Referenced Logical Record.';
COMMENT ON COLUMN rlrs.description  IS 'Description for the Referenced Logical Record.';

ALTER TABLE rlrs ADD
    CONSTRAINT pk_rlrs
    PRIMARY KEY (rlr);

COMMENT ON INDEX pk_rlrs IS 'Primary key for Referenced Logial Records.';

ALTER TABLE rlrs ADD 
    CONSTRAINT fk_rlrs_function
    FOREIGN KEY (function)
    REFERENCES functions (function);

CREATE INDEX ix_rlrs_function ON rlrs (function);

COMMENT ON INDEX ix_rlrs_function IS 'Reference index to the Functions.';

ALTER TABLE rlrs ADD
    CONSTRAINT fk_rlrs_tenant
    FOREIGN KEY (tenant)
    REFERENCES tenants (tenant);

CREATE INDEX ix_rlrs_tenant ON alrs (tenant);

COMMENT ON INDEX ix_rlrs_tenant IS 'Index to management access on tenant scope.';
