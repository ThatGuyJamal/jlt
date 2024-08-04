#!/bin/bash

# Check if a directory is passed as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

DIRECTORY=$1

# Check if the provided argument is a directory
if [ ! -d "$DIRECTORY" ]; then
    echo "Error: $DIRECTORY is not a directory."
    exit 1
fi

# Function to apply chmod +x to .sh files
make_executable() {
    find "$1" -type f -name "*.sh" -exec chmod +x {} \;
}

# Run the function on the provided directory
make_executable "$DIRECTORY"

echo "chmod +x applied to all .sh files in $DIRECTORY and its subdirectories."