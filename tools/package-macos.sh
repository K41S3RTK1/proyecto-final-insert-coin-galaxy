#!/bin/zsh
set -e
cd "${0:A:h}/.."
bundle="target/Insert Coin Galaxy.app"
mkdir -p "$bundle/Contents/MacOS" "$bundle/Contents/Resources/assets"
cp target/release/arcade-galaxy "$bundle/Contents/MacOS/arcade-galaxy.new"
mv -f "$bundle/Contents/MacOS/arcade-galaxy.new" "$bundle/Contents/MacOS/arcade-galaxy"
mkdir -p "$bundle/Contents/Resources/assets/audio"
cp assets/audio/*.wav "$bundle/Contents/Resources/assets/audio/"
cp assets/arcade-ship.obj assets/arcade-ship.mtl "$bundle/Contents/Resources/assets/"
cp platform/macos/Info.plist "$bundle/Contents/Info.plist"

mkdir -p "$bundle/Contents/Resources/assets/fonts"
cp assets/fonts/* "$bundle/Contents/Resources/assets/fonts/"

mkdir -p "$bundle/Contents/Resources/assets/models"
cp assets/models/*.obj assets/models/*.mtl "$bundle/Contents/Resources/assets/models/"

mkdir -p "$bundle/Contents/Resources/assets/textures"
cp assets/textures/*.png "$bundle/Contents/Resources/assets/textures/"
cp assets/models/placements.json "$bundle/Contents/Resources/assets/models/"
