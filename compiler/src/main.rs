use anyhow::Result;
pub use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod ast;
#[cfg(feature = "llvm")]
mod codegen;
#[cfg(feature = "cranelift")]
mod cranelift_codegen;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod stdlib;

#[cfg(feature = "llvm")]
use crate::codegen::LLVMCodegen;
#[cfg(feature = "cranelift")]
use crate::cranelift_codegen::CraneliftCodegen;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
#[cfg(feature = "llvm")]
use inkwell::context::Context;

#[derive(Parser)]
#[command(name = "cylc")]
#[command(about = "Cyl programming language compiler")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and run a Cyl program
    Run {
        /// Input file to compile and run
        file: PathBuf,
        /// Optimization level (0-3)
        #[arg(short = 'O', default_value = "1")]
        opt_level: u8,
        /// Enable debug information
        #[arg(short, long)]
        debug: bool,
        /// Backend to use: interpreter, cranelift, llvm
        #[arg(long, default_value = "cranelift")]
        backend: String,
        /// Suppress output messages
        #[arg(short, long)]
        quiet: bool,
    },
    /// Compile a Cyl program to executable
    Build {
        /// Input file to compile
        file: PathBuf,
        /// Output executable name
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Optimization level (0-3)
        #[arg(short = 'O', default_value = "2")]
        opt_level: u8,
        /// Enable debug information
        #[arg(short, long)]
        debug: bool,
        /// Backend to use: cranelift, llvm, interpreter
        #[arg(long, default_value = "cranelift")]
        backend: String,
    },
    /// Check syntax without compiling
    Check {
        /// Input file to check
        file: PathBuf,
    },
    /// Show AST for a Cyl program
    Ast {
        /// Input file to parse
        file: PathBuf,
        /// Output format (json, pretty)
        #[arg(short, long, default_value = "pretty")]
        format: String,
    },
    /// Run automated tests
    Test {
        /// Test pattern to filter tests (optional)
        #[arg(short, long)]
        pattern: Option<String>,
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
        /// Continue running tests after first failure
        #[arg(short, long)]
        continue_on_failure: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            opt_level,
            debug,
            backend,
            quiet,
        } => {
            compile_and_run(&file, opt_level, debug, &backend, quiet)?;
        }
        Commands::Build {
            file,
            output,
            opt_level,
            debug,
            backend,
        } => {
            compile_to_executable(&file, output, opt_level, debug, &backend)?;
        }
        Commands::Check { file } => {
            check_syntax(&file)?;
        }
        Commands::Ast { file, format } => {
            show_ast(&file, &format)?;
        }
        Commands::Test {
            pattern,
            verbose,
            continue_on_failure,
        } => {
            run_tests(pattern, verbose, continue_on_failure)?;
        }
    }

    Ok(())
}

fn print_error_with_context(error: &crate::error::CylError, source: &str) {
    match error {
        crate::error::CylError::LexError {
            message,
            line,
            column,
        }
        | crate::error::CylError::ParseError {
            message,
            line,
            column,
        } => {
            eprintln!("[error] {message} at line {line}, column {column}");
            if let Some(src_line) = source.lines().nth(line.saturating_sub(1)) {
                eprintln!("   {src_line}");
                let mut caret = String::new();
                for _ in 1..*column {
                    caret.push(' ');
                }
                caret.push('^');
                eprintln!("   {caret}");
            }
        }
        _ => {
            eprintln!("[error] {error}");
        }
    }
}

