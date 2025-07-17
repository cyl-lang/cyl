
import path from 'path';
import { fileURLToPath } from 'url';
let __dirname: string;
if (typeof jest !== 'undefined' || process.env.JEST_WORKER_ID) {
    __dirname = process.cwd();
} else {
    const __filename = fileURLToPath(import.meta.url);
    __dirname = path.dirname(__filename);
}

import type { LanguageGrammar } from '../types/grammar.js';
import * as yaml from 'yaml';
import * as fs from 'fs';

export function loadGrammar(customPath?: string): LanguageGrammar {
    const grammarPath = customPath || path.join(__dirname, '../../specs/syntax.yaml');

    if (!fs.existsSync(grammarPath)) {
        // Return default grammar if file doesn't exist
        return getDefaultGrammar();
    }

    const yamlContent = fs.readFileSync(grammarPath, 'utf8');
    const grammar = yaml.parse(yamlContent) as LanguageGrammar;
    // Ensure all required properties are present as arrays
    return {
        name: grammar.name,
        version: grammar.version,
        keywords: grammar.keywords ?? [],
        operators: grammar.operators ?? [],
        syntaxRules: grammar.syntaxRules ?? [],
        types: grammar.types ?? []
    };
}

export function saveGrammar(grammar: LanguageGrammar, filePath?: string): void {
    const outputPath = filePath || path.join(__dirname, '../../specs/syntax.yaml');
    // Ensure all properties are present for serialization
    const grammarToSave: LanguageGrammar = {
        name: grammar.name,
        version: grammar.version,
        keywords: grammar.keywords ?? [],
        operators: grammar.operators ?? [],
        syntaxRules: grammar.syntaxRules ?? [],
        types: grammar.types ?? []
    };
    const yamlContent = yaml.stringify(grammarToSave, { indent: 2 });
    fs.writeFileSync(outputPath, yamlContent, 'utf8');
}

