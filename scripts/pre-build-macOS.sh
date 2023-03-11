#!/bin/sh

cargo build --release --target aarch64-apple-darwin
cbindgen --config cbindgen.toml --crate clock --output include/clock.h

cp target/aarch64-apple-darwin/release/libclock.a macOS/Clock/Frameworks/libclock.a