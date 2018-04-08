set -euxo pipefail

main() {
    if [ $TARGET = thumbv7em-none-eabihf ]; then
        rustup target add $TARGET
    fi
}

main
