#!/bin/bash

export PATH=$HOME/.local/bin:$PATH
git clone https://github.com/quixdb/squash.git libsquash --depth=1
pushd libsquash || exit $?
git submodule update --init --recursive || exit $?
PATH=/usr/lib/ccache ./configure --disable-external --prefix="$HOME/.usr" --with-plugin-dir="$HOME/.usr/plugins" || exit $?
PATH=/usr/lib/ccache make -j6 || exit $?
PATH=/usr/lib/ccache make install || exit $?
popd

export LIBRARY_PATH="$HOME/.usr/lib${LIBRARY_PATH:+:}${LIBRARY_PATH}"
export SQUASH_PLUGINS="$HOME/.usr/plugins"
export PKG_CONFIG_PATH="$HOME/.usr/lib/pkgconfig${PKG_CONFIG_PATH:+:}${PKG_CONFIG_PATH}"
export LD_LIBRARY_PATH="$HOME/.usr/lib${LD_LIBRARY_PATH:+:}${LD_LIBRARY_PATH}"
