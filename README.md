## Faster linking


On Windows

```shell
nvim ~/.cargo/config.toml
```
```

cargo install -f cargo-binutils
rustup component add llvm-tools-preview

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

Fedora 

```shell
sudo dnf install mold
```
https://github.com/rui314/mold
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

