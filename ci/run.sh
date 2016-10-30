set -ex

main() {
    local target=$1
    local profiles="debug release"

    if [ $target = x86_64-unknown-linux-gnu ]; then
        cargo build

        # test layout of register maps
        cargo run --example _test

        return
    fi

    # test that disabling default features work
    xargo build --target $target --no-default-features
    xargo build --target $target --no-default-features --example minimal

    local flags=
    for example in $(ls examples); do
        case $example in
            _test.rs | *.rs.bk)
                continue
                ;;
            override-default-exception-handler.rs)
                flags="--target $target --example ${example%.*} --no-default-features --features \"compiler-builtins-snapshot/memcpy default-init default-panic-fmt\""
                ;;
            override-init.rs)
                flags="--target $target --example ${example%.*} --no-default-features --features \"compiler-builtins-snapshot/memcpy default-exception-handler default-panic-fmt\""
                ;;
            override-panic-fmt.rs)
                flags="--target $target --example ${example%.*} --no-default-features --features \"compiler-builtins-snapshot/memcpy default-exception-handler default-init\""
                continue
                ;;
            *)
                flags="--target $target --example ${example%.*}"
        esac

        eval "xargo build $flags"
        eval "xargo build $flags --release"
    done

    # show the binary sizes of our examples
    arm-none-eabi-size target/$target/release/examples/*
}

main $1
