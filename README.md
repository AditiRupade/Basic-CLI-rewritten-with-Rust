# Basic CLI commands

I have written basic cli commands like echo, touch, grep, etc. using Rust. 

Usage-
Each folder represents cli command. Inside each folder, there is an src file with .rs extension which contains code for the building the command. You can compile this file to genearte an executable .exe file. You can run the commands directly by running .exe file.

For example-
echo folder has echo.rs source file. For Windows, you can compile this using
```
rustc echo.rs
```
An executable will be generated after this as echo.rs, which can be run to use the command
```
.\echo Hello,World!
```
