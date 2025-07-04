export enum ASTNodeType {
    // Program structure
    Program = 'Program',

    // Statements
    FunctionDeclaration = 'FunctionDeclaration',
    StructDeclaration = 'StructDeclaration',
    EnumDeclaration = 'EnumDeclaration',
    ImportStatement = 'ImportStatement',
    DeclareStatement = 'DeclareStatement',
    ReturnStatement = 'ReturnStatement',
    IfStatement = 'IfStatement',
    WhileStatement = 'WhileStatement',
    ForStatement = 'ForStatement',
    MatchStatement = 'MatchStatement',
    TryStatement = 'TryStatement',
    CatchStatement = 'CatchStatement',
    ThrowStatement = 'ThrowStatement',
    BreakStatement = 'BreakStatement',
    ContinueStatement = 'ContinueStatement',
    BlockStatement = 'BlockStatement',
    ExpressionStatement = 'ExpressionStatement',

    // Expressions
    Identifier = 'Identifier',
    IntLiteral = 'IntLiteral',
    FloatLiteral = 'FloatLiteral',
    StringLiteral = 'StringLiteral',
    BoolLiteral = 'BoolLiteral',
    CharLiteral = 'CharLiteral',
    ArrayLiteral = 'ArrayLiteral',
    ObjectLiteral = 'ObjectLiteral',

    // Operations
    BinaryExpression = 'BinaryExpression',
    UnaryExpression = 'UnaryExpression',
    AssignmentExpression = 'AssignmentExpression',
    CallExpression = 'CallExpression',
    MemberExpression = 'MemberExpression',
    IndexExpression = 'IndexExpression',
    AwaitExpression = 'AwaitExpression',

    // Types
    IntType = 'IntType',
    FloatType = 'FloatType',
    StringType = 'StringType',
    BoolType = 'BoolType',
    CharType = 'CharType',
    VoidType = 'VoidType',
    ArrayType = 'ArrayType',
    CustomType = 'CustomType',
    FunctionType = 'FunctionType',
    GenericType = 'GenericType',
    OptionalType = 'OptionalType',

    // Other
    Parameter = 'Parameter',
    StructField = 'StructField',
    EnumVariant = 'EnumVariant',
    MatchArm = 'MatchArm',
    Pattern = 'Pattern',
    TypeParameter = 'TypeParameter',
}

export interface BaseASTNode {
    type: ASTNodeType;
    location?: SourceLocation;
}

export interface SourceLocation {
    line: number;
    column: number;
    offset: number;
    length: number;
}

export interface Program extends BaseASTNode {
    type: ASTNodeType.Program;
    statements: Statement[];
}

export type Statement =
    | FunctionDeclaration
    | StructDeclaration
    | EnumDeclaration
    | ImportStatement
    | DeclareStatement
    | ReturnStatement
    | IfStatement
    | WhileStatement
    | ForStatement
    | MatchStatement
    | TryStatement
    | BreakStatement
    | ContinueStatement
    | BlockStatement
    | ExpressionStatement;

export interface FunctionDeclaration extends BaseASTNode {
    type: ASTNodeType.FunctionDeclaration;
    name: string;
    parameters: Parameter[];
    returnType?: TypeNode;
    body: BlockStatement;
    isAsync: boolean;
}

export interface Parameter extends BaseASTNode {
    type: ASTNodeType.Parameter;
    name: string;
    paramType: TypeNode;
    isMutable: boolean;
}

export interface StructDeclaration extends BaseASTNode {
    type: ASTNodeType.StructDeclaration;
    name: string;
    fields: StructField[];
    typeParameters: string[];
}

export interface StructField extends BaseASTNode {
    type: ASTNodeType.StructField;
    name: string;
    fieldType: TypeNode;
    isPublic: boolean;
}

export interface EnumDeclaration extends BaseASTNode {
    type: ASTNodeType.EnumDeclaration;
    name: string;
    variants: EnumVariant[];
}

export interface EnumVariant extends BaseASTNode {
    type: ASTNodeType.EnumVariant;
    name: string;
    fields?: TypeNode[];
}

export interface ImportStatement extends BaseASTNode {
    type: ASTNodeType.ImportStatement;
    module: string;
    items?: string[];
}

export interface DeclareStatement extends BaseASTNode {
    type: ASTNodeType.DeclareStatement;
    name: string;
    value: Expression;
    varType?: TypeNode;
    isMutable: boolean;
}

export interface ReturnStatement extends BaseASTNode {
    type: ASTNodeType.ReturnStatement;
    value?: Expression;
}

export interface IfStatement extends BaseASTNode {
    type: ASTNodeType.IfStatement;
    condition: Expression;
    thenBlock: BlockStatement;
    elseBlock?: Statement;
}

