import { CylLanguageDesign } from './index.js';
import chalk from 'chalk';

if (import.meta.url === `file://${process.argv[1]}` || import.meta.url === process.argv[1]) {
    (async () => {
        const design = await CylLanguageDesign.create();
        const args = process.argv.slice(2);
        const command = args[0];
        switch (command) {
            case 'validate':
                design.validateGrammar();
                break;
            case 'generate':
                design.generateAST();
                break;
            case 'info':
                design.displayLanguageInfo();
                design.displayKeywords();
                design.displayOperators();
                design.displaySyntaxRules();
                break;
            case 'check':
                if (args[1]) {
                    const fs = require('fs');
                    if (fs.existsSync(args[1])) {
                        const code = fs.readFileSync(args[1], 'utf8');
                        design.checkSyntax(code);
                    } else {
                        console.error(chalk.red(`File not found: ${args[1]}`));
                        if (typeof process.env.JEST_WORKER_ID === 'undefined') {
                            process.exit(1);
                        }
                    }
                } else {
                    console.error(chalk.red('Please provide a file to check'));
                    if (typeof process.env.JEST_WORKER_ID === 'undefined') {
                        process.exit(1);
                    }
                }
                break;
            case 'full': {
                const success = design.runFullCheck();
                if (typeof process.env.JEST_WORKER_ID === 'undefined') {
                    process.exit(success ? 0 : 1);
                }
                break;
            }
            default:
                console.log(chalk.cyan('Cyl Language Design Tool'));
                console.log('========================');
                console.log('Commands:');
                console.log('  validate  - Validate the grammar');
                console.log('  generate  - Generate AST definitions');
                console.log('  info      - Display language information');
                console.log('  check <file> - Check syntax of a file');
                console.log('  full      - Run complete validation and generation');
                break;
        }
    })();
}
