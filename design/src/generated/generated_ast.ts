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
    ExpressionStatement = 'ExpressionStatement',
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
 * Variable/constant declaration statement
 */
export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    kind: 'let' | 'const' | 'var';
    identifier: Identifier;
    typeAnnotation?: Identifier;
    initializer?: Expression;
}

/**
 * Expression statement wrapper
 */
export interface ExpressionStatement extends BaseASTNode {
    type: ASTNodeType.ExpressionStatement;
    expression: Expression;
}

/**
 * Identifier (variable/function names)
 */
export interface Identifier extends BaseASTNode {
    type: ASTNodeType.Identifier;
    name: string;
}

/**
 * Integer literal
 */
export interface IntLiteral extends BaseASTNode {
    type: ASTNodeType.IntLiteral;
    value: number;
}

/**
 * Float literal
 */
export interface FloatLiteral extends BaseASTNode {
    type: ASTNodeType.FloatLiteral;
    value: number;
}

/**
 * String literal
 */
export interface StringLiteral extends BaseASTNode {
    type: ASTNodeType.StringLiteral;
    value: string;
}

/**
 * Boolean literal
 */
export interface BoolLiteral extends BaseASTNode {
    type: ASTNodeType.BoolLiteral;
    value: boolean;
}

/**
 * Character literal
 */
export interface CharLiteral extends BaseASTNode {
    type: ASTNodeType.CharLiteral;
    value: string;
}

/**
 * Array literal
 */
export interface ArrayLiteral extends BaseASTNode {
    type: ASTNodeType.ArrayLiteral;
    elements: Expression[];
}

/**
 * Object literal
 */
export interface ObjectLiteral extends BaseASTNode {
    type: ASTNodeType.ObjectLiteral;
    properties: ObjectProperty[];
}

/**
 * Object property
 */
export interface ObjectProperty {
    key: Identifier | StringLiteral;
    value: Expression;
}

/**
 * Binary expression (e.g., a + b)
 */
export interface BinaryExpression extends BaseASTNode {
    type: ASTNodeType.BinaryExpression;
    operator: string;
    left: Expression;
    right: Expression;
}

/**
 * Unary expression (e.g., !a, -b)
 */
export interface UnaryExpression extends BaseASTNode {
    type: ASTNodeType.UnaryExpression;
    operator: string;
    argument: Expression;
}

/**
 * Function call expression
 */
export interface CallExpression extends BaseASTNode {
    type: ASTNodeType.CallExpression;
    callee: Expression;
    arguments: Expression[];
}

/**
 * Member access expression (e.g., obj.prop)
 */
export interface MemberExpression extends BaseASTNode {
    type: ASTNodeType.MemberExpression;
    object: Expression;
    property: Identifier;
    computed: boolean;
}

/**
 * Index access expression (e.g., arr[0])
 */
export interface IndexExpression extends BaseASTNode {
    type: ASTNodeType.IndexExpression;
    object: Expression;
    index: Expression;
}

/**
 * Assignment expression
 */
export interface AssignmentExpression extends BaseASTNode {
    type: ASTNodeType.AssignmentExpression;
    operator: string;
    left: Expression;
    right: Expression;
}

/**
 * Await expression
 */
export interface AwaitExpression extends BaseASTNode {
    type: ASTNodeType.AwaitExpression;
    argument: Expression;
}

/**
 * Void type annotation
 */
export interface VoidType extends BaseASTNode {
    type: ASTNodeType.VoidType;
}

/**
 * Mutability modifier
 */
export interface MutabilityModifier extends BaseASTNode {
    type: ASTNodeType.MutabilityModifier;
    kind: 'let' | 'const' | 'mut';
}

