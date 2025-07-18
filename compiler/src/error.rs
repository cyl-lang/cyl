use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)] // Error suffix is conventional for error types
#[allow(dead_code)] // Error variants will be used as the compiler develops
pub enum CylError {
    #[error("Lexical error: {message} at line {line}, column {column}")]
    LexError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Parse error: {message} at line {line}, column {column}")]
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Type error: {message}")]
    TypeError { message: String },

    #[error("Semantic error: {message}")]
    SemanticError { message: String },

    #[error("Code generation error: {message}")]
    CodeGenError { message: String },

    #[error("Runtime error: {message}")]
    RuntimeError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
