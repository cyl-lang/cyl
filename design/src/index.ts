import type { LanguageGrammar } from './types/grammar.js';
import { loadGrammar, saveGrammar, getDefaultGrammar } from './grammar/index.js';
import { GrammarValidator, ValidationResult } from './tools/grammar-validator.js';
import { ASTGenerator } from './tools/ast-generator.js';
import { SyntaxChecker, SyntaxCheckResult } from './tools/syntax-checker.js';
import chalk from 'chalk';
import { loadPythonPlugins } from './tools/python-plugin-loader.js';

export class CylLanguageDesign {
    private grammar: LanguageGrammar;
    private validator: GrammarValidator;
    private astGenerator: ASTGenerator;
    private syntaxChecker: SyntaxChecker;

    private constructor(grammar: LanguageGrammar) {
        this.grammar = grammar;
        this.validator = new GrammarValidator(this.grammar);
        this.astGenerator = new ASTGenerator(this.grammar);
        this.syntaxChecker = new SyntaxChecker(this.grammar);
    }

    // Static async factory for proper initialization
    static async create(grammarPath?: string): Promise<CylLanguageDesign> {
        let grammar = grammarPath ? CylLanguageDesign.prototype.loadCustomGrammar(grammarPath) : loadGrammar();
        // Load Python plugins and merge their info asynchronously
        const pluginInfo = await loadPythonPlugins();
        if (pluginInfo) {
            if (pluginInfo.syntax && Array.isArray(grammar.keywords)) {
                grammar.keywords.push(...pluginInfo.syntax.map((s: string) => ({ value: s, type: 'PluginSyntax', description: 'From Python plugin' })));
            }
            if (pluginInfo.types && Array.isArray(grammar.types)) {
                grammar.types = grammar.types.concat(pluginInfo.types.map((t: string) => ({ name: t, description: 'From Python plugin', kind: 'generic' })));
            }
        }
        return new CylLanguageDesign(grammar);
    }

    // ...existing code...
// ...existing code...

    private loadCustomGrammar(path: string): LanguageGrammar {
        try {
            return loadGrammar();
        } catch (error) {
            console.warn(chalk.yellow(`Failed to load grammar from ${path}, using default grammar`));
            if (error instanceof Error) {
                console.error(chalk.red(`Error details: ${error.message}`));
            } else {
                console.error(chalk.red('Unknown error occurred while loading grammar.'));
            }
            return getDefaultGrammar();
        }
    }

    public validateGrammar(): ValidationResult {
        console.log(chalk.blue('🔍 Validating grammar...'));
        const result = this.validator.validate();

        if (result.isValid) {
            console.log(chalk.green('✅ Grammar validation passed!'));
        } else {
            console.log(chalk.red('❌ Grammar validation failed!'));
            result.errors.forEach(error => {
                console.log(chalk.red(`   Error: ${error.message}`));
            });
        }

        if (result.warnings.length > 0) {
            result.warnings.forEach(warning => {
                console.log(chalk.yellow(`   Warning: ${warning.message}`));
            });
        }

        return result;
    }

    public generateAST(): void {
        console.log(chalk.blue('🏗️  Generating AST definitions...'));

        try {
            const rustAST = this.astGenerator.generateRustAST();
            this.astGenerator.saveToFile(rustAST, 'generated_ast.rs', './compiler/src');

            const tsAST = this.astGenerator.generateTypeScriptDefinitions();
            this.astGenerator.saveToFile(tsAST, 'generated_ast.ts', './design/src/generated');

            console.log(chalk.green('✅ AST generation completed!'));
        } catch (error) {
            console.error(chalk.red('❌ AST generation failed:'), error);
        }
    }

    public checkSyntax(code: string): SyntaxCheckResult {
        console.log(chalk.blue('🔍 Checking syntax...'));
        const result = this.syntaxChecker.checkSyntax(code);

        if (result.isValid) {
            console.log(chalk.green('✅ Syntax is valid!'));
        } else {
            console.log(chalk.red('❌ Syntax errors found!'));
            result.issues.forEach(issue => {
                const icon = issue.severity === 'error' ? '❌' : '⚠️';
                console.log(chalk.gray(`   ${icon} Line ${issue.line}:${issue.column} - ${issue.message}`));
            });
        }

        return result;
    }

