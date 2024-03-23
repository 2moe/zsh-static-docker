# zsh-static-docker

![platforms](./assets/markmap/platforms.svg)

## get zsh bin

```sh
# riscv64
arch=rv64gc

# loongarch64:
arch=loong64

# aarch64:
arch=arm64

# x86_64:
arch=x64

docker run --rm -v /usr/local/bin:/app ghcr.io/2moe/zsh-static:$arch cp /opt/bin/zsh /app/zsh-static
```
