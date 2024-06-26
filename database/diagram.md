# Entity Relationship Diagram

```mermaid
erDiagram
    tenants_status          ||--o{ tenants: fk_tenants_status
    tenants_tier            ||--o{ tenants: fk_tenants_tier
    tenants                 ||--o{ users: fk_users_tenant
    tenants                 ||--o{ projects: fk_projects_tenant
    tenants                 ||--o{ projects_factors: fk_projects_factor_tenant
    tenants                 ||--o{ projects_empiricals: fk_projects_empiricals_tenant
    tenants                 ||--o{ modules: fk_modules_tenant
    tenants                 ||--o{ functions: fk_functions_tenant
    tenants                 ||--o{ ders: fk_ders_tenant
    tenants                 ||--o{ rlrs: fk_rlrs_tenant
    tenants                 ||--o{ alrs: fk_alrs_tenant

    users                   ||--o{ projects: fk_projects_user

    influences              ||--o{ projects_factors: fk_projects_factors_influence
    factors                 ||--o{ projects_factors: fk_projecs_factors_factor
    projects                ||--o{ projects_factors: fk_projects_factors_project

    empiricals              ||--o{ projects_empiricals: fk_projects_empiricals_empirical
    projects                ||--o{ projects_empiricals: fk_projects_empiricals_project

    projects                ||--o{ modules: fk_modules_project
    modules                 ||--o{ functions: fk_functions_module

    functions_types         ||--o{ functions: fk_functions_type
    functions               ||--|| functions_datas: inherit
    functions               ||--|| functions_transactions: inherit
    functions               ||--o{ ders: fk_ders_functions
    functions_datas         ||--o{ rlrs: fk_rlrs_function
    functions_datas         ||--o{ alrs: fk_functions_transactions_alr
    functions_transactions  ||--o{ alrs: fk_functions_transactions_function
        
    tenants_status {
        status      integer     PK
        description brief
    }

    tenants_tier {
        tier        integer     PK
        description brief
    }

    tenants {
        tenant      id          PK
        name        brief
        time        datetime
        status      integer
        tier        integer
    }

    users {
        user        id          PK
        tenant      id
        name        brief
        time        datetime
        email       brief
    }

    versions {
        version     id          PK
        name        brief
        major       integer
        minor       integer
        builde      integer
        time        datetime
    }

    influences {
        influence   integer     PK
        description brief        
    }    

    factors {
        factor      integer     PK
        description brief
    }

    empiricals {
        empirical   integer     PK
        description brief
    } 

    projects {
        project     id          PK
        tenant      id
        name        brief
        description description
        time        datetime
        user        id
        version     integer
    }

    projects_factors {
        project     id          PK
        factor      integer     PK
        tenant      id
        influence   integer
    }

    projects_empiricals {
        project     id          PK
        empirical   integer     PK
        tenant      id
        value       integer
    }

    modules {
        module      id          PK
        name        brief
        description description
        project     id
        tenant      id
    }

    functions_types {
        type        integer     PK
        description brief
    }

    functions {
        function    id          PK
        name        brief
        description description
        type        integer
        module      id
        tenant      id
    }

    functions_datas {
    }

    functions_transactions {
    }

    ders {
        der         id          PK
        name        brief
        description description
        function    id
        tenant      id
    }

    rlrs {
        rlr         id          PK
        name        brief
        description description
        function    id
        tenant      id
    }

    alrs {
        function    id          PK
        alr         id          PK
        tenant      id
    }
```
