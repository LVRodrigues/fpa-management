const replace   = require('replace-in-file');
const moment    = require('moment-timezone');

var stamp = moment().tz('America/Sao_Paulo').format();
const options = {
    files: [
        'src/environments/environment.ts',
        'src/environments/environment.development.ts',
    ],
    from: /releaseStamp: '(.*)'/g,
    to: "releaseStamp: '" + stamp + "'",
    allowEmptyPaths: false,
};

try {
    let files = replace.sync(options);
    if (files == 0) {
        throw (
            'Tenha certeza que o arquivo "' +
            options.files +
            '" possua a propriedade "releaseStamp: "'
        );
    }
    console.log('Data de distribuição: ' + stamp);
} catch (error) {
    console.error("Erro:", error);
    throw error;
}