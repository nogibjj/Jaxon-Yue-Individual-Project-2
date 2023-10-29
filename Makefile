rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cd project && cargo fmt —quiet

lint:
	cargo clippy --quiet

test:
	cd /workspaces/Jaxon-Yue-Individual-Project-2/project/ && cargo test --quiet

run:
	cargo run

release:
	cd /workspaces/Jaxon-Yue-Individual-Project-2/project/ && cargo build --release

install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

all: format lint test run