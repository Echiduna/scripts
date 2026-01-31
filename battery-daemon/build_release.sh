#!/bin/bash
set -e

# Define variables
PROJECT_NAME="battery-daemon"
TARGET="x86_64-unknown-linux-musl"
RELEASE_DIR="target/$TARGET/release"
BINARY_NAME="battery-daemon"

echo "Building $PROJECT_NAME for $TARGET..."

# Ensure the target is added (idempotent)
rustup target add $TARGET

# Build release binary
# Note: Since .cargo/config.toml sets the default target,
# we could just run `cargo build --release`, but being explicit here doesn't hurt.
cargo build --release --target $TARGET

# Check if binary exists
if [ ! -f "$RELEASE_DIR/$BINARY_NAME" ]; then
    echo "Error: Binary not found at $RELEASE_DIR/$BINARY_NAME"
    exit 1
fi

# Create distribution tarball
VERSION=$(grep '^version' Cargo.toml | awk -F '"' '{print $2}')
DIST_NAME="$PROJECT_NAME-v$VERSION-$TARGET"
mkdir -p dist
cp "$RELEASE_DIR/$BINARY_NAME" dist/
tar -czf "dist/$DIST_NAME.tar.gz" -C dist "$BINARY_NAME"
rm dist/"$BINARY_NAME"

echo "Build complete!"
echo "Artifact: dist/$DIST_NAME.tar.gz"
