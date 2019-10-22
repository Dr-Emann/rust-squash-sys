#!/bin/sh

ALL_REGEX='(?i).*squash.*'

my_cflags=$(pkg-config --cflags squash-0.8)
if [ $? -ne 0 ]; then
    echo "Failed to find squash-0.8 library" >&2
    exit 1
fi

bindgen ./wrapper.h -o src/bindings.rs \
    --generate-inline-functions \
    --impl-debug --impl-partialeq \
    --no-prepend-enum-name \
    --default-enum-style moduleconsts \
    --bitfield-enum "SquashCodecInfo" \
    --bitfield-enum "SquashLicense" \
    --whitelist-type "$ALL_REGEX" \
    --whitelist-function "$ALL_REGEX" \
    --whitelist-var "$ALL_REGEX" \
    --blacklist-type 'FILE' \
    --opaque-type 'FILE' \
    --blacklist-type 'wchar_t' \
    -- $my_cflags
