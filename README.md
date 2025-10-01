# wc-rust
A Rust implementation of the ```wc``` Linux command.
## Usage
```wc [flag] [filename]```

If no flag is provided, displays line, byte, and word count.

If no filename is provided, reads from standard input.

## Flags
- **l**: line count
- **m**: UTF-8 character count
- **c**: byte count
- **w**: word count
