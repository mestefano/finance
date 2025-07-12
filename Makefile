# Finance Manager Makefile

APP_NAME := finance
VERSION := $(shell grep '^version' Cargo.toml | cut -d'"' -f2)

.PHONY: all build test install clean release

all: build

# Development
dev:
	cargo run

test:
	cargo test

# Build
build:
	cargo build

build-release:
	cargo build --release

# Install
install:
	cargo install --path .

install-local:
	cargo install --path . --root ~/.local

install-system: build-release
	sudo cp target/release/$(APP_NAME) /usr/local/bin/

# Version management
bump-patch:
	@echo "Current version: $(VERSION)"
	sed -i 's/^version = .*/version = "'$(shell echo $(VERSION) | awk -F. '{print $$1"."$$2"."$$3+1}')'"/g' Cargo.toml
	@echo "New version: $(shell grep '^version' Cargo.toml | cut -d'"' -f2)"

bump-minor:
	@echo "Current version: $(VERSION)"
	sed -i 's/^version = .*/version = "'$(shell echo $(VERSION) | awk -F. '{print $$1"."$$2+1".0"}')'"/g' Cargo.toml
	@echo "New version: $(shell grep '^version' Cargo.toml | cut -d'"' -f2)"

# Release
release: build-release
	mkdir -p releases
	cp target/release/$(APP_NAME) releases/$(APP_NAME)-$(VERSION)-linux-x86_64
	chmod +x releases/$(APP_NAME)-$(VERSION)-linux-x86_64
	@echo "Release created: releases/$(APP_NAME)-$(VERSION)-linux-x86_64"

package: build-release
	mkdir -p dist
	tar -czf dist/$(APP_NAME)-$(VERSION)-linux-x86_64.tar.gz -C target/release $(APP_NAME)
	@echo "Package created: dist/$(APP_NAME)-$(VERSION)-linux-x86_64.tar.gz"

# Maintenance
clean:
	cargo clean
	rm -rf releases/ dist/

update:
	cargo update

# Database
backup-db:
	@if [ -f "finance.db" ]; then \
		cp finance.db "finance.db.backup.$$(date +%Y%m%d_%H%M%S)"; \
		echo "Database backed up"; \
	else \
		echo "No database found"; \
	fi

# Complete workflow
workflow: test build-release package
	@echo "Complete workflow finished for version $(VERSION)"
