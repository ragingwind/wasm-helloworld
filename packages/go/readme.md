# Install

- Go lang with vesion [over 1.11](https://golang.org/dl/)

# Usage

```
# build with your go bin, make sure that your go must support web assembly
GOROOT=~/Go/release/go1.11beta1/ make build

# run and visit https://localhost:8080
make PORT=8080 run

# build server as you want
make build-server
```