    public displayLanguageInfo(): void {
        console.log(chalk.cyan.bold('\n🔧 Cyl Language Design Tool'));
        console.log(chalk.cyan('================================\n'));

        console.log(chalk.white.bold('Language Information:'));
        console.log(`  Name: ${chalk.green(this.grammar.name)}`);
        console.log(`  Version: ${chalk.green(this.grammar.version)}`);
        console.log(`  Keywords: ${chalk.yellow(this.grammar.keywords?.length ?? 0)}`);
        console.log(`  Operators: ${chalk.yellow(this.grammar.operators?.length ?? 0)}`);
        console.log(`  Syntax Rules: ${chalk.yellow(this.grammar.syntaxRules?.length ?? 0)}`);
        console.log(`  Types: ${chalk.yellow(this.grammar.types?.length ?? 0)}\n`);
    }

    public displayKeywords(): void {
        console.log(chalk.white.bold('Keywords:'));
        if (this.grammar.keywords) {
            this.grammar.keywords.forEach(keyword => {
                console.log(`  ${chalk.green(keyword.value.padEnd(12))} - ${chalk.gray(keyword.description ?? 'No description')}`);
            });
        }
        console.log();
    }

    public displayOperators(): void {
        console.log(chalk.white.bold('Operators:'));
        if (this.grammar.operators) {
            // Sort by precedence
            const sortedOps = [...this.grammar.operators].sort((a, b) => b.precedence - a.precedence);

            sortedOps.forEach(op => {
                const precStr = `(${op.precedence})`;
                const typeStr = `[${op.type}]`;
                const assocStr = `{${op.associativity}}`;
                console.log(`  ${chalk.green(op.symbol.padEnd(4))} ${chalk.blue(precStr.padEnd(4))} ${chalk.magenta(typeStr.padEnd(8))} ${chalk.cyan(assocStr.padEnd(8))} - ${chalk.gray(op.description ?? 'No description')}`);
            });
        }
        console.log();
    }

    public displaySyntaxRules(): void {
        console.log(chalk.white.bold('Syntax Rules:'));
        if (this.grammar.syntaxRules) {
            this.grammar.syntaxRules.forEach(rule => {
                console.log(`  ${chalk.green(rule.name)}:`);
                console.log(`    Pattern: ${chalk.yellow(rule.pattern)}`);
                if (rule.examples && rule.examples.length > 0) {
                    console.log(`    Examples:`);
                    rule.examples.forEach(example => {
                        console.log(`      ${chalk.gray(example)}`);
                    });
                }
                console.log();
            });
        }
    }

    public getGrammar(): LanguageGrammar {
        return this.grammar;
    }

    public updateGrammar(newGrammar: LanguageGrammar): void {
        this.grammar = newGrammar;
        this.validator = new GrammarValidator(this.grammar);
        this.astGenerator = new ASTGenerator(this.grammar);
        this.syntaxChecker = new SyntaxChecker(this.grammar);
    }

    public saveGrammar(filePath?: string): void {
        try {
            saveGrammar(this.grammar, filePath);
            console.log(chalk.green(`✅ Grammar saved to ${filePath ?? 'default location'}`));
        } catch (error) {
            console.error(chalk.red('❌ Failed to save grammar:'), error);
        }
    }

    public runFullCheck(): boolean {
        this.displayLanguageInfo();

        const validationResult = this.validateGrammar();
        if (!validationResult.isValid) {
            return false;
        }

        this.generateAST();
        return true;
    }
}

// Export everything for external use
export * from './types/grammar.js';
export * from './ast/nodes.js';
export * from './grammar/index.js';
export * from './tools/grammar-validator.js';
export * from './tools/ast-generator.js';
export * from './tools/syntax-checker.js';

