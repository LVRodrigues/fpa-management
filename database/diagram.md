# Entity Relationship Diagram

```mermaid
erDiagram
    tenants_status  ||--o{ tenants: fk_tenants_status
    tenants_tier    ||--o{ tenants: fk_tenants_tier
    tenants         ||--o{ users: fk_users_tenant

    tenants         ||--o{ projects: fk_projects_tenant
    tenants         ||--o{ projects_factors: fk_projects_factor_tenant
    tenants         ||--o{ projects_empiricals: fk_projects_empiricals_tenant
    tenants         ||--o{ modules: fk_modules_tenant
    tenants         ||--o{ functions: fk_functions_tenant
    tenants         ||--o{ ders: fk_ders_tenant
    tenants         ||--o{ rlrs: fk_rlrs_tenant

    users           ||--o{ projects: fk_projects_user

    influences      ||--o{ projects_factors: fk_projects_factors_influence
    factors         ||--o{ projects_factors: fk_projecs_factors_factor
    projects        ||--o{ projects_factors: fk_projects_factors_project

    empiricals      ||--o{ projects_empiricals: fk_projects_empiricals_empirical
    projects        ||--o{ projects_empiricals: fk_projects_empiricals_project

    projects        ||--o{ modules: fk_modules_project
    modules         ||--o{ functions: fk_functions_module
    functions       ||--o{ ders: fk_ders_function 
    functions       ||--o{ rlrs: fk_rlrs_function

    functions_types ||--o{ functions: fk_functions_type
        
    tenants_status {
        status      integer
        description description
    }

    tenants_tier {
        tier        int
        description description
    }

    tenants {
        tenant      id
        name        description
        time        datetime
        status      integer
        tier        integer
    }

    users {
        user        id
        tenant      id
        name        description
        time        datetime
        email       description
    }

    versions {
        version     id
        name        description
        major       integer
        minor       integer
        builde      integer
        time        datetime
    }

    influences {
        influence   int
        description description        
    }    

    factors {
        factor      int
        description description
    }

    empiricals {
        empirical   int
        description description
    } 

    projects {
        project     id
        tenant      id
        name        description
        description description_long
        time        datetime
        user        id
        version     int
    }

    projects_factors {
        project     id
        factor      int
        tenant      id
        influence   int
    }

    projects_empiricals {
        project     id
        empirical   int
        tenant      id
    }

    modules {
        module      id
        name        description
        description description_long
        project     id
        tenant      id
    }

    functions_types {
        type        int
        description description
    }

    functions {
        function    id
        name        description
        description description_long
        type        int
        module      id
        tenant      id
    }

    ders {
        der         id
        name        description
        description description_long
        function    id
        tenant      id
    }

    rlrs {
        rlr         id
        name        description
        description description_long
        function    id
        tenant      id
    }

```