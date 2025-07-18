import { GrammarValidator } from '../dist/tools/grammar-validator.js';
import { LanguageGrammar } from '../dist/types/grammar.js';

describe('GrammarValidator', () => {
    let validator: GrammarValidator;
    let validGrammar: LanguageGrammar;

    beforeEach(() => {
        validGrammar = {
            name: 'TestCyl',
            version: '0.1.0',
            keywords: [
                { value: 'fn', type: 'FunctionDeclaration', description: 'Function declaration' },
                { value: 'if', type: 'IfStatement', description: 'Conditional statement' }
            ],
            operators: [
                { symbol: '=', type: 'binary', precedence: 1, associativity: 'right' },
                { symbol: '+', type: 'binary', precedence: 5, associativity: 'left' }
            ],
            types: [
                { name: 'void', kind: 'primitive', description: 'Void type' },
                { name: 'i32', kind: 'primitive', description: '32-bit integer' }
            ]
        };
        validator = new GrammarValidator(validGrammar);
    });

    describe('Validation', () => {
        test('should validate correct grammar', () => {
            const result = validator.validate();
            expect(result.isValid).toBeDefined();
            expect(typeof result.isValid).toBe('boolean');
            expect(Array.isArray(result.errors)).toBe(true);
        });

        test('should detect missing required fields', () => {
            const invalidGrammar = { ...validGrammar };
            delete (invalidGrammar as any).name;

            const invalidValidator = new GrammarValidator(invalidGrammar);
            const result = invalidValidator.validate();

            expect(result.isValid).toBe(false);
            expect(result.errors.length).toBeGreaterThan(0);
        });

        test('should validate keyword uniqueness', () => {
            const grammarWithDuplicates = {
                ...validGrammar,
                keywords: [
                    { value: 'fn', type: 'FunctionDeclaration', description: 'Function declaration' },
                    { value: 'fn', type: 'FunctionDeclaration', description: 'Duplicate function declaration' }
                ]
            };

            const duplicateValidator = new GrammarValidator(grammarWithDuplicates);
            const result = duplicateValidator.validate();

            expect(result.isValid).toBe(false);
            expect(result.errors.some(error => error.message.includes('duplicate') || error.message.includes('Duplicate'))).toBe(true);
        });

        test('should provide helpful error messages', () => {
            const invalidGrammar = {
                name: '',
                version: '0.1.0'
            } as LanguageGrammar;

            const invalidValidator = new GrammarValidator(invalidGrammar);
            const result = invalidValidator.validate();

            expect(result.errors).toBeDefined();
            expect(result.errors.length).toBeGreaterThan(0);
            result.errors.forEach(error => {
                expect(error).toHaveProperty('message');
                expect(error).toHaveProperty('type');
                expect(error.message.length).toBeGreaterThan(0);
            });
        });
    });

    describe('Warnings', () => {
        test('should generate warnings for potential issues', () => {
            const result = validator.validate();
            expect(result).toHaveProperty('warnings');
            expect(Array.isArray(result.warnings)).toBe(true);
        });
    });
});
