# Daniel Joffe Submission

This project contains the code for different approaches to the dinner-seating problem.

It is required to have an installation of Rust 1.52.1 and Python 3.7. Later versions of Rust should work, and later versions of Python will hopefully work.

In `./py/` you'll find the source for the integer programming solver, and in `./src/` you'll find the source code for all the Rust code used in this project (everything that's not IP). The source code for Rust executables is in `./src/bin`. `Cargo.toml` and `Cargo.lock` are used for the Rust build system. Inside `target` is all the intermediate steps for Rust compilation, with the executables in `./target/release/`. I don't know what OS and architecture you're using, so then contents are likely useless, but the manual for the course said to include executables, so they're there anyway. When you compile the Rust code with `cargo build --release`, you will likely get a deprecation warning; it is safe to ignore.


In `/weddings` you'll find the schematics for creating suites of test data, the test data itself (inside directories with the same name as the schematics), and the results of different solvers on that test data.
