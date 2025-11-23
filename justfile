build:
  reset; cargo build

release:
  cargo build --release

run:
  cargo run -- --input ../netconfig.ncf --output-dir generated

run-err:
  cargo run -- --input ../netconfig-err.ncf

debug:
  RUST_LOG=debug cargo run

test:
  reset; RUST_LOG=info cargo test -- --show-output --test-threads=1 --nocapture

fmt:
  cargo fmt

