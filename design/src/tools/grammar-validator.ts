import type { LanguageGrammar } from '../types/grammar.js';

export class GrammarValidator {
    private readonly grammar: LanguageGrammar;

    constructor(grammar: LanguageGrammar) {
        this.grammar = grammar;
    }

    public validate(): ValidationResult {
        const errors: ValidationError[] = [];
        const warnings: ValidationWarning[] = [];

        // Validate keywords
        this.validateKeywords(errors);

        // Validate operators
        this.validateOperators(errors);

        // Validate syntax rules
        this.validateSyntaxRules(errors, warnings);

        // Check for conflicts
        this.checkConflicts(errors);

        return {
            isValid: errors.length === 0,
            errors,
            warnings
        };
    }

    private validateKeywords(errors: ValidationError[]): void {
        if (!this.grammar.keywords || this.grammar.keywords.length === 0) {
            errors.push({
                type: 'missing_keywords',
                message: 'No keywords defined in grammar',
                severity: 'error'
            });
            return;
        }

        // Check for duplicate keywords
        const seen = new Set<string>();
        for (const keyword of this.grammar.keywords) {
            if (seen.has(keyword.value)) {
                errors.push({
                    type: 'duplicate_keyword',
                    message: `Duplicate keyword: ${keyword.value}`,
                    severity: 'error',
                    location: keyword.value
                });
            }
            seen.add(keyword.value);
        }
    }

    private validateOperators(errors: ValidationError[]): void {
        if (!this.grammar.operators) {
            errors.push({
                type: 'missing_operators',
                message: 'No operators defined in grammar',
                severity: 'error'
            });
            return;
        }

        // Validate operator precedence
        const precedences = new Set<number>();
        for (const operator of this.grammar.operators) {
            if (precedences.has(operator.precedence)) {
                // Multiple operators with same precedence is okay, but check associativity
                const samePrec = this.grammar.operators.filter(op => op.precedence === operator.precedence);
                const associativities = new Set(samePrec.map(op => op.associativity));
                if (associativities.size > 1) {
                    errors.push({
                        type: 'precedence_conflict',
                        message: `Conflicting associativity for precedence ${operator.precedence}`,
                        severity: 'error'
                    });
                }
            }
            precedences.add(operator.precedence);
        }
    }

    private validateSyntaxRules(errors: ValidationError[], warnings: ValidationWarning[]): void {
        if (!this.grammar.syntaxRules) {
            errors.push({
                type: 'missing_syntax_rules',
                message: 'No syntax rules defined in grammar',
                severity: 'error'
            });
            return;
        }

        for (const rule of this.grammar.syntaxRules) {
            // Validate rule structure
            if (!rule.name || !rule.pattern) {
                errors.push({
                    type: 'invalid_rule',
                    message: `Invalid syntax rule: ${rule.name ?? 'unnamed'}`,
                    severity: 'error'
                });
            }

            // Check for unreachable rules
            if (rule.pattern.includes('UNREACHABLE')) {
                warnings.push({
                    type: 'unreachable_rule',
                    message: `Potentially unreachable rule: ${rule.name}`,
                    severity: 'warning'
                });
            }
        }
    }

    private checkConflicts(errors: ValidationError[]): void {
        // Check for keyword-operator conflicts
        const keywords = new Set(this.grammar.keywords?.map(k => k.value) ?? []);
        const operators = new Set(this.grammar.operators?.map(op => op.symbol) ?? []);

        for (const keyword of keywords) {
            if (operators.has(keyword)) {
                errors.push({
                    type: 'keyword_operator_conflict',
                    message: `Conflict between keyword and operator: ${keyword}`,
                    severity: 'error'
                });
            }
        }
    }
}

export interface ValidationResult {
    isValid: boolean;
    errors: ValidationError[];
    warnings: ValidationWarning[];
}

export interface ValidationError {
    type: string;
    message: string;
    severity: 'error';
    location?: string;
}

export interface ValidationWarning {
    type: string;
    message: string;
    severity: 'warning';
    location?: string;
}

// CLI tool (ESM-compatible, always runs when executed)
import('../grammar/index.js').then(({ loadGrammar }) => {
    const grammar = loadGrammar();
    const validator = new GrammarValidator(grammar);
    const result = validator.validate();

    console.log('Grammar Validation Results:');
    console.log('==========================');

    if (result.isValid) {
        console.log('✅ Grammar is valid!');
    } else {
        console.log('❌ Grammar validation failed!');
    }

    if (result.errors.length > 0) {
        console.log('\nErrors:');
        result.errors.forEach(error => {
            console.log(`  ❌ [${error.type}] ${error.message}`);
        });
    }

    if (result.warnings.length > 0) {
        console.log('\nWarnings:');
        result.warnings.forEach(warning => {
            console.log(`  ⚠️  [${warning.type}] ${warning.message}`);
        });
    }

    // Only exit if not running in a test environment
    if (typeof process.env.JEST_WORKER_ID === 'undefined') {
        process.exit(result.isValid ? 0 : 1);
    }
});
