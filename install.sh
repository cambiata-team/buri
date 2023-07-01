#!/bin/sh

set -e

ARCH=`uname -m`
OS=`uname`
FILENAME="cli-"

if [ "${ARCH}" == "arm64" ]
then
    FILENAME="${FILENAME}aarch64-"
elif [ "${ARCH}" == "x86_64" ]
then
    FILENAME="${FILENAME}x86_64-"
else
    echo "Unsupported architecture: ${ARCH}"
    exit 1
fi

if [ "${OS}" == "Darwin" ]
then
    FILENAME="${FILENAME}apple-darwin"
elif [ "${OS}" == "Linux" ]
then
    FILENAME="${FILENAME}unknown-linux-gnu"
else
    echo "Unsupported OS: ${OS}"
    exit 1
fi

URL="https://github.com/cambiata-team/buri-go/releases/latest/download/"
URL="${URL}${FILENAME}.tar.gz"

echo "Downloading buri... $URL"
curl --fail --location --progress-bar --output /usr/local/bin/buri-tmp $URL
cd /usr/local/bin
if [ "${OS}" == "Windows" ]
then
    unzip -a buri-tmp
else
    tar -xvf buri-tmp
fi
rm /usr/local/bin/buri-tmp
mv /usr/local/bin/cli /usr/local/bin/buri
chmod +x /usr/local/bin/buri
echo "buri was successfully installed to /usr/local/bin/buri"
echo "Run \`buri --help\` to get started!"
