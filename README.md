# IDS 706 Individual Project 2
[![Lint](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/lint.yml)
[![Build](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/build.yml/badge.svg)](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/build.yml)
[![Tests](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/actions/workflows/test.yml)

## Overview
* This repository includes the components for Individual Project 2 - Rust CLI Binary with SQLite

## Goal
* Implementing CRUD operations using Rust code on a SQLite database
* Using GitHub Copilot during the completion process
* Including a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded

## Key Elements in the Repo:
* `project/src/mylib/load_data.rs` (the Rust script to load the csv into a database)
* `project/src/mylib/operations.rs` (the Rust script for CRUD operations)
* `project/src/tests` (contains the respective tests for the written functions)
* `project/Development of Average Annual Wages_1.csv` (the csv file for the database)
* `project/Cargo.toml`
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

### CRUD in main.rs
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
<img width="589" alt="Screenshot 2023-10-29 at 2 17 09 PM" src="https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/assets/70416390/092ef11b-8284-4cac-93df-eb274763bf73">
(Successful CRUD operations)

## Optimized Rust Binary
 I've included a process that generates an optimized Rust binary as below, and you can download an artifact from GitHub Action's latest workflow run.
 ```
 name: Archive Binary
 uses: actions/upload-artifact@v2
 with:
    name: optimized-binary
    path: target/release/sqlite_operations
 ```

## Use of Copilot
I utilized Copilot to help me during the coding process. I started my repository with the same csv file and CRUD operations but in Python, and I was able to finish converting the whole repo to Rust-based with the help and hints from Copilot. Some examples of my Copilot usage are the following:

1. Help me convert the **create_wages_data** function to Rust
2. Write some test cases for the **read_wages_data_by_country** function
3. Explain this **type** error to me and provide some potential solutions

## User Guide to Run
1. Install Rust by running in your terminal
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Restart shell by running
```
source $HOME/.cargo/env
```
3. Check to see if Rust has been installed correctly
```
rustc --version
cargo --version
```
4. Go into the `project` folder by  
```
cd project
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
<img width="701" alt="Screenshot 2023-10-29 at 4 06 15 PM" src="https://github.com/nogibjj/Jaxon-Yue-Individual-Project-2/assets/70416390/ee97f010-7fa8-4b64-b0e5-9f44713b7b51">

## YouTube Introduction

## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
