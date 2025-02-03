import packageJson from '../../package.json';

export const environment = {
    version: packageJson.version + '-dev',
    production: false,
    release: '2025-01-28T08:47:33-03:00',
};
