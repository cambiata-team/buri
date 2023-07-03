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

TEMP_FILE=$(mktemp)
TEMP_DIRECTORY=$(mktemp -d)
BURI_LOCATION=/usr/local/bin/buri
CONFIG_DIRECTORY=~/.config/buri
VERSION_URL=https://version-api.buri-lang.dev/get-latest-cli-version-plaintext

mkdir -p $CONFIG_DIRECTORY
curl --fail --location $CONFIG_DIRECTORY/cli-version $VERSION_URL
curl --fail --location --progress-bar --output $TEMP_FILE $URL
tar -xvf $TEMP_FILE -C $TEMP_DIRECTORY
mv $TEMP_DIRECTORY/cli $BURI_LOCATION
rm $TEMP_FILE
rm -rf $TEMP_DIRECTORY
chmod +x $BURI_LOCATION
echo ""
echo "buri was successfully installed to ${BURI_LOCATION}"
echo "Run \`buri --help\` to get started!"
