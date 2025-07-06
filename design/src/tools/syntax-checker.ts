import type { LanguageGrammar } from '../types/grammar.js';
import { loadGrammar } from '../grammar/index.js';
import fs from 'fs';

export class SyntaxChecker {
    private readonly grammar: LanguageGrammar;

    constructor(grammar?: LanguageGrammar) {
        this.grammar = grammar ?? loadGrammar();
    }

    public checkSyntax(code: string): SyntaxCheckResult {
        const issues: SyntaxIssue[] = [];
        const suggestions: SyntaxSuggestion[] = [];

        // Tokenize the code (simplified)
        const tokens = this.tokenize(code);

        // Check for basic syntax issues
        this.checkBracketMatching(tokens, issues);
        this.checkKeywordUsage(tokens, issues, suggestions);
        this.checkOperatorUsage(tokens, issues);
        this.checkStatementTermination(tokens, issues);

        return {
            isValid: issues.filter(i => i.severity === 'error').length === 0,
            issues,
            suggestions
        };
    }

    private tokenize(code: string): Token[] {
        const tokens: Token[] = [];
        const lines = code.split('\n');

        for (let lineNum = 0; lineNum < lines.length; lineNum++) {
            const line = lines[lineNum];
            let column = 0;

            // Simple tokenizer - split by whitespace and common delimiters
            const tokenRegex = /(\w+|[{}();.,=<>!&|+\-*/]|"[^"]*"|'[^']*')/g;
            let match;

            while ((match = tokenRegex.exec(line)) !== null) {
                const value = match[1];
                column = match.index;

                tokens.push({
                    value,
                    type: this.getTokenType(value),
                    line: lineNum + 1,
                    column: column + 1,
                    length: value.length
                });
            }
        }

        return tokens;
    }

    private getTokenType(value: string): TokenType {
        // Check if it's a keyword
        if (this.grammar.keywords?.some(k => k.value === value)) {
            return 'keyword';
        }

        // Check if it's an operator
        if (this.grammar.operators?.some(op => op.symbol === value)) {
            return 'operator';
        }

        // Check patterns
        if (/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(value)) {
            return 'identifier';
        }

        if (/^\d+$/.test(value)) {
            return 'number';
        }

        if (/^".*"$/.test(value) || /^'.*'$/.test(value)) {
            return 'string';
        }

        if (['(', ')', '{', '}', '[', ']'].includes(value)) {
            return 'bracket';
        }

        if ([';', ',', '.'].includes(value)) {
            return 'punctuation';
        }

        return 'unknown';
    }

    private checkBracketMatching(tokens: Token[], issues: SyntaxIssue[]): void {
        const stack: Array<{ token: Token; expected: string }> = [];
        const pairs: { [key: string]: string } = {
            '(': ')',
            '{': '}',
            '[': ']'
        };

        for (const token of tokens) {
            if (['(', '{', '['].includes(token.value)) {
                stack.push({ token, expected: pairs[token.value] });
            } else if ([')', '}', ']'].includes(token.value)) {
                if (stack.length === 0) {
                    issues.push({
                        type: 'unmatched_bracket',
                        message: `Unmatched closing bracket: ${token.value}`,
                        severity: 'error',
                        line: token.line,
                        column: token.column,
                        length: token.length
                    });
                } else {
                    const last = stack.pop()!;
                    if (last.expected !== token.value) {
                        issues.push({
                            type: 'mismatched_bracket',
                            message: `Expected ${last.expected}, found ${token.value}`,
                            severity: 'error',
                            line: token.line,
                            column: token.column,
                            length: token.length
                        });
                    }
                }
            }
        }

        // Check for unclosed brackets
        for (const item of stack) {
            issues.push({
                type: 'unclosed_bracket',
                message: `Unclosed bracket: ${item.token.value}`,
                severity: 'error',
                line: item.token.line,
                column: item.token.column,
                length: item.token.length
            });
        }
    }

    private checkKeywordUsage(tokens: Token[], issues: SyntaxIssue[], suggestions: SyntaxSuggestion[]): void {
        for (let i = 0; i < tokens.length; i++) {
            const token = tokens[i];

            if (token.type === 'identifier') {
                // Check for common misspellings of keywords
                const similarKeywords = this.findSimilarKeywords(token.value);
                if (similarKeywords.length > 0) {
                    suggestions.push({
                        type: 'keyword_suggestion',
                        message: `Did you mean: ${similarKeywords.join(', ')}?`,
                        line: token.line,
                        column: token.column,
                        length: token.length,
                        suggestions: similarKeywords
                    });
                }
            }

            // Check for specific keyword contexts
            if (token.value === 'fn') {
                // Function declaration should be followed by identifier
                if (i + 1 < tokens.length && tokens[i + 1].type !== 'identifier') {
                    issues.push({
                        type: 'invalid_function_name',
                        message: 'Function declaration must be followed by a name',
                        severity: 'error',
                        line: token.line,
                        column: token.column,
                        length: token.length
                    });
                }
            }
        }
    }

