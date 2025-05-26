#!/bin/bash
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=i686-unknown-linux-gnu

# for target_arch in $(rustup target list | egrep "^[a-z0-9_-]+" -o); do
#   cargo build --target=$target_arch
# done
