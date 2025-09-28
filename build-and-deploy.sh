#!/bin/bash
set -e

git pull

cp -r ./src/assets/* /var/lib/heffree-dev/

sudo systemctl reload nginx

echo "Build, install, and service restart complete."

