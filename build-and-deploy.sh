#!/bin/bash
set -e

git pull

cd src/client

npm run build

cd ../..

# Build the project in release mode
echo "Building the project in release mode..."
cargo build --release

# Verify the executable exists
if [ ! -f "target/release/heffree-dev" ]; then
  echo "Error: Executable not found at target/release/heffree-dev"
  echo "Ensure your Cargo.toml specifies the package name as 'backend'."
  exit 1
fi

# Move the executable to /usr/local/bin (sudo may be required)
echo "Moving executable to /usr/local/bin/heffree-dev"
sudo mv target/release/heffree-dev /usr/local/bin/heffree-dev

# Restart the systemd service
sudo systemctl daemon-reload
echo "Restarting the systemd service 'heffree-dev.service'..."
sudo systemctl restart heffree-dev.service

echo "Build, install, and service restart complete."

