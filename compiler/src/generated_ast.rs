// Generated AST definitions for Cyl language
// Generated from grammar version: 0.1.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ASTNodeType {
    FunctionDeclaration,
    IfStatement,
    ElseStatement,
    ImportStatement,
    ReturnStatement,
    StructDeclaration,
    EnumDeclaration,
    MatchStatement,
    ForStatement,
    WhileStatement,
    BreakStatement,
    ContinueStatement,
    TryStatement,
    CatchStatement,
    ThrowStatement,
    AsyncFunctionDeclaration,
    AwaitExpression,
    VoidType,
    DeclareStatement,
    MutabilityModifier,
    Identifier,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BoolLiteral,
    CharLiteral,
    ArrayLiteral,
    ObjectLiteral,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    MemberExpression,
    IndexExpression,
    AssignmentExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Function(FunctionDeclaration),
    If(IfStatement),
    Else(ElseStatement),
    Import(ImportStatement),
    Return(ReturnStatement),
    Struct(StructDeclaration),
    Enum(EnumDeclaration),
    Match(MatchStatement),
    For(ForStatement),
    While(WhileStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Try(TryStatement),
    Catch(CatchStatement),
    Throw(ThrowStatement),
    AsyncFunction(AsyncFunctionDeclaration),
    Declare(DeclareStatement),
    Declare(DeclareStatement),
    Declare(DeclareStatement),
    Expression(Expression),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add, // +
    Subtract, // -
    Multiply, // *
    Divide, // /
    Modulo, // %
    Equal, // ==
    NotEqual, // !=
    Less, // <
    LessEqual, // <=
    Greater, // >
    GreaterEqual, // >=
    And, // &&
    Or, // ||
    BitwiseAnd, // &
    BitwiseOr, // |
    BitwiseXor, // ^
    LeftShift, // <<
    RightShift, // >>
    Assign, // =
    Arrow, // ->
    Dot, // .
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not, // !
    BitwiseNot, // ~
}

// Implementation methods would go here
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
