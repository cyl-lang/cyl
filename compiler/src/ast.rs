use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Import(ImportStatement),
    Function(FunctionDeclaration),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
    Declare(DeclareStatement),
    Expression(Expression),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Match(MatchStatement),
    Try(TryStatement),
    Break,
    Continue,
    Block(BlockStatement), // <-- Added to support block statements
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStatement {
    pub module: String,
    pub items: Option<Vec<String>>, // None for import all
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: BlockStatement,
    pub is_async: bool,
    pub type_parameters: Vec<String>, // NEW: generics
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
    pub default_value: Option<Expression>, // NEW: default args
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub name: String,
    pub fields: Vec<StructField>,
    pub type_parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub type_parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<Type>>, // None for unit variant
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeclareStatement {
    pub name: String,
    pub value: Expression,
    pub var_type: Option<Type>,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: BlockStatement,
    pub else_block: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryStatement {
    pub body: BlockStatement,
    pub catch_clauses: Vec<CatchClause>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub exception_type: Option<Type>,
    pub variable: Option<String>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
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
    // Null and Dynamic expressions
    Null,    // NEW: null literal
    Dynamic, // NEW: dynamic literal

    // Operations
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    // Function and member access
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
    TupleLiteral(Vec<Expression>), // NEW: tuple literal
    // Async
    Await(Box<Expression>),

    // Assignment
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
    BitwiseNot,
    Await,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    String,
    Bool,
    Char,
    Void,
    Dynamic, // NEW: gradual typing
    Null,    // NEW: null type
    // Collection types
    Array(Box<Type>),
    Tuple(Vec<Type>), // NEW: tuple types
    // Custom types
    Custom(String),
    // Generic types
    Generic(String, Vec<String>), // e.g., Result<T, E>
    // Function types
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
    // Optional types
    Optional(Box<Type>),
    Infer, // NEW: type inference for parameters
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Identifier(String),
    Literal(Expression),
    Wildcard,
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    Enum {
        variant: String,
        fields: Vec<Pattern>,
    },
    TupleOrEnum(String, Vec<Pattern>), // NEW: tuple/enum destructuring
    Tuple(Vec<Pattern>),               // NEW: tuple pattern
}
