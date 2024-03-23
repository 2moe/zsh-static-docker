# zsh-static-docker

![platforms](./assets/markmap/platforms.svg)

## get zsh bin

```sh
# riscv64
tag=rv64gc

# loongarch64:
tag=loong64

# aarch64:
tag=arm64

# x86_64:
tag=x64

# unknown:
tag=latest

docker run --rm -v /usr/local/bin:/app ghcr.io/2moe/zsh-static:$tag cp /opt/bin/zsh /app/zsh-static

# test: print Hello World
/usr/local/bin/zsh-static -c 'print -P "%F{blue}Hello %F{cyan}World%f"'
```
