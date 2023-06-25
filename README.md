# Buri

## Install

The fastest way to install the Buri tool is to run the following command:

```sh
curl -fsSL https://github.com/cambiata-team/buri-go/raw/main/install.sh | sudo sh
```

## Contributing

### Protos

The command to regenerate the protos is:

```sh
protoc -I=./protos --go_out=./ ./protos/workspace.proto
```
