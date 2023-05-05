## Faster linking

https://github.com/rui314/mold

```shell
sudo dnf install mold
```

```shell
nvim ~/.cargo/config.toml
```
```
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/bin/mold"]
```

## Linting
```shell
cargo clippy -- -D warnings
```

You can mute a warning using the `#[allow(clippy::lint_name)]`

## Formatting

```shell
cargo fmt
```

## Security Vulnerabilities
```shell
cargo install cargo-audit
```

## Macros expand

```shell
cargo install cargo-expand
```
```shell
rustup toolchain install nightly --allow-downgrade
```
```shell
cargo +nightly expand
```

