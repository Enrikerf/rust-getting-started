.DEFAULT_GOAL = help

## —— 📦 The amazing Makefile 📦 —————————————————————————————————————————
help: ## Outputs this help screen
	@grep -E '(^[a-zA-Z0-9_-]+:.*?##.*$$)|(^##)' Makefile | awk 'BEGIN {FS = ":.*?## "}{printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'

## —— cargo —————————————————————————————————————————
run:## build and run the program
	cargo run
build:## Build the program
	cargo build
check:## check the program
	cargo build
release:## release the program
	cargo build --release

## ——  Rust  —————————————————————————————————————————
rust-cli: ## rustc $path will compile the program
	rustc --version #cargo new --vcs=git hello2 will create .gitignore if you are inside a git repo already

