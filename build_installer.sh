#!/bin/bash

set -e

cd wrapper
cargo build --release
cd ..

cp wrapper/target/release/wrapper.exe .

cd installer
cargo build --release
cd ..
