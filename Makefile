BIN_DIR ?= $(CURDIR)/bin
#DOCS_DIR ?= $(CURDIR)/docs
VERSION ?=

ifeq ($(VERSION),)
  VERSION = $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="meteora") | .version')
endif

.DEFAULT_GOAL := build

clean:
	rm -rf $(BIN_DIR)
	cargo clean

format:
	cargo fmt

build:
	mkdir -p $(BIN_DIR)
	cargo build --release
	cp -p ./target/release/meteora $(BIN_DIR)

test:
	cargo test

docker-build:
	docker build -t mosuka/meteora:latest .
	docker tag mosuka/meteora:latest mosuka/meteora:$(VERSION)

docker-push:
	docker push mosuka/meteora:latest
	docker push mosuka/meteora:$(VERSION)

docker-clean:
	docker rmi -f $(shell docker images --filter "dangling=true" -q --no-trunc)

#.PHONY: docs
#docs:
#	rm -rf $(DOCS_DIR)
#	mdbook build
