// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_cat_command_should_read_file() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("cat")
        .arg("test.txt")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Line 1\nLine 2\n").eval(&stdout));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}


#[test]
fn test_cat_command_fails_when_file_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("cat")
        .arg("test_file_invalid.txt")
        .output()
        .expect("failed to run command");

    assert_eq!(false,output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(false,stderr.is_empty());
}
