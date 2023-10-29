# IDS 706 Individual Project 2

## Overview
* This repository includes the components for Individual Project 2 - Rust CLI Binary with SQLite

## Goal
* Implementing CRUD operations using Rust code on a SQLite database
* Use of GitHub Copilot during the completion process
* Including a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded

## Key Elements in the Repo:
* project/src/mylib/load_data.rs (the Rust script to load the csv into a database)
* project/src/mylib/operations.rs (the Rust script for CRUD operations)
* project/src/tests (contains the respective tests for the written functions)
* project/Development of Average Annual Wages_1.csv (the csv file for the database)
* project/Cargo.toml
* Makefile
* Dockerfile
* devcontainer
* GitHub Actions
* format.sh
* lint.sh
* test.sh
* bashrc

## CRUD Operations
Functions in `project/src/mylib/operations.rs`:
* `create_wages_data` CREATE: insert a new country's data
* `read_wages_data_by_country` READ: read a given country's data
* `update_wages_data` UPDATE: update a given country's data
* `delete_wages_data` DELETE: delete a given country's data

### CRUD in main.py
* **Load csv data into SQLite database**:
`load("Development of Average Annual Wages_1.csv")`

* **Create new entry for the country China**:
`create_wages_data("China", 10000, 15000, 20000, 22000)`

* **Update Iceland's data**:
`update_wages_data("Iceland", 20000, 25000, 30000, 32000)`

* **Print China's data**
`read_wages_data_by_country("China")`

* **Delete the data for China**
`delete_wages_data("China")`

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