export function getDefaultGrammar(): LanguageGrammar {
    return {
        name: 'Cyl',
        version: '0.1.0',
        keywords: [
            { value: 'fn', type: 'FunctionDeclaration', description: 'Function declaration' },
            { value: 'if', type: 'IfStatement', description: 'Conditional statement' },
            { value: 'else', type: 'ElseStatement', description: 'Alternative branch' },
            { value: 'import', type: 'ImportStatement', description: 'Module import' },
            { value: 'return', type: 'ReturnStatement', description: 'Return from function' },
            { value: 'struct', type: 'StructDeclaration', description: 'Structure type definition' },
            { value: 'enum', type: 'EnumDeclaration', description: 'Enumeration type definition' },
            { value: 'match', type: 'MatchStatement', description: 'Pattern matching' },
            { value: 'for', type: 'ForStatement', description: 'Loop statement' },
            { value: 'while', type: 'WhileStatement', description: 'While loop' },
            { value: 'break', type: 'BreakStatement', description: 'Break from loop' },
            { value: 'continue', type: 'ContinueStatement', description: 'Continue loop' },
            { value: 'try', type: 'TryStatement', description: 'Exception handling' },
            { value: 'catch', type: 'CatchStatement', description: 'Exception catching' },
            { value: 'throw', type: 'ThrowStatement', description: 'Throw exception' },
            { value: 'async', type: 'AsyncFunctionDeclaration', description: 'Async function' },
            { value: 'await', type: 'AwaitExpression', description: 'Await async operation' },
            { value: 'void', type: 'VoidType', description: 'Void type' },
            { value: 'declare', type: 'DeclareStatement', description: 'Variable declaration' },
            { value: 'let', type: 'DeclareStatement', description: 'Variable declaration' },
            { value: 'const', type: 'DeclareStatement', description: 'Constant declaration' },
            { value: 'mut', type: 'MutabilityModifier', description: 'Mutable modifier' },
        ],
        operators: [
            // Arithmetic
            { symbol: '+', type: 'binary', precedence: 6, associativity: 'left', description: 'Addition' },
            { symbol: '-', type: 'binary', precedence: 6, associativity: 'left', description: 'Subtraction' },
            { symbol: '*', type: 'binary', precedence: 7, associativity: 'left', description: 'Multiplication' },
            { symbol: '/', type: 'binary', precedence: 7, associativity: 'left', description: 'Division' },
            { symbol: '%', type: 'binary', precedence: 7, associativity: 'left', description: 'Modulo' },

            // Comparison
            { symbol: '==', type: 'binary', precedence: 4, associativity: 'left', description: 'Equality' },
            { symbol: '!=', type: 'binary', precedence: 4, associativity: 'left', description: 'Inequality' },
            { symbol: '<', type: 'binary', precedence: 5, associativity: 'left', description: 'Less than' },
            { symbol: '<=', type: 'binary', precedence: 5, associativity: 'left', description: 'Less than or equal' },
            { symbol: '>', type: 'binary', precedence: 5, associativity: 'left', description: 'Greater than' },
            { symbol: '>=', type: 'binary', precedence: 5, associativity: 'left', description: 'Greater than or equal' },

            // Logical
            { symbol: '&&', type: 'binary', precedence: 3, associativity: 'left', description: 'Logical AND' },
            { symbol: '||', type: 'binary', precedence: 2, associativity: 'left', description: 'Logical OR' },
            { symbol: '!', type: 'unary', precedence: 8, associativity: 'right', description: 'Logical NOT' },

            // Bitwise
            { symbol: '&', type: 'binary', precedence: 4, associativity: 'left', description: 'Bitwise AND' },
            { symbol: '|', type: 'binary', precedence: 2, associativity: 'left', description: 'Bitwise OR' },
            { symbol: '^', type: 'binary', precedence: 3, associativity: 'left', description: 'Bitwise XOR' },
            { symbol: '<<', type: 'binary', precedence: 6, associativity: 'left', description: 'Left shift' },
            { symbol: '>>', type: 'binary', precedence: 6, associativity: 'left', description: 'Right shift' },
            { symbol: '~', type: 'unary', precedence: 8, associativity: 'right', description: 'Bitwise NOT' },

            // Assignment
            { symbol: '=', type: 'binary', precedence: 1, associativity: 'right', description: 'Assignment' },

            // Other
            { symbol: '->', type: 'binary', precedence: 9, associativity: 'left', description: 'Return type indicator' },
            { symbol: '.', type: 'binary', precedence: 10, associativity: 'left', description: 'Member access' },
        ],
        syntaxRules: [
            {
                name: 'FunctionDeclaration',
                pattern: '[async] fn <identifier> ( [parameters] ) [-> <type>] { <statements> }',
                description: 'Function declaration syntax',
                examples: [
                    'fn main() -> void {}',
                    'async fn fetchData() -> string {}',
                    'fn add(a: int, b: int) -> int { return a + b; }'
                ]
            },
            {
                name: 'VariableDeclaration',
                pattern: '[let|const] [mut] <identifier> [: <type>] = <expression>',
                description: 'Variable declaration syntax',
                examples: [
                    'let x = 5;',
                    'const mut name: string = "John";',
                    'let result: int = calculate();'
                ]
            },
            {
                name: 'IfStatement',
                pattern: 'if <expression> { <statements> } [else <statement>]',
                description: 'Conditional statement syntax',
                examples: [
                    'if x > 0 { print("positive"); }',
                    'if condition { doSomething(); } else { doOther(); }'
                ]
            },
            {
                name: 'Import',
                pattern: 'import <module> [{ <items> }]',
                description: 'Module import syntax',
                examples: [
                    'import net;',
                    'import os { print, exit };'
                ]
            }
        ],
        types: [
            { name: 'int', kind: 'primitive', description: 'Signed integer type' },
            { name: 'float', kind: 'primitive', description: 'Floating point number type' },
            { name: 'string', kind: 'primitive', description: 'UTF-8 string type' },
            { name: 'bool', kind: 'primitive', description: 'Boolean type' },
            { name: 'char', kind: 'primitive', description: 'Unicode character type' },
            { name: 'void', kind: 'primitive', description: 'No value type' },
            { name: 'Array<T>', kind: 'generic', description: 'Dynamic array type' },
            { name: 'Option<T>', kind: 'generic', description: 'Optional value type' },
        ],
        // Always include all properties as arrays
        // (if you add new properties to LanguageGrammar, add them here as empty arrays if not present)
    };
}

export function validateGrammar(grammar: LanguageGrammar): boolean {
    // Basic validation
    if (!grammar.name || !grammar.version) {
        return false;
    }

    if (!grammar.keywords || grammar.keywords.length === 0) {
        return false;
    }

    if (!grammar.operators || grammar.operators.length === 0) {
        return false;
    }

    return true;
}