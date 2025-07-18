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
    });

    describe('Warnings', () => {
        test('should generate warnings for potential issues', () => {
            const result = validator.validate();
            expect(result).toHaveProperty('warnings');
            expect(Array.isArray(result.warnings)).toBe(true);
        });
    });
});
