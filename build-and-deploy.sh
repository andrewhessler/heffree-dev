#!/bin/bash
set -e

git pull

cargo install --path ./

heffree-dev

cp -r ./src/assets/* /var/lib/heffree-dev/

sudo systemctl reload nginx

echo "Build, install, and service restart complete."

