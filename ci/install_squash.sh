#!/bin/sh

export PATH=$HOME/.local/bin:$PATH
git clone https://github.com/quixdb/squash.git libsquash
(
    cd libsquash || exit $?
    git pull && git submodule update --init --recursive || exit $?
    ./configure --disable-external --prefix="$HOME/.usr" --with-plugin-dir="$HOME/.usr/plugins" || exit $?
    make -j6 || exit $?
    make install || exit $?
)


export LIBRARY_PATH="$HOME/.usr/lib${LIBRARY_PATH:+:}${LIBRARY_PATH}"
export SQUASH_PLUGINS="$HOME/.usr/plugins"
export PKG_CONFIG_PATH="$HOME/.usr/lib/pkgconfig${PKG_CONFIG_PATH:+:}${PKG_CONFIG_PATH}"
export LD_LIBRARY_PATH="$HOME/.usr/lib${LD_LIBRARY_PATH:+:}${LD_LIBRARY_PATH}"
