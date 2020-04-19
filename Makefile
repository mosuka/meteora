BIN_DIR ?= $(CURDIR)/bin
#DOCS_DIR ?= $(CURDIR)/docs
VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="meteora") | .version')
PROTO_VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="meteora-proto") | .version')
SERVER_VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="meteora-server") | .version')
CLIENT_VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="meteora-client") | .version')

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

tag:
	git tag v$(VERSION)
	git push origin v$(VERSION)

publish:
ifeq ($(shell cargo show --json meteora-proto | jq -r '.versions[].num' | grep $(PROTO_VERSION)),)
	(cd meteora-proto && cargo package && cargo publish)
	sleep 10
endif
ifeq ($(shell cargo show --json meteora-server | jq -r '.versions[].num' | grep $(SERVER_VERSION)),)
	(cd meteora-server && cargo package && cargo publish)
	sleep 10
endif
ifeq ($(shell cargo show --json meteora-client | jq -r '.versions[].num' | grep $(CLIENT_VERSION)),)
	(cd meteora-client && cargo package && cargo publish)
	sleep 10
endif
ifeq ($(shell cargo show --json meteora-client | jq -r '.versions[].num' | grep $(VERSION)),)
	(cd meteora && cargo package && cargo publish)
	sleep 10
endif

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
