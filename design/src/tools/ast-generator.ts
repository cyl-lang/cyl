import type { LanguageGrammar } from '../types/grammar.js';
import fs from 'fs';
import path from 'path';

export class ASTGenerator {
    private readonly grammar: LanguageGrammar;

    constructor(grammar: LanguageGrammar) {
        this.grammar = grammar;
    }

    public generateRustAST(): string {
        let rustCode = this.generateHeader();
        rustCode += this.generateEnums();
        rustCode += this.generateStructs();
        rustCode += this.generateImplementations();
        return rustCode;
    }

    public generateTypeScriptDefinitions(): string {
        let tsCode = this.generateTSHeader();
        tsCode += this.generateTSEnums();
        tsCode += this.generateTSInterfaces();
        return tsCode;
    }

    private generateHeader(): string {
        return `// Generated AST definitions for Cyl language
// Generated from grammar version: ${this.grammar.version}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

`;
    }

    private generateTSHeader(): string {
        return `// Generated TypeScript AST definitions for Cyl language
// Generated from grammar version: ${this.grammar.version}

export interface SourceLocation {
    line: number;
    column: number;
    offset: number;
    length: number;
}

export interface BaseASTNode {
    type: ASTNodeType;
    location?: SourceLocation;
}

`;
    }

    private generateEnums(): string {
        let code = `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ASTNodeType {
`;

        // Generate from keywords
        const processedTypes = new Set<string>();
        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if (!processedTypes.has(keyword.type)) {
                    code += `    ${keyword.type},\n`;
                    processedTypes.add(keyword.type);
                }
            }
        }

        // Add common expression and literal types
        const commonTypes = [
            'Identifier', 'IntLiteral', 'FloatLiteral', 'StringLiteral',
            'BoolLiteral', 'CharLiteral', 'ArrayLiteral', 'ObjectLiteral',
            'BinaryExpression', 'UnaryExpression', 'CallExpression',
            'MemberExpression', 'IndexExpression', 'AssignmentExpression'
        ];

        for (const type of commonTypes) {
            if (!processedTypes.has(type)) {
                code += `    ${type},\n`;
                processedTypes.add(type);
            }
        }

        code += `}

`;
        return code;
    }

    private generateTSEnums(): string {
        let code = `export enum ASTNodeType {
`;

        // Generate from keywords
        const processedTypes = new Set<string>();

        // Add ExpressionStatement first
        code += `    ExpressionStatement = 'ExpressionStatement',\n`;
        processedTypes.add('ExpressionStatement');

        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if (!processedTypes.has(keyword.type)) {
                    code += `    ${keyword.type} = '${keyword.type}',\n`;
                    processedTypes.add(keyword.type);
                }
            }
        }

        // Add common types
        const commonTypes = [
            'Identifier', 'IntLiteral', 'FloatLiteral', 'StringLiteral',
            'BoolLiteral', 'CharLiteral', 'ArrayLiteral', 'ObjectLiteral',
            'BinaryExpression', 'UnaryExpression', 'CallExpression',
            'MemberExpression', 'IndexExpression', 'AssignmentExpression'
        ];

        for (const type of commonTypes) {
            if (!processedTypes.has(type)) {
                code += `    ${type} = '${type}',\n`;
                processedTypes.add(type);
            }
        }

        code += `}

`;
        return code;
    }

    private generateStructs(): string {
        let code = `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
`;

        // Generate statement types from keywords
        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if (keyword.type.includes('Statement') || keyword.type.includes('Declaration')) {
                    const structName = keyword.type.replace('Statement', '').replace('Declaration', '');
                    code += `    ${structName}(${keyword.type}),\n`;
                }
            }
        }

        code += `    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    CharLiteral(char),
    ArrayLiteral(Vec<Expression>),
    ObjectLiteral(HashMap<String, Expression>),
    
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        property: String,
    },
    IndexAccess {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
}

`;

        // Generate operator enums
        code += this.generateOperatorEnums();

        return code;
    }

    private generateTSInterfaces(): string {
        let code = `export type Statement = 
`;

        // Generate union type for statements - avoid duplicates
        const statementTypes = new Set<string>();

        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if ((keyword.type.includes('Statement') || keyword.type.includes('Declaration')) &&
                    !statementTypes.has(keyword.type)) {
                    code += `  | ${keyword.type}\n`;
                    statementTypes.add(keyword.type);
                }
            }
        }

        code += `  | ExpressionStatement;

export type Expression =
  | Identifier
  | IntLiteral
  | FloatLiteral
  | StringLiteral
  | BoolLiteral
  | CharLiteral
  | ArrayLiteral
  | ObjectLiteral
  | BinaryExpression
  | UnaryExpression
  | CallExpression
  | MemberExpression
  | IndexExpression
  | AssignmentExpression;

// Base interfaces with proper properties
export interface ExpressionStatement extends BaseASTNode {
    type: ASTNodeType.ExpressionStatement;
    expression: Expression;
}

export interface Identifier extends BaseASTNode {
    type: ASTNodeType.Identifier;
    name: string;
}

export interface IntLiteral extends BaseASTNode {
    type: ASTNodeType.IntLiteral;
    value: number;
}

export interface FloatLiteral extends BaseASTNode {
    type: ASTNodeType.FloatLiteral;
    value: number;
}

export interface StringLiteral extends BaseASTNode {
    type: ASTNodeType.StringLiteral;
    value: string;
}

export interface BoolLiteral extends BaseASTNode {
    type: ASTNodeType.BoolLiteral;
    value: boolean;
}

export interface CharLiteral extends BaseASTNode {
    type: ASTNodeType.CharLiteral;
    value: string;
}

export interface ArrayLiteral extends BaseASTNode {
    type: ASTNodeType.ArrayLiteral;
    elements: Expression[];
}

export interface ObjectLiteral extends BaseASTNode {
    type: ASTNodeType.ObjectLiteral;
    properties: Array<{ key: string; value: Expression }>;
}

export interface BinaryExpression extends BaseASTNode {
    type: ASTNodeType.BinaryExpression;
    left: Expression;
    operator: string;
    right: Expression;
}

export interface UnaryExpression extends BaseASTNode {
    type: ASTNodeType.UnaryExpression;
    operator: string;
    operand: Expression;
}

export interface CallExpression extends BaseASTNode {
    type: ASTNodeType.CallExpression;
    callee: Expression;
    arguments: Expression[];
}

export interface MemberExpression extends BaseASTNode {
    type: ASTNodeType.MemberExpression;
    object: Expression;
    property: string;
}

export interface IndexExpression extends BaseASTNode {
    type: ASTNodeType.IndexExpression;
    object: Expression;
    index: Expression;
}

export interface AssignmentExpression extends BaseASTNode {
    type: ASTNodeType.AssignmentExpression;
    target: Expression;
    value: Expression;
}

`;

        // Generate specific interfaces for keywords
        const processedInterfaces = new Set<string>();
        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if ((keyword.type.includes('Statement') || keyword.type.includes('Declaration')) &&
                    !processedInterfaces.has(keyword.type)) {
                    code += this.generateTSInterfaceWithProperties(keyword.type, keyword.description ?? '');
                    processedInterfaces.add(keyword.type);
                }
            }
        }

        return code;
    }

    private generateTSInterfaceWithProperties(typeName: string, description: string): string {
        let properties = '';

        // Add specific properties based on the type
        switch (typeName) {
            case 'FunctionDeclaration':
                properties = `    name: string;
    parameters: Array<{ name: string; type?: string }>;
    returnType?: string;
    body: Statement[];
    isAsync?: boolean;`;
                break;
            case 'IfStatement':
                properties = `    condition: Expression;
    thenBranch: Statement;
    elseBranch?: Statement;`;
                break;
            case 'ImportStatement':
                properties = `    module: string;
    items?: string[];`;
                break;
            case 'ReturnStatement':
                properties = `    value?: Expression;`;
                break;
            case 'StructDeclaration':
                properties = `    name: string;
    fields: Array<{ name: string; type: string; isPublic?: boolean }>;`;
                break;
            case 'EnumDeclaration':
                properties = `    name: string;
    variants: Array<{ name: string; fields?: string[] }>;`;
                break;
            case 'MatchStatement':
                properties = `    expression: Expression;
    arms: Array<{ pattern: string; body: Statement }>;`;
                break;
            case 'ForStatement':
                properties = `    variable: string;
    iterable: Expression;
    body: Statement;`;
                break;
            case 'WhileStatement':
                properties = `    condition: Expression;
    body: Statement;`;
                break;
            case 'DeclareStatement':
                properties = `    name: string;
    valueType?: string;
    value: Expression;
    isMutable?: boolean;`;
                break;
            default:
                // For other statement types, just add basic properties
                properties = `    // Properties for ${typeName}`;
        }

        return `/**
 * ${description}
 */
export interface ${typeName} extends BaseASTNode {
    type: ASTNodeType.${typeName};
${properties}
}

`;
    }

    private generateTSInterface(typeName: string, description: string): string {
        return `/**
 * ${description}
 */
export interface ${typeName} extends BaseASTNode {
    type: ASTNodeType.${typeName};
    // TODO: Add specific properties for ${typeName}
}

`;
    }

    private generateOperatorEnums(): string {
        let code = `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
`;

        if (this.grammar.operators) {
            for (const op of this.grammar.operators) {
                if (op.type === 'binary') {
                    const enumName = this.operatorToEnumName(op.symbol);
                    code += `    ${enumName}, // ${op.symbol}\n`;
                }
            }
        }

        code += `}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
`;

        if (this.grammar.operators) {
            for (const op of this.grammar.operators) {
                if (op.type === 'unary') {
                    const enumName = this.operatorToEnumName(op.symbol);
                    code += `    ${enumName}, // ${op.symbol}\n`;
                }
            }
        }

        code += `}

`;
        return code;
    }

    private generateImplementations(): string {
        return `// Implementation methods would go here
impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
`;
    }

    private operatorToEnumName(symbol: string): string {
        const mapping: { [key: string]: string } = {
            '+': 'Add',
            '-': 'Subtract',
            '*': 'Multiply',
            '/': 'Divide',
            '%': 'Modulo',
            '==': 'Equal',
            '!=': 'NotEqual',
            '<': 'Less',
            '<=': 'LessEqual',
            '>': 'Greater',
            '>=': 'GreaterEqual',
            '&&': 'And',
            '||': 'Or',
            '!': 'Not',
            '&': 'BitwiseAnd',
            '|': 'BitwiseOr',
            '^': 'BitwiseXor',
            '<<': 'LeftShift',
            '>>': 'RightShift',
            '~': 'BitwiseNot',
            '=': 'Assign',
            '->': 'Arrow',
            '.': 'Dot',
        };

        return mapping[symbol] || `Op${symbol.charCodeAt(0)}`;
    }

    public saveToFile(content: string, filename: string, outputDir: string = './generated'): void {
        if (!fs.existsSync(outputDir)) {
            fs.mkdirSync(outputDir, { recursive: true });
        }

        const filePath = path.join(outputDir, filename);
        fs.writeFileSync(filePath, content, 'utf8');
        console.log(`Generated AST file: ${filePath}`);
    }
}

// CLI tool
if (import.meta.url === new URL('../compiler/src/generated/ast-generator', import.meta.url).href) {
    import('../grammar/index.js').then(({ loadGrammar }) => {
        const grammar = loadGrammar();
        const generator = new ASTGenerator(grammar);

        // Generate Rust AST
        const rustAST = generator.generateRustAST();
        generator.saveToFile(rustAST, 'ast.rs', './compiler/src/generated');

        // Generate TypeScript definitions
        const tsAST = generator.generateTypeScriptDefinitions();
        generator.saveToFile(tsAST, 'ast.ts', './design/src/generated');

        console.log('AST generation completed!');
    }).catch(console.error);
}
