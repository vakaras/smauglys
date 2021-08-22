#!/bin/bash

set -e

cd wrapper
cargo build --release
cd ..

cp wrapper/target/release/wrapper.exe .

cd installer
cargo build --release
cp target/release/installer.exe ../smauglys.exe
cd ..

# Tell Windows that we need administrator priviliges.
ls -lh ./tools/mt.exe
chmod 755 ./tools/mt.exe
./tools/mt.exe -manifest "smauglys.exe.manifest" '-outputresource:"smauglys.exe";1'
