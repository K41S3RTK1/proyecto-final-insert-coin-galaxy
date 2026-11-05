#!/bin/zsh
set -e
cd "${0:A:h}"
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release --locked
exec target/release/arcade-galaxy --software --quality fast