fn compile_and_run(file: &PathBuf, _opt_level: u8, _debug: bool, backend: &str, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "Compiling and running: {} (Backend: {})",
            file.display(),
            backend
        );
    }

    // Read source file
    let source = std::fs::read_to_string(file)?;

    // Lex and parse
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };
    let mut parser = parser::helpers::Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };

    match backend {
        "llvm" => {
            #[cfg(feature = "llvm")]
            {
                // Use LLVM backend
                let context = Context::create();
                let mut llvm_codegen = match LLVMCodegen::new(&context) {
                    Ok(cg) => cg,
                    Err(e) => {
                        eprintln!("Failed to initialize LLVM codegen: {e}");
                        std::process::exit(1);
                    }
                };

                match llvm_codegen.compile_program(&program) {
                    Ok(()) => {
                        println!("Successfully compiled with LLVM!");
                        llvm_codegen.print_ir();
                        
                        // For now, also run with interpreter to get output
                        let mut interpreter = Interpreter::new();
                        interpreter.run(&program);
                    }
                    Err(e) => {
                        eprintln!("LLVM compilation error: {e}");
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(feature = "llvm"))]
            {
                eprintln!("LLVM support not compiled in. Please rebuild with --features llvm");
                std::process::exit(1);
            }
        }
        "cranelift" => {
            #[cfg(feature = "cranelift")]
            {
                // Use Cranelift backend for compilation, then interpreter for execution
                let mut cranelift_codegen = match CraneliftCodegen::new() {
                    Ok(cg) => cg,
                    Err(e) => {
                        eprintln!("Failed to initialize Cranelift codegen: {e}");
                        std::process::exit(1);
                    }
                };

                match cranelift_codegen.compile_program(&program) {
                    Ok(()) => {
                        println!("Successfully compiled with Cranelift!");
                        cranelift_codegen.print_ir();
                        
                        // Always run with interpreter to get output
                        let mut interpreter = Interpreter::new();
                        interpreter.run(&program);
                    }
                    Err(e) => {
                        // If Cranelift compilation fails due to unimplemented features,
                        // fall back to interpreter-only mode
                        if e.to_string().contains("not implemented") || e.to_string().contains("String literals") {
                            eprintln!("Cranelift compilation failed due to unimplemented features, falling back to interpreter...");
                            let mut interpreter = Interpreter::new();
                            interpreter.run(&program);
                        } else {
                            eprintln!("Cranelift compilation error: {e}");
                            std::process::exit(1);
                        }
                    }
                }
            }
            #[cfg(not(feature = "cranelift"))]
            {
                eprintln!("Cranelift support not compiled in. Please rebuild with --features cranelift");
                std::process::exit(1);
            }
        }
        "interpreter" | _ => {
            // Use interpreter (fallback for any unrecognized backend)
            let mut interpreter = Interpreter::new();
            interpreter.run(&program);
        }
    }

    Ok(())
}

fn compile_to_executable(
    file: &PathBuf,
    output: Option<PathBuf>,
    _opt_level: u8,
    _debug: bool,
    backend: &str,
) -> Result<()> {
    let output_name = output.unwrap_or_else(|| file.with_extension(""));

    println!(
        "Compiling {} to {} (Backend: {})",
        file.display(),
        output_name.display(),
        backend
    );

    // Read source file
    let source = std::fs::read_to_string(file)?;

    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    // Parsing
    let mut parser = parser::helpers::Parser::new(tokens);
    let ast = parser.parse()?;

    match backend {
        "llvm" => {
            #[cfg(feature = "llvm")]
            {
                // Use LLVM backend
                let context = Context::create();
                let mut llvm_codegen = LLVMCodegen::new(&context)?;
                llvm_codegen.compile_program(&ast)?;

                // Generate executable
                llvm_codegen.compile_to_executable(&output_name, opt_level)?;
                println!(
                    "Successfully generated executable: {}",
                    output_name.display()
                );
            }
            #[cfg(not(feature = "llvm"))]
            {
                eprintln!("LLVM support not compiled in. Please rebuild with --features llvm");
                std::process::exit(1);
            }
        }
        "cranelift" | _ => {
            #[cfg(feature = "cranelift")]
            {
                // Use Cranelift backend
                let mut cranelift_codegen = CraneliftCodegen::new()?;
                cranelift_codegen.compile_program(&ast)?;

                // Generate object file (for now)
                let obj_name = output_name.with_extension("o");
                cranelift_codegen.write_object_file(obj_name.to_str().unwrap())?;
                println!(
                    "Successfully generated object file: {} (linking to executable not yet implemented)",
                    obj_name.display()
                );
            }
            #[cfg(not(feature = "cranelift"))]
            {
                eprintln!("Cranelift support not compiled in. Please rebuild with --features cranelift");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn check_syntax(file: &PathBuf) -> Result<()> {
    println!("Checking syntax: {}", file.display());

    // Read source file
    let source = std::fs::read_to_string(file)?;

    // Print tokens for debugging
    print_tokens(&source);

    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };

    // Parsing (syntax check)
    let mut parser = parser::helpers::Parser::new(tokens);
    let _ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };

    println!("✓ Syntax is valid");

    Ok(())
}

fn show_ast(file: &PathBuf, format: &str) -> Result<()> {
    // Read source file
    let source = std::fs::read_to_string(file)?;

    // Print tokens for debugging
    print_tokens(&source);

    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };

    // Parsing
    let mut parser = parser::helpers::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            print_error_with_context(&e, &source);
            std::process::exit(1);
        }
    };

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&ast)?;
            println!("{json}");
        }
        "pretty" => {
            println!("{ast:#?}");
        }
        _ => {
            anyhow::bail!("Unknown format: {}. Use 'json' or 'pretty'", format);
        }
    }

    Ok(())
}

fn print_tokens(source: &str) {
    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Ok(tokens) => {
            for t in tokens {
                println!("{:?} (line {}, col {})", t.token, t.line, t.column);
            }
        }
        Err(e) => {
            println!("Lexer error: {e}");
        }
    }
}

