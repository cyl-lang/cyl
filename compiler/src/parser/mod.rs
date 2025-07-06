mod functions;
mod statements;
mod expressions;
pub mod helpers;
mod patterns;

// The main Parser struct and parse() entrypoint can remain here, delegating to the submodules.

use crate::ast::*;
use crate::error::CylError;
use crate::parser::helpers::Parser;

impl Parser {
    pub fn parse(&mut self) -> Result<Program, CylError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            // You may want to delegate to a parse_statement method in statements.rs
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }
}
