// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_cat_command() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("pwd")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Getting-started-with-Rust").eval(&stdout));
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}

