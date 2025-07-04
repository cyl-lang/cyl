import { ASTNodeType, Program, Statement, Expression, TypeNode } from '../ast/nodes';
import { LanguageGrammar } from '../types/grammar';
import * as fs from 'fs';
import * as path from 'path';

export class ASTGenerator {
    private grammar: LanguageGrammar;

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

        // Generate union type for statements
        if (this.grammar.keywords) {
            const statements = this.grammar.keywords
                .filter(k => k.type.includes('Statement') || k.type.includes('Declaration'))
                .map(k => `  | ${k.type}`)
                .join('\n');
            code += statements;
        }

        code += `
  | ExpressionStatement;

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

`;

        // Generate specific interfaces
        if (this.grammar.keywords) {
            for (const keyword of this.grammar.keywords) {
                if (keyword.type.includes('Statement') || keyword.type.includes('Declaration')) {
                    code += this.generateTSInterface(keyword.type, keyword.description || '');
                }
            }
        }

        return code;
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
if (require.main === module) {
    import('../grammar').then(({ loadGrammar }) => {
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
