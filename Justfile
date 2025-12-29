# Justfile for Larch

set shell := ["bash", "-c"]

default:
    @just --list

# Run the app in development mode
dev:
    pnpm tauri dev

# Run all checks (clippy, svelte-check)
check: check-rust check-svelte

# Run Rust clippy
check-rust:
    cd src-tauri && cargo clippy -- -D warnings
    cd crates/taiga-client && cargo clippy -- -D warnings

# Run Svelte check
check-svelte:
    pnpm check

# Run all tests
test: test-rust

# Run Rust tests
test-rust:
    cd src-tauri && cargo test
    cd crates/taiga-client && cargo test

# Format code
format: format-rust format-svelte

# Format Rust code
format-rust:
    cd src-tauri && cargo fmt
    cd crates/taiga-client && cargo fmt

# Format Svelte code
format-svelte:
    pnpm format

# Update dependencies
update:
    pnpm up --latest
    cd src-tauri && cargo update
    cd crates/taiga-client && cargo update
