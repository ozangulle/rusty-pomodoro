#!/bin/bash

# Install Rust stdlib for the target
# rustup target add $TARGET

# Compile the binary for the target
cargo build --target=$TARGET --release

# Package the release binary
tar -czf $PACKAGE -C target/$TARGET/release/ $NAME