#!/bin/zsh
set -e
cd "${0:A:h}/.."
command -v ffmpeg >/dev/null
command -v ffprobe >/dev/null
cargo build --release --offline --locked
mkdir -p output
target/release/arcade-galaxy --record output/green-hill-silent.mp4
python3 tools/mix_delivery_audio.py output/green-hill-silent.mp4 output/Proyecto_Final_Green_Hill_Galaxy.mp4
