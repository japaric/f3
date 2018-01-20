set -euxo pipefail

main() {
    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        bash gen-examples.sh
        git diff --exit-code

        cargo check --target $TARGET
        return
    fi

    xargo check --target $TARGET
    xargo check --target $TARGET --examples
}

main
