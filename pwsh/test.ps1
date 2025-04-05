# build for CI
cargo build --verbose

# unigen
cargo test -- --nocapture
