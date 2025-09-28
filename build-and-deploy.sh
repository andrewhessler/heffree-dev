#!/bin/bash
set -e

git pull

cp -r ./src/assets/* /var/lib/heffree-dev/

sudo systemctl nginx reload

echo "Build, install, and service restart complete."

