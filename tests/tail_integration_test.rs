// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_tail_command() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("tail")
        .arg("myfile.txt")
        .arg("2")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Getting started with Rust\nBye\n").eval(&stdout));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}


#[test]
fn test_tail_command_fails_when_filename_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("tail")
        .output()
        .expect("failed to run command");
    
    assert_eq!(false,output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,contains("Usage: target\\release\\tail.exe <filename> <num_lines>").eval(&stderr));
}



#[test]
fn test_cat_command_fails_when_num_lines_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("tail")
        .arg("myfile.txt")
        .output()
        .expect("failed to run command");
    
    assert_eq!(false,output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,contains("Usage: target\\release\\tail.exe <filename> <num_lines>").eval(&stderr));
}
