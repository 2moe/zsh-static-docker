# zsh-static-docker

![platforms](./assets/markmap/platforms.svg)

## get started

### Dockerfile

```dockerfile
# zsh:
COPY --from=ghcr.io/2moe/zsh-static /opt/bin/zsh /bin/zsh
# busybox:
# COPY --chmod=755 --from=ghcr.io/2moe/zsh-static /opt/bin/busybox /bin/ash
```

### CLI

```sh
# files: ./tmp/zsh  ./tmp/busybox
docker run --rm -v "$PWD/tmp":/host ghcr.io/2moe/zsh-static cp busybox zsh /host/

# Considering that not all architectures are added to the `latest` manifest.
# If you are using an "unpopular" architecture (e.g., sparc, mipsle),
# you will need to specify the tag manually. (e.g., ghcr.io/2moe/zsh-static:mipsle)
```

### Github Actions workflow

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    defaults:
      run:
        shell: zsh --pipefail -fex {0}
    steps:
      - name: install zsh
        shell: sh -e {0}
        run: docker run --rm -v /usr/local/bin:/host ghcr.io/2moe/zsh-static cp zsh /host/

      - name: test zsh
        run: |
          local -A map=(
            focal    "20.04"
            groovy   "20.10"
            hirsute  "21.04"
            impish   "21.10"
            jammy    "22.04"
            kinetic  "22.10"
            lunar    "23.04"
            mantic   "23.10"
            noble    "24.04"
          )
          for k v (${(kv)map}) {
            print -P "%F{cyan}key: %F{blue}$k%f \t value: $v"
          }
```

### No Docker

**If you don't have docker installed, or the kernel doesn't support.**

run it on posix-sh:

```sh
# [
#   "x64", "rv64gc", "arm64", "armv7a", "armv5te", "mips64le",
#   "ppc64le", "s390x", "x86", "alpha", "hppa", "loong64",
#   "m68k", "ppc", "ppc64", "sh4", "sparc64", "ia64", "x32",
#   "mipsle", "mipsbe", "armv4t", "sparc", "s390", "armv3"
# ]
#
# You can change arch=x64 to another value.
# e.g., arch=rv64gc, arch=arm64
arch=x64

mkdir -p tmp/zsh-dir
cd tmp
curl -LO "https://github.com/2moe/zsh-static-docker/releases/download/latest/zsh-${arch}.tar.zst"
tar -C zsh-dir -xf zsh-${arch}.tar.zst

# test zsh: print hello world
zsh-dir/opt/bin/zsh -fc 'print -P %F{cyan}Hello World'

# run nspawn as root (i.e., +sudo/+doas)
systemd-nspawn -D zsh-dir -E PATH=/usr/local/bin:/bin:/opt/bin
```

## other notes

- About ia64(a.k.a., 64-Bit Intel Itanium architecture)
  - Modern qemu does not support emulating ia64.
  - If you don't have a machine (server) with ia64 cpu, then you may not be able to run ia64 architecture containers.

## Q&A

**What follows is not necessary to read.**

---

> Q: Should I use zsh-static?

A: When you ask this question, I don't think you need to.
In most cases, you should use normal zsh (i.e., installed with a package manager).
Only in special cases do you need to use zsh-static.

---

> Q: Is it useful?

A: Let's say there is a situation where you use `apt update; apt install zsh` which takes 6s, and `docker` which takes 3s to pull & run the zsh-static container, and the 3s saved is useful to you, then it is useful.

---

> Q: Why did you create this repo?

A：
初衷是为了方便在 Dockerfile 以及 CI 流程 中使用 zsh。

The main reason is that 我不太喜欢用 POSIX-sh & bash。

In my opinion, POSIX-sh 不太好用。

在大多数情况下，使用 POSIX-sh 是出于 compatibility 的考量。
因为不同 Unix-Like 系统预装的 shell 是不一样的，有些是 busybox ash, 有些是 bash, 有些是 zsh, 还有些是 pdksh。
为了兼容不同的 Unix-Like 系统，我们只能用 POSIX-sh 的语法了。

而 bash 呢？

- 客观上看：bash 4.x 以及 5.2.15 的性能相较于 debian 自带的 dash 和 alpine 自带的 ash 没有优势
  - 就算用 --posix --noprofile 也慢了很多
- 主观上看 (仅代表个人看法)： 其语法相较于 zsh 没有优势。

> 上面这段话来自于 gitee 的某个 [issue](https://gitee.com/mo2/linux/issues/I91P73)

其实我真的不是想要刻意贬低 bash 与 posix-sh，最主要的原因是我不太喜欢它们。

> 注：awk 不是 posix-sh, 它是一门单独的语言，挺有意思的。

如果我是真心喜欢一门语言的话，那么它就算有再多的缺点，我都能欣然接受；
如果我太不喜欢的话，那么它的缺点就会被无限放大。

喜欢与否是一件相当主观的事情，上面内容仅代表本人的看法。
君若无意，则不必强求。

其实创建这个 repo 还有一个契机，容我慢慢与您道来。

事情是这样子的：

此前，我想要研究一下 ArchLinux 的基本 rootfs 的构建步骤。
再加上之前有个疑问：为什么 ArchLinux 的最小 rootfs 压缩后都要 100M+？
于是乎，好奇心驱使我对此进行探索。

我想试试裁剪 rootfs, 不求做到 alpine 那种 2 ~ 4M 的大小，只想看看能否做到像 ubuntu noble 那样 25 ~ 40M 的大小。
后来，我发现容器内只要包含 pacman-static + pacman 配置 + ca证书 + busybox-static, 就能构建不同架构的 rootfs 了。

ca 证书 (**/etc/ca-certificates/extracted/tls-ca-bundle.pem**) 是可选的，不过要是无证书的话，就无法保证 https 连接的安全性，此时就得要引入 pgp 相关依赖，这样就变得更麻烦了。

> 由于 `/bin` , `/usr/bin`,  `/sbin` & `/usr/sbin` 合并了，为了避免目录的干扰，故 busybox 安装在 `/opt/bin`。

这些东西加起来，压缩后才 4M+ (i.e., 最小的 ArchLinux 可以像嵌入式发行版一样做到只有 4M)。
这时候如果用 `pacman-static -Syy base --overwrite '*'` 安装 `base`，那么压缩后的体积又变成了 100M+。

如果不装 `base`，只装一些特别基础的包，可以做到 70M+，不过得要手动修复一些问题。
在未安装 systemd 的情况下，有些东西得要手动去配置 (e.g., `useradd` 会弹出没有 `users` 用户的警告)。

> 其实并非越精简越好，比如说：在某些情况下，我需要国际化与本地化数据 (i18n 与 L10n)，以及 man 文档，删掉或过滤掉的话，就与我的需求相违背了。

Dockerfile 里的构建步骤，本来是用 posix-sh 语法来写的。

后来，我就想：反正我挺喜欢 zsh 的，不如搞个 zsh-static 容器，之后不单单是构建 ArchLinux,
其他的东西（诸如 CI 流程）也能用 zsh 语法来写。

最后，让我们庆祝这个 repo 的诞生 🥳！

<del>
Blessings for your birthday!

Blessings for your everyday!

Aunque el mundo se pueda acabar, disfrúalo.
</del>
