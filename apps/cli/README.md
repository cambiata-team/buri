# Buri CLI

Ensures the correct version of Thor is installed.

## Installation

Follow the README for this repository to download the CLI. The CLI will automatically download any version of Thor you need.

## FAQ

### What does this do, really?

Well, the functionality is really simple:

1. If there's not a workspace
    - Tell the user to create a workspace `buri init`
2. If there's a workspace
    - Determine the correct version of Thor
        - Check the `.burirc.toml`
        - If it doesn't exist, call the version API
            - Then write to the `.burirc.toml`
    - If the version isn't downloaded, download it
    - Execute the version of Thor, passing through all arguments

Yes, it's really that simple.

### How does it know which version of Thor to run?

It has a simple algorithm:

- If there's a `.burirc.toml` file in the current directory, use that version.
- Else, download the latest version.

If it downloads the latest version, it will also write a `.burirc.toml` file in the current directory to ensure you make a network request for every subsequent run.

### How does it know where to download Thor from?

It gets the download URL and checksum from [https://version-api.buri-lang.dev](https://version-api.buri-lang.dev).
