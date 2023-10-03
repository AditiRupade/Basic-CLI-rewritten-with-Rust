use std::env;
// use std::path::Path;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a directory as an argument
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    // Parse the command-line argument as the directory path
    let new_dir = &args[1];

    // Attempt to change the current working directory
    if let Err(err) = env::set_current_dir(new_dir) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    // Print the new current working directory
    if let Ok(new_cwd) = env::current_dir() {
        println!("Changed directory to: {}", new_cwd.display());
    } else {
        eprintln!("Unable to determine the current directory.");
    }
}
