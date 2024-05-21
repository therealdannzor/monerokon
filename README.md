# Monerkon Tari Workshop

## Get started

```shell
# Unfortunately, latest stable rust breaks the current
rustup install 1.77
rustup target add wasm32-unknown-unknown

cargo install cargo-generate
cargo generate https://github.com/sdbondi/monerokon.git exercise-1
cargo generate https://github.com/sdbondi/monerokon.git exercise-2
```