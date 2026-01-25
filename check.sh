#!/bin/sh

set -e

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -A clippy::new_without_default -A clippy::collapsible_if -A clippy::empty_line_after_doc_comments -A clippy::doc_overindented_list_items -A clippy::unnecessary_to_owned -A clippy::needless_borrow -A clippy::vec_init_then_push -A clippy::unnecessary_map_or -A clippy::unwrap_or_default -A clippy::len_zero -A clippy::unusual_byte_groupings -A unused_imports -A dead_code -A clippy::assertions_on_constants -A clippy::needless_borrows_for_generic_args -A clippy::bool_assert_comparison
cargo build --verbose
cargo test --verbose -- --nocapture
wasm-pack test --node

# Build WASM for Node.js
wasm-pack build --target nodejs --out-dir pkg-nodejs

# Install ws module if needed
if [ ! -d "node_modules/ws" ]; then
    npm install ws
fi

# Run Node.js integration tests
node nodejs/test/integration.test.js

# Run Node.js timer tests
node nodejs/test/timer.test.js

# Run transport integration tests (new transport abstraction)
node nodejs/test/transport-integration.test.js
