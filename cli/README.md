# Buri CLI

Ensures the correct version of Buri is installed.

## FAQ

### How does it know which version of Thor to run?

It has a simple algorithm:

- If there's a `.burirc` file in the current directory, use that version
- Else, use the latest Thor release.

### How does it know where to download Thor from?

It gets the download URL and checksum from [https://version-api.buri-lang.dev](https://version-api.buri-lang.dev).
