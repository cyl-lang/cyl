// Generated TypeScript AST definitions for Cyl language
// Generated from grammar version: 0.1.0

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

export enum ASTNodeType {
    ExpressionStatement = 'ExpressionStatement',
    FunctionDeclaration = 'FunctionDeclaration',
    IfStatement = 'IfStatement',
    ElseStatement = 'ElseStatement',
    ImportStatement = 'ImportStatement',
    ReturnStatement = 'ReturnStatement',
    StructDeclaration = 'StructDeclaration',
    EnumDeclaration = 'EnumDeclaration',
    MatchStatement = 'MatchStatement',
    ForStatement = 'ForStatement',
    WhileStatement = 'WhileStatement',
    BreakStatement = 'BreakStatement',
    ContinueStatement = 'ContinueStatement',
    TryStatement = 'TryStatement',
    CatchStatement = 'CatchStatement',
    ThrowStatement = 'ThrowStatement',
    AsyncFunctionDeclaration = 'AsyncFunctionDeclaration',
    AwaitExpression = 'AwaitExpression',
    VoidType = 'VoidType',
    DeclareStatement = 'DeclareStatement',
    MutabilityModifier = 'MutabilityModifier',
    Identifier = 'Identifier',
    IntLiteral = 'IntLiteral',
    FloatLiteral = 'FloatLiteral',
    StringLiteral = 'StringLiteral',
    BoolLiteral = 'BoolLiteral',
    CharLiteral = 'CharLiteral',
    ArrayLiteral = 'ArrayLiteral',
    ObjectLiteral = 'ObjectLiteral',
    BinaryExpression = 'BinaryExpression',
    UnaryExpression = 'UnaryExpression',
    CallExpression = 'CallExpression',
    MemberExpression = 'MemberExpression',
    IndexExpression = 'IndexExpression',
    AssignmentExpression = 'AssignmentExpression',
}

export type Statement = 
  | FunctionDeclaration
  | IfStatement
  | ElseStatement
  | ImportStatement
  | ReturnStatement
  | StructDeclaration
  | EnumDeclaration
  | MatchStatement
  | ForStatement
  | WhileStatement
  | BreakStatement
  | ContinueStatement
  | TryStatement
  | CatchStatement
  | ThrowStatement
  | AsyncFunctionDeclaration
  | DeclareStatement
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

/**
 * Function declaration
 */
export interface FunctionDeclaration extends BaseASTNode {
    type: ASTNodeType.FunctionDeclaration;
    name: string;
    parameters: Array<{ name: string; type?: string }>;
    returnType?: string;
    body: Statement[];
    isAsync?: boolean;
}

/**
 * Conditional statement
 */
export interface IfStatement extends BaseASTNode {
    type: ASTNodeType.IfStatement;
    condition: Expression;
    thenBranch: Statement;
    elseBranch?: Statement;
}

/**
 * Alternative branch
 */
export interface ElseStatement extends BaseASTNode {
    type: ASTNodeType.ElseStatement;
    // Properties for ElseStatement
}

/**
 * Module import
 */
export interface ImportStatement extends BaseASTNode {
    type: ASTNodeType.ImportStatement;
    module: string;
    items?: string[];
}

/**
 * Return from function
 */
export interface ReturnStatement extends BaseASTNode {
    type: ASTNodeType.ReturnStatement;
    value?: Expression;
}

/**
 * Structure type definition
 */
export interface StructDeclaration extends BaseASTNode {
    type: ASTNodeType.StructDeclaration;
    name: string;
    fields: Array<{ name: string; type: string; isPublic?: boolean }>;
}

/**
 * Enumeration type definition
 */
export interface EnumDeclaration extends BaseASTNode {
    type: ASTNodeType.EnumDeclaration;
    name: string;
    variants: Array<{ name: string; fields?: string[] }>;
}

/**
 * Pattern matching
 */
export interface MatchStatement extends BaseASTNode {
    type: ASTNodeType.MatchStatement;
    expression: Expression;
    arms: Array<{ pattern: string; body: Statement }>;
}

/**
 * Loop statement
 */
export interface ForStatement extends BaseASTNode {
    type: ASTNodeType.ForStatement;
    variable: string;
    iterable: Expression;
    body: Statement;
}

/**
 * While loop
 */
export interface WhileStatement extends BaseASTNode {
    type: ASTNodeType.WhileStatement;
    condition: Expression;
    body: Statement;
}

/**
 * Break from loop
 */
export interface BreakStatement extends BaseASTNode {
    type: ASTNodeType.BreakStatement;
    // Properties for BreakStatement
}

/**
 * Continue loop
 */
export interface ContinueStatement extends BaseASTNode {
    type: ASTNodeType.ContinueStatement;
    // Properties for ContinueStatement
}

/**
 * Exception handling
 */
export interface TryStatement extends BaseASTNode {
    type: ASTNodeType.TryStatement;
    // Properties for TryStatement
}

/**
 * Exception catching
 */
export interface CatchStatement extends BaseASTNode {
    type: ASTNodeType.CatchStatement;
    // Properties for CatchStatement
}

/**
 * Throw exception
 */
export interface ThrowStatement extends BaseASTNode {
    type: ASTNodeType.ThrowStatement;
    // Properties for ThrowStatement
}

/**
 * Async function
 */
export interface AsyncFunctionDeclaration extends BaseASTNode {
    type: ASTNodeType.AsyncFunctionDeclaration;
    // Properties for AsyncFunctionDeclaration
}

/**
 * Variable declaration
 */
export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    name: string;
    valueType?: string;
    value: Expression;
    isMutable?: boolean;
}

