#!/bin/zsh
set -e
cd "${0:A:h}"
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release --locked
./tools/package-macos.sh
open "target/Insert Coin Galaxy.app"
