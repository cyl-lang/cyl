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

    // Set up environment for subprocess
    let python_lib = "/opt/homebrew/opt/python@3.11/lib";
    let python_bin = "/opt/homebrew/bin/python3.11";
    let mut cmd = Command::new(cylc_binary);
    cmd.arg("run")
        .arg("--backend")
        .arg(backend)
        .arg("--quiet")
        .arg(&format!("../{}", cyl_file_path));

    // Set env vars for Homebrew Python
    cmd.env("DYLD_LIBRARY_PATH", python_lib)
        .env("LIBRARY_PATH", python_lib)
        .env("PYO3_PYTHON", python_bin);

    // Print debug info about the command and environment
    println!("[debug] About to run: {:?}", cmd);
    let env_map: std::collections::HashMap<_, _> = cmd.get_envs().filter_map(|(k, v)| {
        Some((k.to_string_lossy().to_string(), v?.to_string_lossy().to_string()))
    }).collect();
    println!("[debug] Subprocess ENV: {:?}", env_map);

    let run_output = match cmd.output() {
        Ok(output) => output,
        Err(e) => {
            println!("[debug] Failed to launch subprocess: {}", e);
            return Ok(CylTestResult {
                exit_code: -1,
                stdout: String::new(),
                stderr: format!("Failed to launch subprocess: {}", e),
                compiled: false,
            });
        }
    };

    if !run_output.status.success() {
        println!("[debug] Subprocess failed: status={:?}", run_output.status);
        println!("[debug] Subprocess STDOUT: {}", String::from_utf8_lossy(&run_output.stdout));
        println!("[debug] Subprocess STDERR: {}", String::from_utf8_lossy(&run_output.stderr));
    }

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

    // Print environment variables at the start of the test suite
    #[test]
    fn print_env_debug() {
        println!("[debug] ENV: {:?}", std::env::vars().collect::<Vec<_>>());
    }

    // Set DYLD_LIBRARY_PATH at runtime for all tests
    fn ensure_dyld_library_path() {
        const PYTHON_LIB: &str = "/opt/homebrew/opt/python@3.11/lib";
        let var = std::env::var("DYLD_LIBRARY_PATH").unwrap_or_default();
        if !var.contains(PYTHON_LIB) {
            let new_val = if var.is_empty() {
                PYTHON_LIB.to_string()
            } else {
                format!("{}:{}", PYTHON_LIB, var)
            };
            std::env::set_var("DYLD_LIBRARY_PATH", new_val);
        }
    }


    #[test]
    fn test_hello_world() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("examples/hello_world.cyl", "interpreter")
            .expect("Failed to run hello_world.cyl");
        assert!(result.success(), "Hello world test should succeed: {:?}", result);
        let expected = "Hello, World!\nWelcome to Cyl programming language!";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_print_functionality() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("examples/print_test.cyl", "interpreter")
            .expect("Failed to run print_test.cyl");
        assert!(result.success(), "Print test should succeed: {:?}", result);
        let expected = "Hello, World!\n42";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_arithmetic() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file("tests/fixtures/valid/arithmetic_test.cyl")
            .expect("Failed to run arithmetic_test.cyl");
        assert!(result.success(), "Arithmetic test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have arithmetic output");
    }

    #[test]
    fn test_variables() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/variables_test.cyl", "interpreter")
            .expect("Failed to run variables_test.cyl");
        assert!(result.success(), "Variables test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have variable output");
    }

    #[test]
    fn test_simple_if() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/simple_if_test.cyl", "interpreter")
            .expect("Failed to run simple_if_test.cyl");
        assert!(result.success(), "Simple if test should succeed: {:?}", result);
        assert!(!result.stdout.trim().is_empty(), "Should have if output");
    }

    #[test]
    fn test_struct_member_access() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("examples/struct_test.cyl", "interpreter")
            .expect("Failed to run struct_test.cyl");
        assert!(result.success(), "Struct test should succeed: {:?}", result);
        // No output expected, just check for success
    }

    #[test]
    fn test_array_indexing() {
        ensure_dyld_library_path();
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
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("examples/array_for_loop_combined.cyl", "interpreter")
            .expect("Failed to run array_for_loop_combined.cyl");
        assert!(result.success(), "Array for loop test should succeed: {:?}", result);
        let expected = "10\n20\n30\n40\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_comprehensive_array_fixture() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/array_comprehensive_test.cyl", "interpreter")
            .expect("Failed to run array_comprehensive_test.cyl");
        assert!(result.success(), "Comprehensive array test should succeed: {:?}", result);
        let expected = "100\n200\n500";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_array_arithmetic_fixture() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/array_arithmetic_test.cyl", "interpreter")
            .expect("Failed to run array_arithmetic_test.cyl");
        assert!(result.success(), "Array arithmetic test should succeed: {:?}", result);
        let expected = "30\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_while_countdown_fixture() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/while_countdown_test.cyl", "interpreter")
            .expect("Failed to run while_countdown_test.cyl");
        assert!(result.success(), "While countdown test should succeed: {:?}", result);
        let expected = "3\n2\n1\n999";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_while_loop_fixture() {
        ensure_dyld_library_path();
        let result = compile_and_run_cyl_file_with_backend("tests/fixtures/valid/while_loop_test.cyl", "interpreter")
            .expect("Failed to run while_loop_test.cyl");
        assert!(result.success(), "While loop test should succeed: {:?}", result);
        let expected = "999";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_all_valid_fixtures() {
        ensure_dyld_library_path();
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
        ensure_dyld_library_path();
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
