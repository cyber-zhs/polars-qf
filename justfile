#!/usr/bin/env just --justfile

set windows-shell := ["powershell"]

check:
  cargo fmt --check
  uv run ruff format --check
  cargo check
  cargo clippy
  uv run ruff check

fmt:
  cargo fmt
  uv run ruff format

test:
  uv run maturin develop --uv
  uv run pytest ./tests -s