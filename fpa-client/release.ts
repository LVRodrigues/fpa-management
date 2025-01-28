import {replaceInFileSync} from 'replace-in-file'
import moment from 'moment-timezone';

const stamp: string = moment.tz('America/Sao_Paulo').format();
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
    const files = replaceInFileSync(options);
    if (files.length === 0) {
        throw new Error(
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
