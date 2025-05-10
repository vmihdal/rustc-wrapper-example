## Example of rustc wrapper that prints memory layout

How to run:

```bash
rustup toolchain install nightly-2025-05-10
rustup component add rustc-dev llvm-tools-preview --toolchain nightly
export RUSTFLAGS="-Zunstable-options --sysroot=$(rustc +nightly --print sysroot)"
cargo build -p rustc-wrapper
RUSTC_WRAPPER=./target/debug/rustc-wrapper cargo build -p example
```

Output:

```rust
TyAndLayout {
    ty: A,
    layout: Layout {
        size: Size(16 bytes),
        align: AbiAndPrefAlign {
            abi: Align(2 bytes),
            pref: Align(8 bytes),
        },
        backend_repr: Memory {
            sized: true,
        },
        fields: Arbitrary {
            offsets: [
                Size(14 bytes),
                Size(12 bytes),
                Size(0 bytes),
            ],
            memory_index: [
                2,
                1,
                0,
            ],
        },
        largest_niche: None,
        uninhabited: false,
        variants: Single {
            index: 0,
        },
        max_repr_align: None,
        unadjusted_abi_align: Align(2 bytes),
        randomization_seed: 3750484725686110678,
    },
}

[0] c: [u8; 12_usize], offset = 0, size = 12

[1] b: u16, offset = 12, size = 2

[2] a: u8, offset = 14, size = 1
```