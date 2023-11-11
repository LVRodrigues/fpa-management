# Entity Relationship Diagram

```mermaid
erDiagram
    tenants_status ||--o{ tenants: fk_tenants_status
    tenants_tier ||--o{ tenants: fk_tenants_tier
    tenants ||--o{ users: fk_users_tenant

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
        date        datetime
        status      integer
        tier        integer
    }

    users {
        user        id
        tenant      id
        name        description
        date        datetime
        email       description
    }

    versions {
        version     id
        name        description
        major       integer
        minor       integer
        builde      integer
        date        datetime
    }
```