# PostgreSQL

PostgreSQL database for the **FPA Management** application.

[![Static Badge](https://img.shields.io/badge/Entity_Relationship_Diagram-blue)](./diagram.md)

## Docker

The **Dockerfile** file configures a database for use in the **FPA** system.

To create the Docker image for the project, run:

```bash
docker build -f Dockerfile -t fpa/database .
```

To run the image with a volatile database:

```bash
docker run --rm -p 5432:5432/tcp fpa/database
```

To run the image with a persistent database:

```bash
docker run --name fpa-database -p 5432:5432 fpa/database
docker start fpa-database
docker stop fpa-database
```

## Access the Database

Two users are created:

1. **fpa-admin**: user with DBA rights.
2. **fpa-user**: user for system operation.

In both cases, the password is **fpa-pass**.

The database is named **fpa** and is accessible via the address **localhost**, on port **5432**.

As the user **fpa-admin** is the owner of the objects, he has full access to the database.

The user **fpa-user** must provide the unique identifier of the tenant that owns the data on each connection. This isolates the information and does not allow data from one client to be viewed by another.

```sql
SET app.current_tenant = '00000000-0000-0000-0000-000000000000';
```

> Even during development, it is recommended to use the user **fpa-user**.
