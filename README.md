## Example of rustc wrapper that prints memory layout

How to run:

```bash
rustup toolchain install nightly-2025-05-10
rustup component add rustc-dev llvm-tools-preview --toolchain nightly
export RUSTFLAGS="-Zunstable-options --sysroot=$(rustc +nightly --print sysroot)"
cargo build -p rustc-wrapper
RUSTC_WRAPPER=./target/debug/rustc-wrapper cargo build -p example
```