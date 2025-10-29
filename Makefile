# Builds the program "bminor" from sources.
VERSION=1.0
NAME=rust-makefile
EXEC=rust-exec
PREFIX=$(HOME)/.local
SHOW_CORRECT := true

default: build

build:
	@cargo build --release
clean:
	@rm -rf target/*
	@cargo clean
	@rm test/encode/*.out || true
	@rm test/scanner/*.out || true
	@rm test/parser/*.out || true
.PHONY: test
test:
	@sh test/encode/runtest.sh $(SHOW_CORRECT)
	@sh test/scanner/runtest.sh $(SHOW_CORRECT)
	@sh test/parser/runtest.sh $(SHOW_CORRECT)