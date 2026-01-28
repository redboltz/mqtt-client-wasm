#!/bin/sh

# Publish to npm
# Usage: ./publish_npm.sh [--dry-run]

set -e

DRY_RUN=""
if [ "$1" = "--dry-run" ]; then
    DRY_RUN="--dry-run"
    echo "Dry run mode - no actual publish will occur"
fi

echo "Building for browser (bundler target)..."
wasm-pack build --target bundler

echo ""
echo "Building for Node.js (nodejs target)..."
wasm-pack build --target nodejs --out-dir pkg-nodejs

# Remove .gitignore files that wasm-pack creates (they contain "*" which blocks npm pack)
echo ""
echo "Removing .gitignore files from pkg directories..."
rm -f pkg/.gitignore pkg-nodejs/.gitignore

echo ""
echo "Package contents:"
echo "=== pkg/ (browser) ==="
ls -la pkg/
echo ""
echo "=== pkg-nodejs/ (Node.js) ==="
ls -la pkg-nodejs/
echo ""
echo "=== nodejs/ (Node.js wrapper) ==="
ls -la nodejs/*.js

echo ""
echo "Root package.json:"
cat package.json

echo ""
if [ -n "$DRY_RUN" ]; then
    echo "Dry run - showing what would be published:"
    npm pack --dry-run
    echo ""
    echo "To publish for real, run: ./publish_npm.sh"
else
    echo "Publishing to npm..."
    npm publish --access public
    echo "Published successfully!"
fi

# Rebuild with web target for local www testing
echo ""
echo "Rebuilding with web target for local testing..."
wasm-pack build --target web
echo "Done. pkg/ now contains web target for www/ testing."
