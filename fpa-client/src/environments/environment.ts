import packageJson from '../../package.json';

export const environment = {
    version: packageJson.version,
    production: false,
    release: '2025-01-28T08:47:33-03:00',

    keycloak: {
        url: 'http://localhost:8080',
        realm: 'default',
        clientId: 'fpa-client',
        sessionTimeout: 300000
    }
};
