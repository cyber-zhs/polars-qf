#!/usr/bin/env just --justfile

set windows-shell := ["powershell"]

prepare:
  uv sync

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