#!/bin/bash

# Get current directory name (assumes it's the binary name)
BIN_NAME="$(basename "$PWD")"
BIN_PATH="zig-out/bin/$BIN_NAME"

# Step 1: Build
echo "[*] Building $BIN_NAME for aarch64-linux-android..."                                      zig build -Dtarget=aarch64-linux-android        
# Step 2: Check and run
if [ -f "$BIN_PATH" ]; then                         echo "[*] Build successful. Running $BIN_NAME..."                                               "$BIN_PATH"                                 else
    echo "[!] Build failed or binary not found: $BIN_PATH"                                          exit 1                                      fi
