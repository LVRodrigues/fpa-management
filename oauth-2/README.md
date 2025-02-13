# OAuth 2

Authorizing access to **FPA** application services. 
Uses Keycloak to manage users and their access profiles.

## Docker

The **Dockerfile** file configures the keycloak for use in the 
**FPA** system.

To create the Docker image for the project, run:

```bash
docker build -f Dockerfile -t fpa/oauth-2 .
```

To run the image with a volatile database:

```bash
docker run --rm -p 8080:8080/tcp -p 8443:8443/tcp fpa/oauth-2
```

To run the image with a persistent database:

```bash
docker run --name fpa-oauth-2 -p 8080:8080 -p 8443:8443 fpa/oauth-2
docker start fpa-oauth-2
docker stop fpa-oauth-2
```

## Access the administration console

* http://localhost:8080/
* https://localhost:8443/

Users are created:

| User          | Password   | Role                       | Realm                         |
| ------------- | ---------- | -------------------------- | ----------------------------- |
| admin         | admin      | Keycloak Administrator.    | Master                        |
| system        | fpa-pass   | system                     | default, tenant-01, tenant-02 |
| admin         | fpa-pass   | administrator              | default, tenant-01, tenant-02 |
| user          | fpa-pass   | user                       | default, tenant-01, tenant-02 |

## Settings

To reconfigure the authorization application, use the user **admin**. 
After carrying out the settings, you must make a new backup copy and 
export the file to rebuild the container.

To perform the backup, run:

```bash
docker exec -it fpa-oauth-2 /bin/bash
```
Run the command to export the domain data:

```bash
cp -rp /opt/keycloak/data/h2 /tmp

/opt/keycloak/bin/kc.sh export \
  --dir /opt/keycloak/data/export \
  --users realm_file \
  --db dev-file \
  --db-url 'jdbc:h2:file:/tmp/h2/keycloakdb;NON_KEYWORDS=VALUE'
```

After the backup, the **fpa-management-realm.json** file must be extracted.

```bash
docker cp fpa-oauth-2:/opt/keycloak/data/export/default-realm.json   .
docker cp fpa-oauth-2:/opt/keycloak/data/export/tenant-01-realm.json .
docker cp fpa-oauth-2:/opt/keycloak/data/export/tenant-02-realm.json .
```

Rerun the container rebuild to persist the changes.

## Realms

There are 3 realms in development. To access them, the customer identifier is required.

| Realm     | Client ID       | Client Secret                    |
| --------- | --------------- | -------------------------------- |
| default   | fpa-server      | ogIzFgW9nY8kbptdREn5cw2rrn0Cihpv |
| tenant-01 | fpa-server      | jKQO0Pxb1gFrSz64iUgqlgsoANs86d31 |
| tenant-02 | fpa-server      | mUyu1Jd9VKIWCxrHkl00NauuAxzO7KCP |

A aplicação cliente tem a API pública no Keycloak.

| Realm     | Client ID       |
| --------- | --------------- | 
| default   | fpa-lcient      | 
| tenant-01 | fpa-client      | 
| tenant-02 | fpa-client      | 

## Authentication

Start the container and execute:

```bash
curl -X POST \
  'http://localhost:8080/realms/default/protocol/openid-connect/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'grant_type=password' \
  --data-urlencode 'client_id=fpa-server' \
  --data-urlencode 'client_secret=ogIzFgW9nY8kbptdREn5cw2rrn0Cihpv' \
  --data-urlencode 'username=user' \
  --data-urlencode 'password=fpa-pass' 
```

### Decompose de Token

Using de tools **jq**, **tr** and **jwt-cli**.

```bash
curl -X POST \
  'http://localhost:8080/realms/default/protocol/openid-connect/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'grant_type=password' \
  --data-urlencode 'client_id=fpa-server' \
  --data-urlencode 'client_secret=ogIzFgW9nY8kbptdREn5cw2rrn0Cihpv' \
  --data-urlencode 'username=user' \
  --data-urlencode 'password=fpa-pass' \
  | jq ".access_token" \
  | tr -d '"' \
  | jwt decode -
```