    private checkOperatorUsage(tokens: Token[], issues: SyntaxIssue[]): void {
        for (let i = 0; i < tokens.length; i++) {
            const token = tokens[i];

            if (token.type === 'operator') {
                const operator = this.grammar.operators?.find(op => op.symbol === token.value);

                if (operator) {
                    // Check binary operator context
                    if (operator.type === 'binary') {
                        const hasLeftOperand = i > 0 && this.canBeOperand(tokens[i - 1]);
                        const hasRightOperand = i + 1 < tokens.length && this.canBeOperand(tokens[i + 1]);

                        if (!hasLeftOperand || !hasRightOperand) {
                            issues.push({
                                type: 'invalid_operator_usage',
                                message: `Binary operator ${token.value} requires operands on both sides`,
                                severity: 'error',
                                line: token.line,
                                column: token.column,
                                length: token.length
                            });
                        }
                    }

                    // Check unary operator context
                    if (operator.type === 'unary') {
                        const hasOperand = i + 1 < tokens.length && this.canBeOperand(tokens[i + 1]);

                        if (!hasOperand) {
                            issues.push({
                                type: 'invalid_operator_usage',
                                message: `Unary operator ${token.value} requires an operand`,
                                severity: 'error',
                                line: token.line,
                                column: token.column,
                                length: token.length
                            });
                        }
                    }
                }
            }
        }
    }

    private checkStatementTermination(tokens: Token[], issues: SyntaxIssue[]): void {
        let expectingSemicolon = false;
        let lastStatementToken: Token | null = null;

        for (const token of tokens) {
            if (this.isStatementStart(token)) {
                expectingSemicolon = true;
                lastStatementToken = token;
            } else if (token.value === ';') {
                expectingSemicolon = false;
                lastStatementToken = null;
            } else if (token.value === '{' || token.value === '}') {
                expectingSemicolon = false;
                lastStatementToken = null;
            }
        }

        if (expectingSemicolon && lastStatementToken) {
            issues.push({
                type: 'missing_semicolon',
                message: 'Statement should be terminated with semicolon',
                severity: 'warning',
                line: lastStatementToken.line,
                column: lastStatementToken.column,
                length: lastStatementToken.length
            });
        }
    }

    private findSimilarKeywords(word: string): string[] {
        if (!this.grammar.keywords) return [];

        return this.grammar.keywords
            .map(k => k.value)
            .filter(keyword => this.calculateSimilarity(word, keyword) > 0.6)
            .slice(0, 3);
    }

    private calculateSimilarity(str1: string, str2: string): number {
        const longer = str1.length > str2.length ? str1 : str2;
        const shorter = str1.length > str2.length ? str2 : str1;

        if (longer.length === 0) return 1.0;

        const editDistance = this.levenshteinDistance(longer, shorter);
        return (longer.length - editDistance) / longer.length;
    }

    private levenshteinDistance(str1: string, str2: string): number {
        const matrix = Array(str2.length + 1).fill(null).map(() => Array(str1.length + 1).fill(null));

        for (let i = 0; i <= str1.length; i++) matrix[0][i] = i;
        for (let j = 0; j <= str2.length; j++) matrix[j][0] = j;

        for (let j = 1; j <= str2.length; j++) {
            for (let i = 1; i <= str1.length; i++) {
                const indicator = str1[i - 1] === str2[j - 1] ? 0 : 1;
                matrix[j][i] = Math.min(
                    matrix[j][i - 1] + 1,
                    matrix[j - 1][i] + 1,
                    matrix[j - 1][i - 1] + indicator
                );
            }
        }

        return matrix[str2.length][str1.length];
    }

    private canBeOperand(token: Token): boolean {
        return ['identifier', 'number', 'string'].includes(token.type) || token.value === ')';
    }

    private isStatementStart(token: Token): boolean {
        const statementKeywords = ['let', 'const', 'return', 'if', 'while', 'for'];
        return token.type === 'keyword' && statementKeywords.includes(token.value);
    }
}

export interface Token {
    value: string;
    type: TokenType;
    line: number;
    column: number;
    length: number;
}

export type TokenType = 'keyword' | 'identifier' | 'operator' | 'number' | 'string' | 'bracket' | 'punctuation' | 'unknown';

export interface SyntaxCheckResult {
    isValid: boolean;
    issues: SyntaxIssue[];
    suggestions: SyntaxSuggestion[];
}

export interface SyntaxIssue {
    type: string;
    message: string;
    severity: 'error' | 'warning';
    line: number;
    column: number;
    length: number;
}

export interface SyntaxSuggestion {
    type: string;
    message: string;
    line: number;
    column: number;
    length: number;
    suggestions: string[];
}

// CLI tool (ESM-compatible)
if (import.meta.url === `file://${process.argv[1]}` || import.meta.url === process.argv[1]) {
    const args = process.argv.slice(2);

    if (args.length === 0) {
        console.log('Usage: syntax-checker <file.cyl>');
        if (typeof process.env.JEST_WORKER_ID === 'undefined') {
            process.exit(1);
        }
    }

    const filename = args[0];

    if (!fs.existsSync(filename)) {
        console.error(`File not found: ${filename}`);
        if (typeof process.env.JEST_WORKER_ID === 'undefined') {
            process.exit(1);
        }
    }

    const code = fs.readFileSync(filename, 'utf8');
    const checker = new SyntaxChecker();
    const result = checker.checkSyntax(code);

    console.log(`Syntax Check Results for ${filename}:`);
    console.log('=====================================');

    if (result.isValid) {
        console.log('✅ Syntax is valid!');
    } else {
        console.log('❌ Syntax errors found!');
    }

    if (result.issues.length > 0) {
        console.log('\nIssues:');
        result.issues.forEach(issue => {
            const icon = issue.severity === 'error' ? '❌' : '⚠️';
            console.log(`  ${icon} Line ${issue.line}:${issue.column} - ${issue.message}`);
        });
    }

    if (result.suggestions.length > 0) {
        console.log('\nSuggestions:');
        result.suggestions.forEach(suggestion => {
            console.log(`  💡 Line ${suggestion.line}:${suggestion.column} - ${suggestion.message}`);
        });
    }

    if (typeof process.env.JEST_WORKER_ID === 'undefined') {
        process.exit(result.isValid ? 0 : 1);
    }
}
