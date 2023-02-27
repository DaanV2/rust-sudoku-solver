# Sudoku Solver

[![🦀 Checks](https://github.com/DaanV2/rust-sudoku-solver/actions/workflows/rust.yml/badge.svg)](https://github.com/DaanV2/rust-sudoku-solver/actions/workflows/rust.yml)


## Assembly

To see assembly:

```bash
cargo rustc --release -- --emit asm

# then check:
target/release/deps/<crate_name>-<hash>.s
```