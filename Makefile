rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

install:
	cargo install mdbook
	#install node
	#curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - &&\
	#sudo apt-get install -y nodejs
	#npm install -g @githubnext/github-copilot-cli
	
	echo 'eval "$(github-copilot-cli alias -- "$0")"' >> ~/.bashrc

format:
	@echo "Formatting all projects with cargo"
	./format.sh

lint:
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	./lint.sh

test:
	@echo "Testing all projects with cargo"
	./test.sh

run:
	cargo run

release:
	cd project/ && cargo build --release

all: format lint test run