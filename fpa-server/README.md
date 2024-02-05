# FPA Server

Application Server for **FPA Management** system.

## Docker

The **Dockerfile** file configures a server for use in the **FPA Management** system.

To create the Docker image for the project, run:

```bash
docker build -f Dockerfile -t fpa-management/fpa-server .
```

To run the image with a volatile server:

```bash
docker run --rm -p 5000:5000/tcp fpa-management/fpa-server
```

To run the image with a persistent server:

```bash
docker run --name fpa-management-server -p 5000:5000 fpa-management/fpa-server
docker start fpa-management-server
docker stop fpa-management-server
```

## Access the Database

Two users are created:

1. **fpa-admin**: user with DBA rights.
2. **fpa-user**: user for system operation.

In both cases, the password is **fpa-pass**.

The database is named **fpa-management** and is accessible via the address **localhost**, on port **5432**.

As the user **fpa-admin** is the owner of the objects, he has full access to the database.

The user **fpa-user** must provide the unique identifier of the tenant that owns the data on each connection. This isolates the information and does not allow data from one client to be viewed by another.

```sql
SET app.current_tenant = '00000000-0000-0000-0000-000000000000';
```

> Even during development, it is recommended to use the user **fpa-user**.
