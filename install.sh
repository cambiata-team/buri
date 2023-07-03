#!/bin/sh

#
# Purpose
#
# This script installs the latest version of the Buri CLI tool.
#
# Overview
#
# 1. Build up the URL to download the latest version of the CLI tool.
#    This URL depends on your current operating system and architecture.
# 2. Fetch the version number of the latest version of the CLI tool and
#    store it in a configuration file. While this is not necessary for
#    this script, the CLI tool itself will use this file when a user
#    invokes it.
# 3. Download the latest version of the CLI tool.
# 4. Uncompress the file and move it to the correct location.
# 5. Cleanup all temporary files and directories.
#
# Assumptions
#
# - No Windows builds.
# - All latest builds are the same version.
# - Failing is better than error handling.
#
# Contributing
#
# If you want to contribute to this script, please do! Just follow these
# guidelines:
#
# - Before starting, read through the entire script to know what it does.
# - Make sure the script is idempotent. The user should be able to run
#   it multiple times, downloading the newest version of the CLI each time.
# - Comment everything. Shell scripts are impossible to test, so comments
#   are the only way future readers know contributors know for sure your
#   intent.
# - Update all comments. Outdated comments are worse than no comments.
#   Read through the entire script and all it's comments to ensure they
#   remain up to date with your changes.

# Exit if any command fails.
# https://www.gnu.org/savannah-checkouts/gnu/bash/manual/bash.html#index-set
set -e

#
# Build up the file name
#

ARCHITECTURE=`uname -m`
OS=`uname`
# The name of the rust crate is CLI, so the file name starts with CLI
# even though the user will invoke it with `buri`.
FILENAME="cli-"
BASE_DOWNLOAD_URL="https://github.com/cambiata-team/buri-go/releases/latest/download/"

# Add the architecture to the file name
if [ "${ARCHITECTURE}" == "arm64" ]
then
    FILENAME="${FILENAME}aarch64-"
elif [ "${ARCHITECTURE}" == "x86_64" ]
then
    FILENAME="${FILENAME}x86_64-"
else
    echo "Unsupported architecture: ${ARCHITECTURE}"
    exit 1
fi

# Add the OS to the file name
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

# Generate the download URL.
DOWNLOAD_URL="${BASE_DOWNLOAD_URL}${FILENAME}.tar.gz"

#
# Initialize variables to be used later
#

# Where we download the cli to.
DOWNLOAD_LOCATION=$(mktemp)
# Where we uncompress the cli into.
UNCOMPRESSED_TEMP_DIRECTORY=$(mktemp -d)
# The final location for the CLI tool. Should be in a directory already
# included in the user's PATH.
BURI_LOCATION=/usr/local/bin/buri
# The directory where we store config files.
CONFIG_DIRECTORY=~/.config/buri
CLI_VERSION_FILE_NAME=cli-version.txt
CLI_VERSION_FILE_PATH=$CONFIG_DIRECTORY/$CLI_VERSION_FILE_NAME
# The URL where we fetch the latest version number.
VERSION_URL=https://version-api.buri-lang.dev/get-latest-cli-version-plaintext

#
# Fetch the latest version number and store it in the config directory.
# Will be used later by the CLI tool to know it's own version.
# We can remove this once we can compile the CLI tool with the
# version number baked in. Until then, it will need to rely on this
# version file to know it's own version.
#
echo "Fetching the latest version number... $VERSION_URL"
mkdir -p $CONFIG_DIRECTORY
curl --fail --location $CLI_VERSION_FILE_PATH $VERSION_URL

#
# Download the latest version of the CLI tool.
#
echo "Downloading buri... $DOWNLOAD_URL"
curl --fail --location --progress-bar --output $DOWNLOAD_LOCATION $DOWNLOAD_URL

#
# Uncompress the file and move it to the correct location.
# Make it executable.
#
tar -xvf $DOWNLOAD_LOCATION -C $UNCOMPRESSED_TEMP_DIRECTORY
mv $UNCOMPRESSED_TEMP_DIRECTORY/cli $BURI_LOCATION
chmod +x $BURI_LOCATION

#
# Cleanup temporary files and directories.
#
rm $DOWNLOAD_LOCATION
rm -rf $UNCOMPRESSED_TEMP_DIRECTORY

#
# Report success messages to the user.
#
echo ""
echo "buri was successfully installed to ${BURI_LOCATION}"
echo "Run \`buri --help\` to get started!"
