#!/bin/sh

# Publish to npm
# Usage: ./publish_npm.sh [--dry-run]

set -e

DRY_RUN=""
if [ "$1" = "--dry-run" ]; then
    DRY_RUN="--dry-run"
    echo "Dry run mode - no actual publish will occur"
fi

echo "Building for npm (bundler target)..."
wasm-pack build --target bundler --scope redboltz

echo ""
echo "Package contents:"
ls -la pkg/

echo ""
echo "package.json:"
cat pkg/package.json

echo ""
if [ -n "$DRY_RUN" ]; then
    echo "Dry run - skipping actual publish"
    echo "To publish for real, run: ./publish_npm.sh"
else
    echo "Publishing to npm..."
    cd pkg
    npm publish --access public
    cd ..
    echo "Published successfully!"
fi

# Rebuild with web target for local www testing
echo ""
echo "Rebuilding with web target for local testing..."
wasm-pack build --target web
echo "Done. pkg/ now contains web target for www/ testing."
