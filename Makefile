COLOR ?= always
CARGO = cargo --color $(COLOR)

.PHONY = all build code_format quality

all: build

build:
	@$(CARGO) build

quality:
	@$(CARGO) clippy

code_format:
	@$(CARGO) fmt --all -- --check

