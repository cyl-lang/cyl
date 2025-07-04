export interface LanguageGrammar {
    name: string;
    version: string;
    keywords?: Keyword[];
    operators?: Operator[];
    syntaxRules?: SyntaxRule[];
    types?: TypeDefinition[];
}

export interface Keyword {
    value: string;
    type: string;
    description?: string;
}

export interface Operator {
    symbol: string;
    type: 'binary' | 'unary' | 'ternary';
    precedence: number;
    associativity: 'left' | 'right' | 'none';
    description?: string;
}

export interface SyntaxRule {
    name: string;
    pattern: string;
    description?: string;
    examples?: string[];
}

export interface TypeDefinition {
    name: string;
    kind: 'primitive' | 'composite' | 'generic';
    description?: string;
    properties?: TypeProperty[];
}

export interface TypeProperty {
    name: string;
    type: string;
    optional?: boolean;
    description?: string;
}
