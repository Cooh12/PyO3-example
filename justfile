
@default:
    just --list

@install:
    pip install maturin
    maturin develop -m src/pyo3_example/rust/Cargo.toml
    pip install ./src/pyo3_example/rust/

@bench:
    python src/pyo3_example/fib/benchmarks.py
