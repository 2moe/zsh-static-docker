# syntax=docker/dockerfile:1
#
# reg.tmoe.me:2096/pkgs/zsh-static
ARG ZSH_REPO=ghcr.io/2moe/zsh-static
# reg.tmoe.me:2096/debian/sid
ARG SID_REPO=ghcr.io/2cd/debian-sid
#
FROM --platform=${BUILDPLATFORM} ${ZSH_REPO} as zsh-host
FROM --platform=${BUILDPLATFORM} ${SID_REPO} as sid
#
ARG DEBIAN_FRONTEND noninteractive
# -----------------
# build-args:
ARG REGION=US
ARG DEB_ARCH
ARG DEB_SUITE
COPY --chmod=755 --from=zsh-host /opt/bin/busybox /opt/bin/zsh /bin/

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
        *)
        # sed -E -e "$sed_exp" -e 's@^#.*(URIs:)(.*?mirror.*\s+)@\1 @' -e 's@^URIs:.*snapshot@#&@' -i $SRC
        sed -E -e "$sed_exp" -e 's@^(URIs:).*@\1 https://deb.debian.org/debian/@g' -i $SRC
        ;;
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
        (amd64|riscv64|arm64|armhf|armel|s390x|ppc64el|i386|mips64el|mips|mipsel|sparc|s390|arm) ;;
        (*) sed -E 's@/(debian)/@/\1-ports/@g' -i $SRC ;;
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
#-------------
    list=($(/bin/busybox --list))
    for i (${list:#busybox}) {
        ln -sf ./busybox $i
    }
    #
    # -: last, busybox, nuke, uncompress, w, who
    # list=(
    #     '[' '[[' acpid adjtimex ar arch arp arping ascii ash awk base64 basename bc blkdiscard blockdev brctl bunzip2 bzcat bzip2 cal cat chgrp chmod chown chroot chvt clear cmp cp cpio crc32 crond crontab cttyhack cut date dc dd deallocvt depmod devmem df diff dirname dmesg dnsdomainname dos2unix dpkg dpkg-deb du dumpkmap dumpleases echo ed egrep env expand expr factor fallocate false fatattr fdisk fgrep find findfs fold free freeramdisk fsfreeze fstrim ftpget ftpput getopt getty grep groups gunzip gzip halt head hexdump hostid hostname httpd hwclock i2cdetect i2cdump i2cget i2cset i2ctransfer id ifconfig ifdown ifup init insmod ionice ip ipcalc kill killall klogd  less link linux32 linux64 linuxrc ln loadfont loadkmap logger login logname logread losetup ls lsmod lsscsi lzcat lzma lzop md5sum mdev microcom mim mkdir mkdosfs mke2fs mkfifo mknod mkpasswd mkswap mktemp modinfo modprobe more mount mt mv nameif nbd-client nc netstat nl nologin nproc nsenter nslookup  od openvt partprobe passwd paste patch pidof ping ping6 pivot_root poweroff printf ps pwd rdate readlink realpath reboot renice reset resume rev rm rmdir rmmod route rpm rpm2cpio run-init run-parts sed seq setkeycodes setpriv setsid sh sha1sum sha256sum sha3sum sha512sum shred shuf sleep sort ssl_client start-stop-daemon stat strings stty su sulogin svc svok swapoff swapon switch_root sync sysctl syslogd tac tail tar taskset tc tee telnet telnetd test tftp time timeout top touch tr traceroute traceroute6 true truncate ts tty tunctl ubirename udhcpc udhcpc6 udhcpd uevent umount uname unexpand uniq unix2dos unlink unlzma unshare unxz unzip uptime usleep uudecode uuencode vconfig vi watch watchdog wc wget which whoami xargs xxd xz xzcat yes zcat
    # )
    # for i ($list) {
    #     ln -sf ./busybox $i
    # }
LINK_BUSYBOX

# ----------
FROM scratch
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/local/games:/bin:/usr/games:/opt/bin
COPY --from=sid /app/ /opt/bin
SHELL [ "/opt/bin/zsh", "--pipefail", "-fexc"]
CMD [ "/opt/bin/zsh" ]
