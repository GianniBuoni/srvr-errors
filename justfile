lint:
  cargo fmt --check
  cargo clippy --all-targets -- -D warnings

test:
  cargo test

build: test
  cargo build
