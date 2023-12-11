#!/bin/bash

# Build the project for the wasm32-unknown-unknown target
cargo build --release --target wasm32-unknown-unknown --target-dir "target"

# Remove existing build and out directories if they exist
rm -rf ./build/*
rm -rf ./out/*

mkdir -p ./out
# Run wasm-bindgen
wasm-bindgen --no-typescript --target web --out-dir ./out --out-name "bevy_game" ./target/wasm32-unknown-unknown/release/flora_cause.wasm

# Copy files from the wasm directory to the out directory
cp -r ./wasm/* ./out/

# Create the build directory if it doesn't exist
mkdir -p ./build

# Compress the contents of the out directory into a zip file in the build directory
zip -r "./build/flora-cause-wasm2.zip" ./out/*
