Create cargo project: `cargo new project_name` <br>
Build cargo project: `cargo build`

Build can be found under /target/debug/appname.exe

Run cargo build: `cargo run`

To check if the program can compile, it is 
easiest to make use of `cargo check` as it 
does not make an executable.

If you're ready to release your software,
build it with the --release argument. This optimizes 
the compilation.

Generally you only want to use rustc if you 
do not have any dependencies or have a really small program. Otherwise cargo is much much much more convenient.