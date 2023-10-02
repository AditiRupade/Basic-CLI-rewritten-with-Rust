use std::env;

fn main() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        eprintln!("Error: Unable to determine the current directory.");
        std::process::exit(1);
    }
}
