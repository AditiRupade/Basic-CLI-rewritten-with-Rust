// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_echo_command() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("echo")
        .arg("Echo Hello World")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Echo Hello World").eval(&stdout));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}


#[test]
fn test_echo_command_when_no_argument_passed() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("echo")
        .output()
        .expect("failed to run command");

    assert_eq!(true,output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, "\n");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}
