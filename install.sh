#!/bin/bash

# Download kill_port.zip using curl
curl -LOk https://github.com/Ma1y0/kill_port/releases/download/1.0.0/kill_port.zip

# Unzip kill_port.zip and move it to /usr/bin
unzip -o kill_port.zip
mv -f kill_port /usr/bin

# Clean up the downloaded zip file
rm kill_port.zip

echo "kill_port has been installed successfully!"

