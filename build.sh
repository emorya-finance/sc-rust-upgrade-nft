# pip install abi-markdowner
cargo fmt && cargo clippy && sc-meta all build && sc-meta all proxy && abi-markdowner --output-file README.md