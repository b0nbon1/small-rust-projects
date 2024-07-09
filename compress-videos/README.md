# Rust File Compressor

This Rust project provides a simple command-line utility to compress files using gzip compression.

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/rust-file-compressor.git
   cd rust-file-compressor
   ```
2. **Build the project:**
    ```bash
    cargo build --release
    ```

## Usage

To run the program, use the following command:
    ```bash
    cargo run --release -- <source> <target>
    ```

- `<source>`: Path to the source file to be compressed.
- `<target>`: Path to the target compressed file.


#### Example
```bash
cargo run --release -- source.txt target.gz
```

## Running Tests
To run the tests for this project, use the following command:

```bash
cargo test
```