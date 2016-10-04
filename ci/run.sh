set -ex

main() {
    local target=$1
    local profiles="debug release"
    local weak_symbols=

    if [ $target = x86_64-unknown-linux-gnu ]; then
        cargo build

        # test layout of register maps
        cargo run --example _test

        return
    fi

    # test weak symbols and overrides
    for example in $(ls examples); do
        case $example in
            _test.rs | *.rs.bk)
                continue
                ;;
        esac

        xargo build --target $target --example ${example%.*}
        xargo build --target $target --example ${example%.*} --release
    done

    for profile in $profiles; do
        weak_symbols=$(arm-none-eabi-nm -g target/$target/$profile/examples/_all-the-weak-symbols | grep ' W ')

        echo "$weak_symbols" | grep _default_exception_handler
        echo "$weak_symbols" | grep _init
        echo "$weak_symbols" | grep rust_begin_unwind
    done

    for profile in $profiles; do
        weak_symbols=$(arm-none-eabi-nm -g target/$target/$profile/examples/override-init | grep ' W ')

        # `_init` should have been overriden
        set +e
        echo "$weak_symbols" | grep _init
        test $? -eq 0 && exit 1
        set -e
    done

    for profile in $profiles; do
        weak_symbols=$(arm-none-eabi-nm -g target/$target/$profile/examples/override-panic-fmt | grep ' W ')

        # `rust_begin_unwind` should have been overriden
        set +e
        echo "$weak_symbols" | grep rust_begin_unwind
        test $? -eq 0 && exit 1
        set -e
    done

    for profile in $profiles; do
        weak_symbols=$(arm-none-eabi-nm -g target/$target/$profile/examples/override-default-exception-handler | grep ' W ')

        # `_default_exception_handler` should have been overriden
        set +e
        echo "$weak_symbols" | grep _default_exception_handler
        test $? -eq 0 && exit 1
        set -e
    done

    # show the binary sizes of our examples
    arm-none-eabi-size target/$target/release/examples/*
}

main $1
