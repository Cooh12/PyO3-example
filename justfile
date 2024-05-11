# Show help message
[private]
@default:
    just --list

# Install package with dependencies
@install:
    pdm install -G:all
    maturin build -m src/pyo3_example/rust/Cargo.toml

# Build Pyo3 lib
@build-rust-lib:
    maturin develop -m src/pyo3_example/rust/Cargo.toml

@run-benchmarks:
    python src/pyo3_example/fib/benchmarks.py
