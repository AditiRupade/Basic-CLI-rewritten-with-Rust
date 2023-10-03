// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_touch_and_rm_command() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("touch")
        .arg("newfile.txt")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    let mut stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Touched: newfile.txt").eval(&stdout));
    
    let mut stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());

    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("rm")
        .arg("newfile.txt")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Removed: newfile.txt").eval(&stdout));
    
    stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}


#[test]
fn test_touch_command_fails_when_file_attribute_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("touch")
        .output()
        .expect("failed to run command");
    
    assert_eq!(false,output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,contains("Usage: target\\release\\touch.exe <file>...").eval(&stderr));
}



#[test]
fn test_rm_command_fails_when_file_attribute_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("rm")
        .output()
        .expect("failed to run command");
    
    assert_eq!(false,output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,contains("Usage: target\\release\\rm.exe <file_or_directory>...").eval(&stderr));
}
