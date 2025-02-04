import packageJson from '../../package.json';

export const environment = {
    version: packageJson.version + '-dev',
    production: false,
    release: '2025-01-28T08:47:33-03:00',

    loginUrl: 'http://localhost:8080/realms/default/protocol/openid-connect/token',
    oauthClientId: 'fpa-management',
    oauthSecret: 'ogIzFgW9nY8kbptdREn5cw2rrn0Cihpv'
};
