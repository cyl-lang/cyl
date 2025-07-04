use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod lexer;
mod parser;
mod ast;
mod codegen;
mod error;
mod stdlib;

use crate::lexer::Lexer;
use crate::parser::Parser as CylParser;
use crate::codegen::CodeGenerator;

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
        Commands::Run { file, opt_level, debug } => {
            compile_and_run(&file, opt_level, debug)?;
        }
        Commands::Build { file, output, opt_level, debug } => {
            compile_to_executable(&file, output, opt_level, debug)?;
        }
        Commands::Check { file } => {
            check_syntax(&file)?;
        }
        Commands::Ast { file, format } => {
            show_ast(&file, &format)?;
        }
        Commands::Test { pattern, verbose, continue_on_failure } => {
            run_tests(pattern, verbose, continue_on_failure)?;
        }
    }

    Ok(())
}

fn compile_and_run(file: &PathBuf, opt_level: u8, debug: bool) -> Result<()> {
    println!("Compiling and running: {}", file.display());
    
    // Read source file
    let source = std::fs::read_to_string(file)?;
    
    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    // Parsing
    let mut parser = CylParser::new(tokens);
    let ast = parser.parse()?;
    
    // Code generation
    let mut codegen = CodeGenerator::new(opt_level, debug);
    let executable = codegen.compile_to_executable(&ast)?;
    
    // Execute
    executable.run()?;
    
    Ok(())
}

fn compile_to_executable(
    file: &PathBuf, 
    output: Option<PathBuf>, 
    opt_level: u8, 
    debug: bool
) -> Result<()> {
    let output_name = output.unwrap_or_else(|| {
        file.with_extension("")
    });
    
    println!("Compiling {} to {}", file.display(), output_name.display());
    
    // Read source file
    let source = std::fs::read_to_string(file)?;
    
    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    // Parsing
    let mut parser = CylParser::new(tokens);
    let ast = parser.parse()?;
    
    // Code generation
    let mut codegen = CodeGenerator::new(opt_level, debug);
    codegen.compile_to_file(&ast, &output_name)?;
    
    println!("Successfully compiled to {}", output_name.display());
    
    Ok(())
}

fn check_syntax(file: &PathBuf) -> Result<()> {
    println!("Checking syntax: {}", file.display());
    
    // Read source file
    let source = std::fs::read_to_string(file)?;
    
    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    // Parsing (syntax check)
    let mut parser = CylParser::new(tokens);
    let _ast = parser.parse()?;
    
    println!("✓ Syntax is valid");
    
    Ok(())
}

fn show_ast(file: &PathBuf, format: &str) -> Result<()> {
    // Read source file
    let source = std::fs::read_to_string(file)?;
    
    // Lexical analysis
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    // Parsing
    let mut parser = CylParser::new(tokens);
    let ast = parser.parse()?;
    
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&ast)?;
            println!("{}", json);
        }
        "pretty" => {
            println!("{:#?}", ast);
        }
        _ => {
            anyhow::bail!("Unknown format: {}. Use 'json' or 'pretty'", format);
        }
    }
    
    Ok(())
}

fn run_tests(pattern: Option<String>, verbose: bool, continue_on_failure: bool) -> Result<()> {
    use std::fs;
    
    println!("🧪 Running Cyl automated tests...\n");
    
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
        let (total, passed, failed) = run_test_directory(&valid_dir, true, pattern.as_deref(), verbose, continue_on_failure)?;
        total_tests += total;
        passed_tests += passed;
        failed_tests += failed;
        println!();
    }
    
    // Run invalid tests (should fail to parse)
    if invalid_dir.exists() {
        println!("📂 Running invalid test cases...");
        let (total, passed, failed) = run_test_directory(&invalid_dir, false, pattern.as_deref(), verbose, continue_on_failure)?;
        total_tests += total;
        passed_tests += passed;
        failed_tests += failed;
        println!();
    }
    
    // Print summary
    println!("📊 Test Summary:");
    println!("   Total:  {}", total_tests);
    println!("   Passed: {} ✅", passed_tests);
    println!("   Failed: {} ❌", failed_tests);
    
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
    continue_on_failure: bool
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
            path.extension().map_or(false, |ext| ext == "cyl")
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
                    println!("  ✅ {}", filename);
                }
            }
            Ok(false) => {
                failed += 1;
                println!("  ❌ {}", filename);
                if !continue_on_failure {
                    break;
                }
            }
            Err(e) => {
                failed += 1;
                println!("  ❌ {} (error: {})", filename, e);
                if !continue_on_failure {
                    break;
                }
            }
        }
    }
    
    if !verbose {
        println!("  Ran {} tests: {} passed, {} failed", total, passed, failed);
    }
    
    Ok((total, passed, failed))
}

fn run_single_test(file: &PathBuf, should_succeed: bool, verbose: bool) -> Result<bool> {
    // Read source file
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            if verbose {
                println!("    Failed to read file: {}", e);
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
                println!("    Parse failed unexpectedly: {}", e);
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

fn try_parse_file(source: &str) -> Result<crate::ast::Program> {
    // Lexical analysis
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    // Parsing
    let mut parser = CylParser::new(tokens);
    let ast = parser.parse()?;
    
    Ok(ast)
}
