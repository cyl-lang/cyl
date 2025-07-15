use std::fs;
use std::path::Path;
use std::process::Command;

/// Integration test framework for Cyl language
/// This module provides utilities to compile and execute Cyl programs,
/// then verify their output and behavior automatically.

#[derive(Debug)]
pub struct CylTestResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub compiled: bool,
}

impl CylTestResult {
    pub fn success(&self) -> bool {
        self.compiled && self.exit_code == 0
    }

    pub fn compilation_failed(&self) -> bool {
        !self.compiled
    }

    pub fn runtime_failed(&self) -> bool {
        self.compiled && self.exit_code != 0
    }
}

/// Compile and run a Cyl program, returning the result
pub fn compile_and_run_cyl_file(cyl_file_path: &str) -> Result<CylTestResult, Box<dyn std::error::Error>> {
    compile_and_run_cyl_file_with_backend(cyl_file_path, "interpreter")
}

/// Compile and run a Cyl program with a specific backend, returning the result
pub fn compile_and_run_cyl_file_with_backend(cyl_file_path: &str, backend: &str) -> Result<CylTestResult, Box<dyn std::error::Error>> {
    // Try to find the cylc binary - check both debug and release directories
    let possible_binaries = [
        "../target/release/cylc",
        "../target/debug/cylc", 
        "./target/release/cylc",
        "./target/debug/cylc",
    ];
    
    let mut cylc_binary = None;
    for binary_path in &possible_binaries {
        if Path::new(binary_path).exists() {
            cylc_binary = Some(*binary_path);
            break;
        }
    }
    
    let cylc_binary = cylc_binary.ok_or("Could not find cylc binary in target/debug or target/release")?;

    // Use the 'run' command which compiles and interprets the code
    let run_output = Command::new(cylc_binary)
        .arg("run")
        .arg("--backend")
        .arg(backend)
        .arg("--quiet")
        .arg(&format!("../{}", cyl_file_path))
        .output()?;

    Ok(CylTestResult {
        exit_code: run_output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
        compiled: run_output.status.success(),
    })
}

