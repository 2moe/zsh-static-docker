# zsh-static-docker

## Platforms

![platforms](./assets/markmap/platforms.svg)

## Run

```sh
# x86_64:
arch=x64
# aarch64:
arch=arm64
# loongarch64:
arch=loong64
# riscv64
arch=rv64gc

docker run -it --rm ghcr.io/2moe/zsh-static:$arch
```
