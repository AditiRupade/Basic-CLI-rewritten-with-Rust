# Basic CLI commands

This repository contains basic cli commands like echo, touch, grep, etc. rewritten using Rust. 

Usage-
The src/bin folder contains cli command. Inside that folder, there is an src file with .rs extension which contains code for the building the command. You can compile this file to genearte an executable .exe file. You can run the commands directly by running .exe file.

For example-
echo folder has echo.rs source file. For Windows, you can compile this using
```
rustc echo.rs
```
An executable will be generated after this as echo.rs, which can be run to use the command
```
.\echo Hello,World!
```

To make things easy, Rust provides cargo to build and run everything in a go. To build all the files, use the following command-
```
cargo build --bins --release
```

This repository also contains unit + integration tests to verify the functionality of the code. To run all the tests using cargo, use the following command-
```
cargo test
```

In case if you want to run each test separately, use the following command-
```
cargo run -r -q --bin <executable_file> [arguments]
```