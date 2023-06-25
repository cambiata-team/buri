#!/bin/sh

set -e

ARCH=`uname -m`
OS=`uname`
FILENAME="buri_${OS}_${ARCH}"
EXTENSION=""

if [ "${OS}" = "Windows" ]
then
    EXTENSION=".zip"
else
    EXTENSION=".tar.gz"
fi

URL="https://github.com/cambiata-team/buri-go/releases/latest/download/"
URL="${URL}${FILENAME}${EXTENSION}"

echo "Downloading buri... $URL"
curl --fail --location --progress-bar --output /usr/local/bin/buri $URL
if [ "${OS}" == "Windows" ]
then
    unzip -a /usr/local/bin/buri
else
    tar -xvf /usr/local/bin/buri
fi
rm /usr/local/bin/buri
mv buri /usr/local/bin/buri
chmod +x /usr/local/bin/buri
echo "buri was successfully installed to /usr/local/bin/buri"
echo "Run \`bkg --help\` to get started!"
