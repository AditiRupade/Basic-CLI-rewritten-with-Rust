// tests/integration_test.rs

use std::process::Command;
use predicates::str::contains;
use predicates::Predicate;

#[test]
fn test_mkdir_and_rm_command() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("mkdir")
        .arg("newDir")
        .output()
        .expect("failed to run command");

    assert_eq!(true,output.status.success());

    let mut stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("Directory 'newDir' created successfully.").eval(&stdout));
    
    let mut stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());

    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("rm")
        .arg("newDir")
        .output()
        .expect("failed to run command");

    assert!(output.status.success());

    stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,contains("newDir").eval(&stdout));
    
    stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(true,stderr.is_empty());
}


#[test]
fn test_mkdir_command_fails_when_directory_name_attribute_not_present() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("-r")
        .arg("-q")
        .arg("--bin")
        .arg("mkdir")
        .output()
        .expect("failed to run command");
    
    assert_eq!(false,output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(true,stdout.is_empty());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{}", &stderr);
    assert_eq!(true,contains("Usage: target\\release\\mkdir.exe <directory_name>").eval(&stderr));
}

