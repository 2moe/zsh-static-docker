# zsh-static-docker

![platforms](./assets/markmap/platforms.svg)

## get started

run it on posix-sh:

```sh
# values: latest, rv64gc, x64, x86, loong64,
#       arm64, armv7a, armv5te, armv4t, armv3,
#       mips64le, mipsle, mipsbe, m68k, sh4,
#       s390x, alpha, hppa, sparc64, sparc,
#       ppc64le, ppc64, ppc, x32, ia64
tag=latest

get_bin() {
    docker run --rm -v $PWD/tmp:/app ghcr.io/2moe/zsh-static:$tag cp /opt/bin/$bin /app/
}

# "zsh" | "busybox"
bin=zsh

get_bin
# test: print Hello World
./tmp/zsh -fc 'print -P "%F{blue}Hello %F{cyan}World%f"'

bin=busybox
get_bin
./tmp/busybox ash
```
