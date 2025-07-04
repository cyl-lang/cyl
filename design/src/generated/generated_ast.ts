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
  | DeclareStatement
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

/**
 * Function declaration keyword
 */
export interface FunctionDeclaration extends BaseASTNode {
    type: ASTNodeType.FunctionDeclaration;
    // TODO: Add specific properties for FunctionDeclaration
}

/**
 * Conditional statement
 */
export interface IfStatement extends BaseASTNode {
    type: ASTNodeType.IfStatement;
    // TODO: Add specific properties for IfStatement
}

/**
 * Alternative branch for conditional
 */
export interface ElseStatement extends BaseASTNode {
    type: ASTNodeType.ElseStatement;
    // TODO: Add specific properties for ElseStatement
}

/**
 * Module import statement
 */
export interface ImportStatement extends BaseASTNode {
    type: ASTNodeType.ImportStatement;
    // TODO: Add specific properties for ImportStatement
}

/**
 * Return from function
 */
export interface ReturnStatement extends BaseASTNode {
    type: ASTNodeType.ReturnStatement;
    // TODO: Add specific properties for ReturnStatement
}

/**
 * Structure type definition
 */
export interface StructDeclaration extends BaseASTNode {
    type: ASTNodeType.StructDeclaration;
    // TODO: Add specific properties for StructDeclaration
}

/**
 * Enumeration type definition
 */
export interface EnumDeclaration extends BaseASTNode {
    type: ASTNodeType.EnumDeclaration;
    // TODO: Add specific properties for EnumDeclaration
}

/**
 * Pattern matching statement
 */
export interface MatchStatement extends BaseASTNode {
    type: ASTNodeType.MatchStatement;
    // TODO: Add specific properties for MatchStatement
}

/**
 * For loop statement
 */
export interface ForStatement extends BaseASTNode {
    type: ASTNodeType.ForStatement;
    // TODO: Add specific properties for ForStatement
}

/**
 * While loop statement
 */
export interface WhileStatement extends BaseASTNode {
    type: ASTNodeType.WhileStatement;
    // TODO: Add specific properties for WhileStatement
}

/**
 * Break from loop
 */
export interface BreakStatement extends BaseASTNode {
    type: ASTNodeType.BreakStatement;
    // TODO: Add specific properties for BreakStatement
}

/**
 * Continue to next iteration
 */
export interface ContinueStatement extends BaseASTNode {
    type: ASTNodeType.ContinueStatement;
    // TODO: Add specific properties for ContinueStatement
}

/**
 * Exception handling block
 */
export interface TryStatement extends BaseASTNode {
    type: ASTNodeType.TryStatement;
    // TODO: Add specific properties for TryStatement
}

/**
 * Exception catching block
 */
export interface CatchStatement extends BaseASTNode {
    type: ASTNodeType.CatchStatement;
    // TODO: Add specific properties for CatchStatement
}

/**
 * Throw an exception
 */
export interface ThrowStatement extends BaseASTNode {
    type: ASTNodeType.ThrowStatement;
    // TODO: Add specific properties for ThrowStatement
}

/**
 * Asynchronous function declaration
 */
export interface AsyncFunctionDeclaration extends BaseASTNode {
    type: ASTNodeType.AsyncFunctionDeclaration;
    // TODO: Add specific properties for AsyncFunctionDeclaration
}

/**
 * Variable declaration (legacy)
 */
export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    // TODO: Add specific properties for DeclareStatement
}

/**
 * Variable declaration
 */
export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    // TODO: Add specific properties for DeclareStatement
}

/**
 * Constant declaration
 */
export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    // TODO: Add specific properties for DeclareStatement
}

