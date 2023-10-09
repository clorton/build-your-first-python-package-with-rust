# build-your-first-python-package-with-rust

https://www.youtube.com/watch?v=jp2EQiU9RxA

1. [install Rust](https://www.rust-lang.org/tools/install)
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - choose default install
2. For Pythonistas:
  - popular libraries used to create Rust + Python libraries
  - understand a Rust crate directory structure
  - how to navigate a Rust + Python project
  - become acquainted with Rust syntax
3. For Rustaceans:
  - popular libraries used to create Rust + Python libraries
  - how to navigate a Rust + Python project
  - inspiration on how to contribute to Python ecosystem
4. Python libraries with Rust core:
  - [`orjson`](https://pypi.org/project/orjson/) - "Fast, correct Python JSON library supporting dataclasses, datetimes, and numpy"
  - [`pydantic`](https://pypi.org/project/pydantic/) - "Data validation using Python type hints"
  - [`ruff`](https://pypi.org/project/ruff/) - "An extremely fast Python linter, written in Rust."
  - [`polars`](https://pypi.org/project/polars/) - "Blazingly fast DataFrame library"
5. Building Python + Rust Packages
  - packaging: making a **distributable artifact** that contains both Rust and Python code
  - calling Rust code from Python and vice versa (e.g., [`rust-numpy`](https://docs.rs/numpy/latest/numpy/) provides a Rust interface for numpy and [`tch-rs`](https://github.com/LaurentMazare/tch-rs) provides PyTorch bindings)
  - [`maturin`](https://www.maturin.rs/) - "Build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages"
    - lower level interfaces: [`setuptools-rust`](https://pypi.org/project/setuptools-rust/) or [`milksnake`](https://pypi.org/project/milksnake/)
  - [`PyO3`](https://pyo3.rs/) - Rust library to write a native Python module in Rust, or to embed Python in a Rust binary
    - `rust-cpython` - another option
6. Python >= 3.6 and Ryst >= 1.41
  - `pyenv`
  - `asdf`
  - `rtx` - Rust rewrite of `asdf`
7. `python3 -m venv .venv`
8. `source .venv/bin/activate`
9. `python3 -m pip install maturin`
10. `maturin new pymi-pyo3`
11. `cd pymi-pyo3`
12. main.py
13. `maturin develop`
14. `python3 ./main.py`

----

Reference: "[Build your Python Extension in Rust](https://pganssle-talks.github.io/pygotham-2019-rust-extensions/#/)"
