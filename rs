#!/bin/bash

# Get the path to the last edited C file
last_edited_C_file=$(ls -t *.c 2>/dev/null | head -n 1)

if [[ -z "$last_edited_C_file" ]]; then
  echo "Error: No C files found in the current directory."
  exit 1
fi

# Extract the filename without extension
filename="${last_edited_C_file%.*}"

# Compile the program with gcc
gcc "$last_edited_C_file" -o "$filename"

if [[ $? -eq 0 ]]; then
  echo "Compilation successful! Output: $filename"
  echo
  ./"$filename"
else
  echo "Compilation failed."
  exit 1
fi
