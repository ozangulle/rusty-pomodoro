#!/bin/bash
if [ "$TARGET" == "x86_64-pc-windows-gnud" ]
then
    FILENAME=$NAME.exe
fi
# Install Rust stdlib for the target
rustup target add $TARGET

# Compile the binary for the target
cargo build --target=$TARGET --release

# Package the release binary
echo $FILENAME
tar -C target/$TARGET/release -czf $PACKAGE $FILENAME