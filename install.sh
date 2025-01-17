#!/bin/bash

# Build the project
cargo build --release

# Create installation directory
sudo mkdir -p /usr/local/bin

# Copy the binary
sudo cp target/release/ricky /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/ricky

echo "Ricky Log manager has been installed successfully!"
echo "You can now use the 'ricky' command from anywhere in your terminal."