/// Discover all .cyl files in a directory (non-recursive)
fn discover_cyl_files(dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cyl_files = Vec::new();
    
    let entries = fs::read_dir(&format!("../{}", dir_path))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "cyl" {
                    if let Some(path_str) = path.to_str() {
                        // Remove the "../" prefix to get the original relative path
                        let relative_path = path_str.strip_prefix("../").unwrap_or(path_str);
                        cyl_files.push(relative_path.to_string());
                    }
                }
            }
        }
    }
    
    cyl_files.sort();
    Ok(cyl_files)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_hello_world() {
        let result = compile_and_run_cyl_file_with_backend("examples/hello_world.cyl", "interpreter")
            .expect("Failed to run hello_world.cyl");
        assert!(result.success(), "Hello world test should succeed: {:?}", result);
        let expected = "Hello, World!\nWelcome to Cyl programming language!";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_print_functionality() {
        let result = compile_and_run_cyl_file_with_backend("examples/print_test.cyl", "interpreter")
            .expect("Failed to run print_test.cyl");
        assert!(result.success(), "Print test should succeed: {:?}", result);
        let expected = "Hello, World!\n42";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_arithmetic() {
        let result = compile_and_run_cyl_file("tests/fixtures/valid/arithmetic_test.cyl")
            .expect("Failed to run arithmetic_test.cyl");
        assert!(result.success(), "Arithmetic test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have arithmetic output");
    }

    #[test]
    fn test_variables() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/variables_test.cyl", "interpreter")
            .expect("Failed to run variables_test.cyl");
        assert!(result.success(), "Variables test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have variable output");
    }

    #[test]
    fn test_simple_if() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/simple_if_test.cyl", "interpreter")
            .expect("Failed to run simple_if_test.cyl");
        assert!(result.success(), "Simple if test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have if output");
    }

    #[test]
    fn test_struct_member_access() {
        let result = compile_and_run_cyl_file_with_backend("examples/struct_test.cyl", "interpreter")
            .expect("Failed to run struct_test.cyl");
        assert!(result.success(), "Struct test should succeed: {:?}", result);
        // No output expected, just check for success
    }

    #[test]
    fn test_array_indexing() {
        let result = compile_and_run_cyl_file_with_backend("examples/array_test.cyl", "interpreter")
            .expect("Failed to run array_test.cyl");
        if !result.success() || result.stdout.trim() != "10\n30\n50" {
            println!("[debug] STDOUT:\n{}", result.stdout);
            println!("[debug] STDERR:\n{}", result.stderr);
        }
        assert!(result.success(), "Array test should succeed: {:?}", result);
        let expected = "10\n30\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_array_for_loop() {
        let result = compile_and_run_cyl_file_with_backend("examples/array_for_loop_combined.cyl", "interpreter")
            .expect("Failed to run array_for_loop_combined.cyl");
        assert!(result.success(), "Array for loop test should succeed: {:?}", result);
        let expected = "10\n20\n30\n40\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_comprehensive_array_fixture() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/array_comprehensive_test.cyl", "interpreter")
            .expect("Failed to run array_comprehensive_test.cyl");
        assert!(result.success(), "Comprehensive array test should succeed: {:?}", result);
        let expected = "100\n200\n500";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_array_arithmetic_fixture() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/array_arithmetic_test.cyl", "interpreter")
            .expect("Failed to run array_arithmetic_test.cyl");
        assert!(result.success(), "Array arithmetic test should succeed: {:?}", result);
        let expected = "30\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_while_countdown_fixture() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/while_countdown_test.cyl", "interpreter")
            .expect("Failed to run while_countdown_test.cyl");
        assert!(result.success(), "While countdown test should succeed: {:?}", result);
        let expected = "3\n2\n1\n999";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_while_loop_fixture() {
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/while_loop_test.cyl", "interpreter")
            .expect("Failed to run while_loop_test.cyl");
        assert!(result.success(), "While loop test should succeed: {:?}", result);
        let expected = "999";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_all_valid_fixtures() {
        let valid_dir = "tests/fixtures/valid";
        let cyl_files = discover_cyl_files(valid_dir)
            .expect("Failed to discover valid test files");
        
        println!("Testing {} valid Cyl files...", cyl_files.len());
        
        let mut tested_files = 0;
        for cyl_file in cyl_files {
            // Skip disabled files
            if cyl_file.ends_with(".disabled") {
                println!("Skipping disabled file: {}", cyl_file);
                continue;
            }
            
            println!("Testing: {}", cyl_file);
            
            let result = compile_and_run_cyl_file(&cyl_file)
                .expect(&format!("Failed to run {}", cyl_file));
            
            assert!(result.success(), "Test failed for {}: {:?}", cyl_file, result);
            tested_files += 1;
        }
        
        assert!(tested_files > 0, "No valid test files were executed");
        println!("Successfully tested {} valid files", tested_files);
    }

    #[test]
    fn test_all_invalid_fixtures() {
        let invalid_dir = "tests/fixtures/invalid";
        let cyl_files = discover_cyl_files(invalid_dir)
            .expect("Failed to discover invalid test files");
        
        println!("Testing {} invalid Cyl files (expecting compilation failures)...", cyl_files.len());
        
        let mut tested_files = 0;
        for cyl_file in cyl_files {
            println!("Testing (expecting failure): {}", cyl_file);
            
            let result = compile_and_run_cyl_file(&cyl_file)
                .expect(&format!("Failed to run {}", cyl_file));
            
            assert!(result.compilation_failed(), 
                "Expected compilation to fail for {}, but it succeeded: {:?}", 
                cyl_file, result);
            tested_files += 1;
        }
        
        assert!(tested_files > 0, "No invalid test files were executed");
        println!("Successfully tested {} invalid files", tested_files);
    }
}
