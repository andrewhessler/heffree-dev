#!/bin/bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd $parent_path

git pull

cargo install --path ./

heffree-dev

cp -r ./dist/* /var/lib/heffree-dev/

sudo systemctl reload nginx

echo "Build, install, and service restart complete."

