set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        bash gen-examples.sh
        git diff --exit-code

        return
    fi

    cargo check --target $TARGET --examples
}

main
