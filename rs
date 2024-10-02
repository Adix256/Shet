#!/bin/bash

# Get the path to the last edited Rust file
last_edited_rust_file=$(ls -t *.rs | head -n 1)

if [[ -z "$last_edited_rust_file" ]]; then
  echo "Error: No Rust files found in the current directory."
  exit 1
fi

# Extract the filename without extension
filename="${last_edited_rust_file%.*}"

# Compile the program with rustc
rustc "$last_edited_rust_file" -o "$filename"

if [[ $? -eq 0 ]]; then
  echo "Compilation successful! Output: $filename"
# Run the compiled program
./"$filename"

else
  echo "Compilation failed."
  exit 1
fi