export interface WhileStatement extends BaseASTNode {
    type: ASTNodeType.WhileStatement;
    condition: Expression;
    body: BlockStatement;
}

export interface ForStatement extends BaseASTNode {
    type: ASTNodeType.ForStatement;
    variable: string;
    iterable: Expression;
    body: BlockStatement;
}

export interface MatchStatement extends BaseASTNode {
    type: ASTNodeType.MatchStatement;
    expression: Expression;
    arms: MatchArm[];
}

export interface MatchArm extends BaseASTNode {
    type: ASTNodeType.MatchArm;
    pattern: Pattern;
    guard?: Expression;
    body: BlockStatement;
}

export interface TryStatement extends BaseASTNode {
    type: ASTNodeType.TryStatement;
    body: BlockStatement;
    catchClauses: CatchClause[];
}

export interface CatchClause extends BaseASTNode {
    type: ASTNodeType.CatchStatement;
    exceptionType?: TypeNode;
    variable?: string;
    body: BlockStatement;
}

export interface BreakStatement extends BaseASTNode {
    type: ASTNodeType.BreakStatement;
}

export interface ContinueStatement extends BaseASTNode {
    type: ASTNodeType.ContinueStatement;
}

export interface BlockStatement extends BaseASTNode {
    type: ASTNodeType.BlockStatement;
    statements: Statement[];
}

export interface ExpressionStatement extends BaseASTNode {
    type: ASTNodeType.ExpressionStatement;
    expression: Expression;
}

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
    | AssignmentExpression
    | CallExpression
    | MemberExpression
    | IndexExpression
    | AwaitExpression;

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
    properties: ObjectProperty[];
}

export interface ObjectProperty {
    key: string;
    value: Expression;
}

export interface BinaryExpression extends BaseASTNode {
    type: ASTNodeType.BinaryExpression;
    left: Expression;
    operator: BinaryOperator;
    right: Expression;
}

export interface UnaryExpression extends BaseASTNode {
    type: ASTNodeType.UnaryExpression;
    operator: UnaryOperator;
    operand: Expression;
}

export interface AssignmentExpression extends BaseASTNode {
    type: ASTNodeType.AssignmentExpression;
    target: Expression;
    value: Expression;
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

export interface AwaitExpression extends BaseASTNode {
    type: ASTNodeType.AwaitExpression;
    argument: Expression;
}

export type TypeNode =
    | IntType
    | FloatType
    | StringType
    | BoolType
    | CharType
    | VoidType
    | ArrayType
    | CustomType
    | FunctionType
    | GenericType
    | OptionalType;

export interface IntType extends BaseASTNode {
    type: ASTNodeType.IntType;
}

export interface FloatType extends BaseASTNode {
    type: ASTNodeType.FloatType;
}

export interface StringType extends BaseASTNode {
    type: ASTNodeType.StringType;
}

export interface BoolType extends BaseASTNode {
    type: ASTNodeType.BoolType;
}

export interface CharType extends BaseASTNode {
    type: ASTNodeType.CharType;
}

export interface VoidType extends BaseASTNode {
    type: ASTNodeType.VoidType;
}

export interface ArrayType extends BaseASTNode {
    type: ASTNodeType.ArrayType;
    elementType: TypeNode;
}

export interface CustomType extends BaseASTNode {
    type: ASTNodeType.CustomType;
    name: string;
}

export interface FunctionType extends BaseASTNode {
    type: ASTNodeType.FunctionType;
    parameters: TypeNode[];
    returnType: TypeNode;
}

export interface GenericType extends BaseASTNode {
    type: ASTNodeType.GenericType;
    name: string;
    typeArguments: TypeNode[];
}

export interface OptionalType extends BaseASTNode {
    type: ASTNodeType.OptionalType;
    baseType: TypeNode;
}

export interface Pattern extends BaseASTNode {
    type: ASTNodeType.Pattern;
    kind: 'identifier' | 'literal' | 'wildcard' | 'struct' | 'enum';
    value?: string | number | boolean;
    name?: string;
    fields?: PatternField[];
}

export interface PatternField {
    name: string;
    pattern: Pattern;
}

export enum BinaryOperator {
    Add = '+',
    Subtract = '-',
    Multiply = '*',
    Divide = '/',
    Modulo = '%',
    Equal = '==',
    NotEqual = '!=',
    Less = '<',
    LessEqual = '<=',
    Greater = '>',
    GreaterEqual = '>=',
    And = '&&',
    Or = '||',
    BitwiseAnd = '&',
    BitwiseOr = '|',
    BitwiseXor = '^',
    LeftShift = '<<',
    RightShift = '>>',
}

export enum UnaryOperator {
    Not = '!',
    Minus = '-',
    Plus = '+',
    BitwiseNot = '~',
}