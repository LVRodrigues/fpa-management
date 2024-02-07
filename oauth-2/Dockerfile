FROM quay.io/keycloak/keycloak:22.0.5

ENV KEYCLOAK_ADMIN=admin
ENV KEYCLOAK_ADMIN_PASSWORD=admin

COPY \
    default-realm.json \
    tenant-01-realm.json \
    tenant-02-realm.json \
    /opt/keycloak/data/import/

EXPOSE 8080
EXPOSE 8443

ENTRYPOINT [ \
    "/opt/keycloak/bin/kc.sh", \
    "start-dev", \
    "--import-realm"]
