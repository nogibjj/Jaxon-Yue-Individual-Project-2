# IDS 706 Mini Project 8 [![Tests](https://github.com/nogibjj/Jaxon-Yue-Mini-Project-8/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Jaxon-Yue-Mini-Project-8/actions/workflows/tests.yml)

## Overview
* This repository includes the components for Mini-Project 8 - Rewrite a Python Script in Rust

## Goal
* Rewrite an existing Python script in Rust for data processing
* Highlight improvements in speed and resource usage

## Key Elements in the Repo:
* project/src (contains the Rust script and main function)
* project/tests (contains the respective tests)
* project/Cargo.toml
* project/dataset (contains the csv file)
* mylib/lib.py (contains the Python version)
* Makefile
* Dockerfile
* devcontainer
* GitHub Actions
* format.sh
* lint.sh
* test.sh
* bashrc

## Data Processing Script
* For this project, I wrote a script in Rust to get the 25%, 50%, and 75% percentile of column of the "Annual Wages" data table. Then, I wrote the same function in Python and compared the performance between Rust and Python.
* With the same data frame operation, Rust has beat the Python version during all my tries, both in run time and CPU and memory usage.
* <img width="555" alt="Screenshot 2023-10-28 at 6 04 33 PM" src="https://github.com/nogibjj/Jaxon-Yue-Mini-Project-8/assets/70416390/de64fe26-5af6-492c-8ad6-5a4aecbdfca8">
Rust script
* <img width="559" alt="Screenshot 2023-10-28 at 6 21 26 PM" src="https://github.com/nogibjj/Jaxon-Yue-Mini-Project-8/assets/70416390/b73ee2aa-8935-43e5-b287-90d04a2c00f2">
Python script

* While the Python version took about 3.87 ms, the Rust version only took about 1.10 ms, resulting in a 3.5x increase in speed.

## User Guide to Run
1. Fork the repository at **https://github.com/nogibjj/rust-data-engineering**
2. Install Rust by running
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Restart shell by running
```
source $HOME/.cargo/env
```
4. Check to see if Rust has been installed correctly
```
rustc --version
cargo --version
```
5. Now, you can run `cargo build` to compile your changes  
```
cargo build
```
6.  Run `cargo run` to test your modified tool 
```
cargo run
```

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```

I have passed all GitHub Actions as below:
<img width="684" alt="Screenshot 2023-10-28 at 6 22 32 PM" src="https://github.com/nogibjj/Jaxon-Yue-Mini-Project-8/assets/70416390/aeb9727d-d93a-41de-9cda-7ddc28940c90">


## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
