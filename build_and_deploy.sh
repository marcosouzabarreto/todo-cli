#!/bin/bash

# Export Rust-related environment variables
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"

# Set the default Rust toolchain (optional)
rustup default stable

# Build the Rust project
cargo build --release

# Move the executable to the bin folder
mv target/release/todo-list /usr/local/bin/todo
