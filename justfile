# Finance Manager - Task Automation

# Variables
version := `grep '^version' Cargo.toml | cut -d'"' -f2`
app_name := "finance"

# Default recipe
default:
    @just --list

# Development tasks
dev:
    cargo run

test:
    cargo test

# Build tasks
build:
    cargo build

build-release:
    cargo build --release

# Install tasks
install:
    cargo install --path .

install-local:
    cargo install --path . --root ~/.local

# Version management
bump-patch:
    cargo bump patch
    git add Cargo.toml
    git commit -m "Bump version to $(cargo pkgid | cut -d'#' -f2)"

bump-minor:
    cargo bump minor
    git add Cargo.toml
    git commit -m "Bump version to $(cargo pkgid | cut -d'#' -f2)"

bump-major:
    cargo bump major
    git add Cargo.toml
    git commit -m "Bump version to $(cargo pkgid | cut -d'#' -f2)"

# Release tasks
release: build-release
    #!/bin/bash
    echo "Creating release for version {{version}}"
    mkdir -p releases
    cp target/release/{{app_name}} releases/{{app_name}}-{{version}}-linux-x86_64
    chmod +x releases/{{app_name}}-{{version}}-linux-x86_64
    echo "Release created: releases/{{app_name}}-{{version}}-linux-x86_64"

# Git tasks
tag:
    git tag v{{version}}
    git push origin v{{version}}

# Package tasks
package: build-release
    #!/bin/bash
    mkdir -p dist
    tar -czf dist/{{app_name}}-{{version}}-linux-x86_64.tar.gz -C target/release {{app_name}}
    echo "Package created: dist/{{app_name}}-{{version}}-linux-x86_64.tar.gz"

# Deployment tasks
deploy-local: install-local
    echo "Deployed to ~/.local/bin/{{app_name}}"

deploy-system: build-release
    sudo cp target/release/{{app_name}} /usr/local/bin/
    echo "Deployed to /usr/local/bin/{{app_name}}"

# Maintenance tasks
clean:
    cargo clean
    rm -rf releases/ dist/

update-deps:
    cargo update
    cargo audit

# Database tasks
backup-db:
    #!/bin/bash
    if [ -f "finance.db" ]; then
        cp finance.db "finance.db.backup.$(date +%Y%m%d_%H%M%S)"
        echo "Database backed up"
    else
        echo "No database found"
    fi

# Complete workflow
workflow: test build-release package
    echo "Complete workflow finished"
