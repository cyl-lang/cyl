use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

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
    let temp_dir = TempDir::new()?;
    let cyl_path = Path::new(cyl_file_path);
    let executable_name = cyl_path.file_stem().unwrap().to_str().unwrap();
    let executable_path = temp_dir.path().join(executable_name);

    // Step 1: Compile the Cyl file
    let compile_output = Command::new("./target/release/cylc")
        .arg("build")
        .arg(cyl_file_path)
        .arg("-o")
        .arg(&executable_path)
        .output()?;

    if !compile_output.status.success() {
        return Ok(CylTestResult {
            exit_code: compile_output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&compile_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
            compiled: false,
        });
    }

    // Step 2: Execute the compiled binary
    let run_output = Command::new(&executable_path)
        .output()?;

    Ok(CylTestResult {
        exit_code: run_output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&run_output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&run_output.stderr).to_string(),
        compiled: true,
    })
}

/// Extract expected output from a Cyl file's comments
/// Looks for comments like: // EXPECTED_OUTPUT: "Hello, World!"
/// or // EXPECTED_OUTPUT: 42
fn extract_expected_output(cyl_file_path: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(cyl_file_path)?;
    
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("// EXPECTED_OUTPUT:") {
            let output = line.trim_start_matches("// EXPECTED_OUTPUT:").trim();
            return Ok(Some(output.to_string()));
        }
    }
    
    Ok(None)
}

/// Discover all .cyl files in a directory (non-recursive)
fn discover_cyl_files(dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cyl_files = Vec::new();
    
    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "cyl" {
                    if let Some(path_str) = path.to_str() {
                        cyl_files.push(path_str.to_string());
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
        let result = compile_and_run_cyl_file("examples/hello_world.cyl")
            .expect("Failed to run hello_world.cyl");
        
        assert!(result.success(), "Hello world test should succeed: {:?}", result);
        
        let expected = "\"Hello, World!\"\n\"Welcome to Cyl programming language!\"";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_print_functionality() {
        let result = compile_and_run_cyl_file("examples/print_test.cyl")
            .expect("Failed to run print_test.cyl");
        
        assert!(result.success(), "Print test should succeed: {:?}", result);
        
        let expected = "\"Hello, World!\"\n42";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_arithmetic() {
        let result = compile_and_run_cyl_file("tests/fixtures/valid/arithmetic_test.cyl")
            .expect("Failed to run arithmetic_test.cyl");
        
        assert!(result.success(), "Arithmetic test should succeed: {:?}", result);
        
        let expected = "15\n5\n50";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_variables() {
        let result = compile_and_run_cyl_file("tests/fixtures/valid/variables_test.cyl")
            .expect("Failed to run variables_test.cyl");
        
        assert!(result.success(), "Variables test should succeed: {:?}", result);
        
        let expected = "\"Cyl\"\n3";
        assert_eq!(result.stdout.trim(), expected);
    }

    #[test]
    fn test_control_flow() {
        let result = compile_and_run_cyl_file("tests/fixtures/valid/control_flow_test.cyl")
            .expect("Failed to run control_flow_test.cyl");
        
        assert!(result.success(), "Control flow test should succeed: {:?}", result);
        
        let expected = "\"x is greater\"\n\"x equals 10\"";
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
