# syntax=docker/dockerfile:1
# build-args:
ARG OLD_REPO
ARG SID_REPO

FROM $OLD_REPO as old
FROM $SID_REPO as sid

ARG DEBIAN_FRONTEND noninteractive
# -----------------
# build-args:
ARG REGION=US
ARG DEB_ARCH
ARG DEB_SUITE
COPY --chmod=755 --from=old /opt/bin/busybox /opt/bin/zsh /bin/

ARG SRC=/etc/apt/sources.list.d/mirror.sources
RUN <<UPDATE_MIRROR_SRC
#!/bin/dash -ex
# ---------
    sed_exp="s@(Suites:) sid@\1 $DEB_SUITE@"

    case $REGION in
        CN)
            mirror_src=/usr/local/etc/apt/mirrors/NJU.CN.sources
            unlink $SRC
            sed -E -e "$sed_exp" $mirror_src > $SRC ;;
        *) sed -E -e "$sed_exp" -e 's@^#.*(URIs:)(.*?mirror.*\s+)@\1 @' -e 's@^URIs:.*snapshot@#&@' -i $SRC ;;
    esac

    # apt-get update
    # for i in zsh busybox; do
    #     apt-get install --no-install-recommends -y $i-static
    # done
    # cp -a /bin/zsh-static /bin/zsh ||:
UPDATE_MIRROR_SRC

# ---------
COPY --chmod=755 <<'GET_DEB_BIN' /bin/get_deb_bin
#!/bin/zsh -fex
    cd $(mktemp -d)
    pkg=$1
    dst=$2

    case $DEB_SUITE {
        (lenny|wheezy|stretch|buster)
            sed_args=( -E -e 's/non-free-firmware//g' )

            case $DEB_SUITE {
                (lenny|wheezy|stretch)
                    sed_args+=( -e 's@^(URIs:).*@\1 https://archive.debian.org/debian/@' )
            }
        sed_args+=(-i $SRC)
        sed $sed_args
    }

    case $DEB_ARCH {
        (amd64) name=$pkg ;;
        (*)
            dpkg --add-architecture $DEB_ARCH
            name=$pkg:$DEB_ARCH
        ;;
    }
    case $DEB_ARCH {
        (amd64|riscv64|arm64|armhf|armel|s390x|ppc64el|i386|mips64el|mips|mipsel|sparc|arm) ;;
        (*) sed 's@/debian/@/debian-ports/@g' -i $SRC ;;
    }
    cat $SRC
    apt-get update
    apt-get download $name
    dpkg-deb -X ${pkg}*.deb  .
    rm ${pkg}*.deb
    () {
        for i (./bin ./usr/bin) {
            for bin ($i/$pkg $i/$dst $i/${dst}4-static) {
                if [[ -e $bin ]] {
                    return
                }
            }
        }
    }
    install -Dm755 $bin /app/$dst
GET_DEB_BIN

# ---------
RUN <<DOWNLOAD_BINS
#!/bin/zsh -fex

    for i (zsh busybox) {
        pkg=$i-static
        get_deb_bin $pkg $i
    }
DOWNLOAD_BINS

WORKDIR /app
RUN <<LINK_BUSYBOX
#!/bin/zsh -fe
list=($(/bin/busybox --list))

for i (${list:#busybox}) {
    ln -sf busybox $i
}
LINK_BUSYBOX

# ----------
FROM scratch
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/local/games:/bin:/usr/games:/opt/bin
COPY --from=sid /app/ /opt/bin
SHELL [ "/opt/bin/zsh", "--pipefail", "-fexc"]
CMD [ "/opt/bin/zsh" ]
