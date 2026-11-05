#!/bin/zsh
set -e
cd "${0:A:h}/.."
command -v ffmpeg >/dev/null
command -v ffprobe >/dev/null
cargo build --release --locked --offline
mkdir -p output
target/release/arcade-galaxy --software --record output/cpu-flight-silent.mp4 --quality balanced
python3 tools/mix_delivery_audio.py output/cpu-flight-silent.mp4 output/Proyecto_Final_CPU.mp4
