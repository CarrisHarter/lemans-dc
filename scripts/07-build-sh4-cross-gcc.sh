#!/usr/bin/env bash
# Build a freestanding SH-4 cross compiler (stage 1) without root.
#
# Why not the AUR package: sh-elf-gcc makedepends sh-elf-newlib and newlib
# depends on sh-elf-gcc, a cycle yay cannot resolve. Decompiled game functions
# are freestanding C, so headers/libc are not needed; --without-headers is
# exactly the right bootstrap.
#
# Result: $PREFIX/bin/sh-elf-gcc, ~/opt/sh-elf/bin
set -euo pipefail

PREFIX="$HOME/opt/sh-elf"
DL="$HOME/opt/dl"
SRC="$HOME/opt/src"
GCCVER=15.2.0

mkdir -p "$DL" "$SRC" "$PREFIX"

if [ ! -f "$DL/gcc-$GCCVER.tar.gz" ]; then
  echo "== downloading gcc $GCCVER"
  curl -fSL --retry 3 -o "$DL/gcc-$GCCVER.tar.gz" \
    "https://github.com/gcc-mirror/gcc/archive/refs/tags/releases/gcc-$GCCVER.tar.gz"
fi

if [ ! -d "$SRC/gcc-releases-gcc-$GCCVER" ]; then
  echo "== extracting"
  tar -xzf "$DL/gcc-$GCCVER.tar.gz" -C "$SRC"
fi

BUILD="$SRC/build-sh-elf"
mkdir -p "$BUILD"
cd "$BUILD"

# Host libstdc++ 16 makes libcody's u8"..." literals char8_t (C++20), which
# breaks its std::string comparisons. The ASCII literals are rewritten below;
# an explicit -std is not used because it breaks libcody's own C++11 probe.
if [ ! -f "$SRC/gcc-releases-gcc-$GCCVER/.libcody-patched" ]; then
  find "$SRC/gcc-releases-gcc-$GCCVER/libcody" -maxdepth 1 -type f \( -name '*.cc' -o -name '*.h' \) \
    -exec sed -i 's/u8"/"/g' {} +
  touch "$SRC/gcc-releases-gcc-$GCCVER/.libcody-patched"
  echo "== patched libcody u8 literals"
fi

echo "== configure ($PREFIX)"
"$SRC/gcc-releases-gcc-$GCCVER/configure" \
  --target=sh-elf \
  --prefix="$PREFIX" \
  --without-headers \
  --with-newlib \
  --disable-nls \
  --disable-shared \
  --disable-threads \
  --disable-libssp \
  --disable-libgomp \
  --disable-libquadmath \
  --disable-libatomic \
  --disable-libmudflap \
  --enable-languages=c \
  --with-multilib-list=m4-single-only

echo "== make all-gcc"
make -j"$(nproc)" all-gcc
make install-gcc

echo "== done"
"$PREFIX/bin/sh-elf-gcc" -v 2>&1 | tail -2
