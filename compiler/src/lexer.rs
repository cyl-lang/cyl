use crate::error::CylError;
use logos::Logos;
use serde::{Deserialize, Serialize};

#[derive(Logos, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("import")]
    Import,
    #[token("return")]
    Return,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("match")]
    Match,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("throw")]
    Throw,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("declare")]
    Declare,
    #[token("void")]
    Void,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("mut")]
    Mut,

    // Types
    #[token("int")]
    IntType,
    #[token("float")]
    FloatType,
    #[token("string")]
    StringType,
    #[token("bool")]
    BoolType,
    #[token("char")]
    CharType,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_owned())]
    StringLiteral(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLiteral(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(f64),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLiteral(bool),

    #[regex(r"'[^']*'", |lex| lex.slice().chars().nth(1))]
    CharLiteral(char),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    // Operators
    #[token("=")]
    Assign,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<", priority = 2)]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">", priority = 2)]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,

    // Punctuation
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("<", priority = 1)]
    LeftAngle,
    #[token(">", priority = 1)]
    RightAngle,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("?")]
    Question,

    // Special
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Comment,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    Eof,
}

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token>,
    current_line: usize,
    current_column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenWithLocation {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub span: std::ops::Range<usize>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            current_line: 1,
            current_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenWithLocation>, CylError> {
        let mut tokens = Vec::new();

        while let Some(result) = self.lexer.next() {
            let span = self.lexer.span();
            let token_line = self.current_line;
            let token_column = self.current_column;

            // Update line and column tracking first
            let slice = self.lexer.slice();
            for ch in slice.chars() {
                if ch == '\n' {
                    self.current_line += 1;
                    self.current_column = 1;
                } else {
                    self.current_column += 1;
                }
            }

            match result {
                Ok(token) => {
                    tokens.push(TokenWithLocation {
                        token,
                        line: token_line,
                        column: token_column,
                        span: span.clone(),
                    });
                }
                Err(_) => {
                    return Err(CylError::LexError {
                        message: format!("Invalid token at position {}", span.start),
                        line: token_line,
                        column: token_column,
                    });
                }
            }
        }

        tokens.push(TokenWithLocation {
            token: Token::Eof,
            line: self.current_line,
            column: self.current_column,
            span: self.lexer.span(),
        });

        Ok(tokens)
    }
}
