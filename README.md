TODO.py
===========

[![Build Status](https://img.shields.io/github/workflow/status/USER/TODO.py/CI/master)](https://github.com/USER/TODO.py/actions?query=workflow%3ACI)
[![Downloads](https://pepy.tech/badge/TODO)](https://pepy.tech/project/TODO)

Python bindings for Rust code.

## Installation

TODO.py is available for Python 3.7+ via pip:

```bash
$ pip install TODO
```

## Usage

```python
import TODO

result = TODO.my_function(123)
print(result)
```

## Development

*TODO.py* uses [PyO3](https://github.com/PyO3/pyo3) for binding Rust
to Python. It uses [Maturin](https://github.com/PyO3/maturin) to coerce the
Rust build into a `pip` and PyPI-compatible wheel.

Assuming that you have Rust and a relatively recent Python 3 installed,
the following should just work:

```bash
$ make develop
$ source env/bin/activate
```

A local build of *TODO.py* will be created and installed in your virtual environment.