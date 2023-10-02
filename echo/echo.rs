use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Join all command line arguments (excluding the first one, which is the program name)
    let message = args[1..].join(" ");
    println!("{}", message);
}
