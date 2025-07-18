
import { loadGrammar, saveGrammar, getDefaultGrammar } from '../dist/grammar/index.js';
import { LanguageGrammar } from '../dist/types/grammar.js';
import * as fs from 'fs';
import * as path from 'path';
// Polyfill __dirname for Jest compatibility
const __dirname = path.resolve();

describe('Grammar Module', () => {
    describe('loadGrammar', () => {
        test('should load default grammar when file does not exist', () => {
            const grammar = loadGrammar();
            expect(grammar).toBeDefined();
            expect(grammar.name).toBe('Cyl');
            if (grammar.keywords) {
                const keywordValues = grammar.keywords.map(k => k.value);
                expect(keywordValues).toContain('fn');
                expect(keywordValues).toContain('if');
                expect(keywordValues).toContain('else');
                expect(keywordValues).toContain('import');
                expect(keywordValues).toContain('struct');
                expect(keywordValues).toContain('return');
            }
        });
    });

    describe('getDefaultGrammar', () => {
        test('should return valid grammar structure', () => {
            const grammar = getDefaultGrammar();

            expect(grammar).toHaveProperty('name');
            expect(grammar).toHaveProperty('version');
            expect(grammar).toHaveProperty('keywords');
            expect(grammar).toHaveProperty('operators');
            expect(grammar).toHaveProperty('types');
        });

        test('should have consistent keyword structure', () => {
            const grammar = getDefaultGrammar();

            if (grammar.keywords) {
                grammar.keywords.forEach(keyword => {
                    expect(keyword).toHaveProperty('value');
                    expect(keyword).toHaveProperty('type');
                    expect(keyword).toHaveProperty('description');
                    expect(typeof keyword.value).toBe('string');
                    expect(typeof keyword.type).toBe('string');
                });
            }
        });

        test('should include all basic operators', () => {
            const grammar = getDefaultGrammar();
            if (grammar.operators) {
                const operatorSymbols = grammar.operators.map(op => op.symbol);

                expect(operatorSymbols).toContain('=');
                expect(operatorSymbols).toContain('+');
                expect(operatorSymbols).toContain('-');
            }
        });
    });

    describe('saveGrammar', () => {
        const testOutputPath = path.join(__dirname, 'test-grammar.yaml');

        afterEach(() => {
            // Clean up test file
            if (fs.existsSync(testOutputPath)) {
                fs.unlinkSync(testOutputPath);
            }
        });

        test('should save grammar to specified path', () => {
            const testGrammar: LanguageGrammar = {
                name: 'TestCyl',
                version: '0.1.0',
                keywords: [
                    { value: 'test', type: 'TestKeyword', description: 'Test keyword' }
                ],
                operators: [
                    { symbol: '=', type: 'binary', precedence: 1, associativity: 'right' }
                ],
                types: []
            };

            saveGrammar(testGrammar, testOutputPath);

            expect(fs.existsSync(testOutputPath)).toBe(true);

            const savedContent = fs.readFileSync(testOutputPath, 'utf8');
            expect(savedContent).toContain('name: TestCyl');
            expect(savedContent).toContain('test');
        });
    });
});
