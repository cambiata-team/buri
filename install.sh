#!/bin/sh

set -e

ARCH=`uname -m`
OS=`uname`
TARGET="buri_${OS}_${ARCH}"

if [ "${OS}" = "Windows" ]
then
    TARGET="${TARGET}.zip"
else
    TARGET="${TARGET}.tar.gz"
fi

URL="https://github.com/cambiata-team/buri-go/releases/latest/download/"
URL="${URL}${TARGET}"

echo "Downloading buri... $URL"
curl --fail --location --progress-bar --output /usr/local/bin/buri $URL
chmod +x /usr/local/bin/buri
echo "buri was successfully installed to /usr/local/bin/buri"
echo "Run \`bkg --help\` to get started!"
