Several useful tips:
 - Install [rust](https://www.rust-lang.org/learn/get-started) and [cargo make](https://github.com/sagiegurari/cargo-make).
 - Run `cargo make` to compile the project and run all tests. This will also automatically format the code 
   according to Rust specification, so I recommend running it before every commit.
 - Also, you can run `cargo clippy` to run static analysis of your code - this will produce extra warnings 
   that are normally not provided by the compiler.
 - Run `cargo run --release [model file] [phenotype file]` to run the `src/main.rs` binary. The binary will load 
   the given model file and compute its attractors. Then, it will parse the phenotypes (one line = one specification) 
   and run analysis for each attractor-phenotype pair using the library functions.
 - Library root file is in `src/lib.rs`.
 - Algorithms adapted from Aeon are in `src/aeon/` - there shouldn't be a need to modify those.
 - There are Github Actions specified for this project that will run all checks in the cloud as well.
 - You can find a lot of different `.aeon` models in this 
   [benchmark folder](https://github.com/sybila/biodivine-aeon-server/tree/master/benchmark). The models
   do not have parameters (so there will always be only one color per attractor), but that should be good
   enough for testing now.