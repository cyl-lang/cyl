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

/// Test a Cyl program and expect it to succeed with specific output
pub fn test_cyl_program_output(cyl_file_path: &str, expected_output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_and_run_cyl_file(cyl_file_path)?;
    
    if !result.success() {
        if result.compilation_failed() {
            panic!(
                "Compilation failed for {}: {}\n{}",
                cyl_file_path, result.stderr, result.stdout
            );
        } else {
            panic!(
                "Runtime failed for {} (exit code {}): {}\n{}",
                cyl_file_path, result.exit_code, result.stderr, result.stdout
            );
        }
    }

    let actual_output = result.stdout.trim();
    let expected_output = expected_output.trim();

    if actual_output != expected_output {
        panic!(
            "Output mismatch for {}:\nExpected:\n{}\nActual:\n{}",
            cyl_file_path, expected_output, actual_output
        );
    }

    Ok(())
}

/// Test that a Cyl program compiles successfully but don't check output
pub fn test_cyl_program_compiles(cyl_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_and_run_cyl_file(cyl_file_path)?;
    
    if result.compilation_failed() {
        panic!(
            "Compilation failed for {}: {}\n{}",
            cyl_file_path, result.stderr, result.stdout
        );
    }

    Ok(())
}

/// Test that a Cyl program fails to compile (for negative testing)
pub fn test_cyl_program_compilation_fails(cyl_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_and_run_cyl_file(cyl_file_path)?;
    
    if result.compiled {
        panic!(
            "Expected compilation to fail for {}, but it succeeded",
            cyl_file_path
        );
    }

    Ok(())
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

/// Test all .cyl files in the valid fixtures directory
pub fn test_all_valid_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    let valid_dir = "tests/fixtures/valid";
    let cyl_files = discover_cyl_files(valid_dir)?;
    
    println!("Testing {} valid Cyl files...", cyl_files.len());
    
    for cyl_file in cyl_files {
        // Skip disabled files
        if cyl_file.ends_with(".disabled") {
            println!("Skipping disabled file: {}", cyl_file);
            continue;
        }
        
        println!("Testing: {}", cyl_file);
        
        // Check if file has expected output annotation
        if let Some(expected_output) = extract_expected_output(&cyl_file)? {
            test_cyl_program_output(&cyl_file, &expected_output)?;
        } else {
            // Just test that it compiles and runs successfully
            test_cyl_program_compiles(&cyl_file)?;
        }
    }
    
    Ok(())
}

/// Test all .cyl files in the invalid fixtures directory (should fail to compile)
pub fn test_all_invalid_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    let invalid_dir = "tests/fixtures/invalid";
    let cyl_files = discover_cyl_files(invalid_dir)?;
    
    println!("Testing {} invalid Cyl files (expecting compilation failures)...", cyl_files.len());
    
    for cyl_file in cyl_files {
        println!("Testing (expecting failure): {}", cyl_file);
        test_cyl_program_compilation_fails(&cyl_file)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        test_cyl_program_output(
            "examples/hello_world.cyl",
            "\"Hello, World!\"\n\"Welcome to Cyl programming language!\""
        ).expect("Hello world test failed");
    }

    #[test]
    fn test_print_functionality() {
        test_cyl_program_output(
            "examples/print_test.cyl",
            "\"Hello, World!\"\n42"
        ).expect("Print test failed");
    }

    #[test]
    fn test_struct_compilation() {
        test_cyl_program_compiles("examples/struct_test.cyl")
            .expect("Struct test compilation failed");
    }

    #[test]
    fn test_array_compilation() {
        test_cyl_program_compiles("examples/array_test.cyl")
            .expect("Array test compilation failed");
    }

    #[test]
    fn test_file_processing_compilation() {
        test_cyl_program_compiles("examples/file_processing.cyl")
            .expect("File processing test compilation failed");
    }

    // Comprehensive fixture tests
    #[test]
    fn test_all_valid_test_fixtures() {
        test_all_valid_fixtures()
            .expect("Valid fixture tests failed");
    }

    #[test]
    fn test_all_invalid_test_fixtures() {
        test_all_invalid_fixtures()
            .expect("Invalid fixture tests failed");
    }
}
