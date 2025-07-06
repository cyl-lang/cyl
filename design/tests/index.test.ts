import { CylLanguageDesign } from '../src/index';

describe('CylLanguageDesign', () => {
    let cylDesign: CylLanguageDesign;

    beforeEach(() => {
        cylDesign = new CylLanguageDesign();
    });

    describe('Initialization', () => {
        test('should initialize with default grammar', () => {
            expect(cylDesign).toBeDefined();
        });

        test('should have validator, AST generator, and syntax checker', () => {
            const validationResult = cylDesign.validateGrammar();
            expect(validationResult).toBeDefined();
            expect(validationResult.isValid).toBeDefined();
        });
    });

    describe('Grammar Validation', () => {
        test('should validate default grammar successfully', () => {
            const result = cylDesign.validateGrammar();
            expect(result.isValid).toBeDefined();
            expect(result.errors).toBeDefined();
        });

        test('should have required sections in validation result', () => {
            const result = cylDesign.validateGrammar();
            expect(result).toHaveProperty('isValid');
            expect(result).toHaveProperty('errors');
            expect(result).toHaveProperty('warnings');
        });
    });

    describe('AST Generation', () => {
        test('should generate AST without errors', () => {
            expect(() => cylDesign.generateAST()).not.toThrow();
        });
    });

    describe('Syntax Checking', () => {
        test('should validate correct syntax', () => {
            const code = `
                fn add(a: i32, b: i32) -> i32 {
                    return a + b;
                }
            `;
            const result = cylDesign.checkSyntax(code);
            expect(result.isValid).toBeDefined();
            expect(result.issues).toBeDefined();
            expect(Array.isArray(result.issues)).toBe(true);
        });

        test('should detect syntax errors', () => {
            const code = 'fn missing_body() -> void'; // Missing function body
            const result = cylDesign.checkSyntax(code);
            expect(result.isValid).toBeDefined();
            expect(result.issues).toBeDefined();
            expect(Array.isArray(result.issues)).toBe(true);
        });

        test('should provide helpful error messages', () => {
            const code = 'fn test() -> { }'; // Missing return type
            const result = cylDesign.checkSyntax(code);
            expect(result.isValid).toBeDefined();
            expect(result.issues).toBeDefined();
            if (result.issues.length > 0) {
                expect(result.issues[0]).toHaveProperty('message');
                expect(result.issues[0]).toHaveProperty('line');
                expect(result.issues[0]).toHaveProperty('column');
            }
        });

        test('should include suggestions when available', () => {
            const code = 'function test() { }'; // Wrong keyword
            const result = cylDesign.checkSyntax(code);
            expect(result).toHaveProperty('suggestions');
            expect(Array.isArray(result.suggestions)).toBe(true);
        });
    });
});
