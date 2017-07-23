#!/bin/bash

export PATH=$HOME/.local/bin:$PATH
git clone https://github.com/quixdb/squash.git libsquash --depth=1
pushd libsquash || exit $?
git pull && git submodule update --init --recursive || exit $?
CC=ccache ./configure --disable-external --prefix="$HOME/.usr" --with-plugin-dir="$HOME/.usr/plugins" || exit $?
CC=ccache make -j6 || exit $?
CC=ccache make install || exit $?
popd

export LIBRARY_PATH="$HOME/.usr/lib${LIBRARY_PATH:+:}${LIBRARY_PATH}"
export SQUASH_PLUGINS="$HOME/.usr/plugins"
export PKG_CONFIG_PATH="$HOME/.usr/lib/pkgconfig${PKG_CONFIG_PATH:+:}${PKG_CONFIG_PATH}"
export LD_LIBRARY_PATH="$HOME/.usr/lib${LD_LIBRARY_PATH:+:}${LD_LIBRARY_PATH}"
