# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    # cross build --target $TARGET
    # cross build --target $TARGET --release

    cargo build
    cargo build --release

    echo "$TRAVIS_RUST_VERSION"
    if [ "$TRAVIS_RUST_VERSION" -eq "nightly" ]; then
        cargo build --features=nightly
    fi

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cargo test
    cargo test --release
    if [ "$TRAVIS_RUST_VERSION" -eq "nightly" ]; then
        cargo test --features=nightly
        cargo test --release --features=nightly
    fi

    # cross test --target $TARGET
    # cross test --target $TARGET --release

    # cross run --target $TARGET
    # cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
