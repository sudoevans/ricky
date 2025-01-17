#!/bin/bash

# Build the project
cargo build --release

# Create installation directory
sudo mkdir -p /usr/local/bin

# Copy the binary
sudo cp target/release/log /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/log

echo "Ricky has been installed successfully!"
echo "You can now use the 'log' command from anywhere in your terminal."