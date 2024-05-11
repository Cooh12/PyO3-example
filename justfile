# Show help message
[private]
@default:
    just --list

# Install package with dependencies
@install:
    pip install maturin
    maturin build -m src/pyo3_example/rust/Cargo.toml

@run-benchmarks:
    python src/pyo3_example/fib/benchmarks.py