fn try_parse_file(source: &str) -> Result<crate::ast::Program> {
    print_tokens(source); // Print tokens for debugging

    // Lexical analysis
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = crate::parser::helpers::Parser::new(tokens);
    let ast = parser.parse();
    match &ast {
        Ok(prog) => eprintln!("[test debug] AST: {prog:#?}"),
        Err(e) => eprintln!("[test debug] Parse error: {e}"),
    }
    ast.map_err(Into::into)
}

fn run_tests(pattern: Option<String>, verbose: bool, continue_on_failure: bool) -> Result<()> {
    use std::env;
    println!("🧪 Running Cyl automated tests...\n");
    println!(
        "[debug] Current working directory: {}",
        env::current_dir()?.display()
    );

    let tests_dir = PathBuf::from("tests");
    if !tests_dir.exists() {
        anyhow::bail!("Tests directory not found. Expected 'tests/' directory in project root.");
    }

    let fixtures_dir = tests_dir.join("fixtures");
    if !fixtures_dir.exists() {
        anyhow::bail!("Test fixtures directory not found. Expected 'tests/fixtures/' directory.");
    }

    let valid_dir = fixtures_dir.join("valid");
    let invalid_dir = fixtures_dir.join("invalid");

    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    // Run valid tests (should parse successfully)
    if valid_dir.exists() {
        println!("📂 Running valid test cases...");
        let (total, passed, failed) = run_test_directory(
            &valid_dir,
            true,
            pattern.as_deref(),
            verbose,
            continue_on_failure,
        )?;
        total_tests += total;
        passed_tests += passed;
        failed_tests += failed;
        println!();
    }

    // Run invalid tests (should fail to parse)
    if invalid_dir.exists() {
        println!("📂 Running invalid test cases...");
        let (total, passed, failed) = run_test_directory(
            &invalid_dir,
            false,
            pattern.as_deref(),
            verbose,
            continue_on_failure,
        )?;
        total_tests += total;
        passed_tests += passed;
        failed_tests += failed;
        println!();
    }

    // Print summary
    println!("📊 Test Summary:");
    println!("   Total:  {total_tests}");
    println!("   Passed: {passed_tests} ✅");
    println!("   Failed: {failed_tests} ❌");

    if failed_tests > 0 {
        println!("\n❌ Some tests failed");
        if !continue_on_failure {
            std::process::exit(1);
        }
    } else {
        println!("\n✅ All tests passed!");
    }

    Ok(())
}

fn run_test_directory(
    dir: &PathBuf,
    should_succeed: bool,
    pattern: Option<&str>,
    verbose: bool,
    continue_on_failure: bool,
) -> Result<(usize, usize, usize)> {
    use std::fs;

    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;

    let entries = fs::read_dir(dir)?;
    let mut test_files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.extension().is_some_and(|ext| ext == "cyl")
        })
        .collect();

    // Sort for consistent test ordering
    test_files.sort_by_key(|entry| entry.file_name());

    for entry in test_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();

        // Filter by pattern if provided
        if let Some(pattern) = pattern {
            if !filename.contains(pattern) {
                continue;
            }
        }

        total += 1;

        let test_result = run_single_test(&path, should_succeed, verbose);

        match test_result {
            Ok(true) => {
                passed += 1;
                if verbose {
                    println!("  ✅ {filename}");
                }
            }
            Ok(false) => {
                failed += 1;
                println!("  ❌ {filename}");
                if !continue_on_failure {
                    break;
                }
            }
            Err(e) => {
                failed += 1;
                println!("  ❌ {filename} (error: {e})");
                if !continue_on_failure {
                    break;
                }
            }
        }
    }

    if !verbose {
        println!("  Ran {total} tests: {passed} passed, {failed} failed");
    }

    Ok((total, passed, failed))
}

fn run_single_test(file: &PathBuf, should_succeed: bool, verbose: bool) -> Result<bool> {
    // Read source file
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            if verbose {
                println!("    Failed to read file: {e}");
            }
            return Ok(false);
        }
    };

    // Try to parse the file
    let parse_result = try_parse_file(&source);

    match (should_succeed, parse_result) {
        (true, Ok(_)) => {
            if verbose {
                println!("    Parsed successfully (as expected)");
            }
            Ok(true)
        }
        (true, Err(e)) => {
            if verbose {
                println!("    Parse failed unexpectedly: {e}");
            }
            Ok(false)
        }
        (false, Err(_)) => {
            if verbose {
                println!("    Parse failed (as expected)");
            }
            Ok(true)
        }
        (false, Ok(_)) => {
            if verbose {
                println!("    Parse succeeded unexpectedly");
            }
            Ok(false)
        }
    }
}
