// Tests for new Cyl language features: generics, default args, tuples, nullable, dynamic, pattern matching

use cylc::lexer::Lexer;
use cylc::parser::helpers::Parser;

fn parse_ok(src: &str) {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse().expect("parsing failed");
}

#[test]
fn test_var_decl_type_infer_and_angle() {
    parse_ok("x = 42;");
    parse_ok("y <float> = 3.14;");
    parse_ok("const PI <float> = 3.14;");
}

#[test]
fn test_fn_generics_and_defaults() {
    parse_ok("fn add<T>(a: T, b: T = 0) -> T { return a + b; }");
    parse_ok("fn get_coords() -> (int, int) { return (1, 2); }");
}

#[test]
fn test_nullable_and_dynamic_types() {
    parse_ok("let x: int? = null;");
    parse_ok("let y: dynamic = 42;");
}

#[test]
fn test_pattern_matching() {
    parse_ok("match result { Ok(v) => v, Err(e) => 0 }");
}

#[test]
fn test_struct_enum_generics() {
    parse_ok("struct Point<T> { x: T, y: T }");
    parse_ok("enum Option<T> { Some(T), None }");
}
