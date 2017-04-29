set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release
}

main
