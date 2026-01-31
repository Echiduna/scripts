# GitHub Release Plan for Battery Daemon

This document outlines the steps to create a binary release for the `battery-daemon`.

## Prerequisites

1.  **GitHub Repository**: Ensure you have a GitHub repository set up for this project.
2.  **Git Tag**: Releases are tied to git tags.

## Automated Release Workflow (GitHub Actions)

The most robust way to handle releases is using GitHub Actions. Create a file at `.github/workflows/release.yml` in your repository with the following content. This will automatically build the `musl` binary and upload it when you push a new tag (e.g., `v0.1.0`).

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build Release Asset
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-unknown-linux-musl

      - name: Build
        working-directory: battery-daemon
        run: cargo build --release --target x86_64-unknown-linux-musl

      - name: Package
        working-directory: battery-daemon
        run: |
          mkdir -p dist
          cp target/x86_64-unknown-linux-musl/release/battery-daemon dist/
          cd dist
          tar -czf battery-daemon-x86_64-unknown-linux-musl.tar.gz battery-daemon

      - name: Release
        uses: softprops/action-gh-release@v1
        if: startsWith(github.ref, 'refs/tags/')
        with:
          files: battery-daemon/dist/battery-daemon-x86_64-unknown-linux-musl.tar.gz
```

## Manual Release Steps

If you prefer to release manually, follow these instructions:

### 1. Update Version
Ensure `Cargo.toml` has the correct version number.
```toml
[package]
version = "0.1.0"
```

### 2. Commit and Tag
Commit your changes and create a git tag.
```bash
git add .
git commit -m "chore: bump version to 0.1.0"
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### 3. Build Artifact
Run the provided helper script to build the `musl` binary and create a tarball.
```bash
cd battery-daemon
./build_release.sh
```
This will create a file like `dist/battery-daemon-v0.1.0-x86_64-unknown-linux-musl.tar.gz`.

### 4. Create Release on GitHub
1.  Go to your GitHub repository page.
2.  Click on "Releases" -> "Draft a new release".
3.  Choose the tag `v0.1.0`.
4.  Title the release "v0.1.0".
5.  Write a description of the changes.
6.  **Attach the Binary**: Drag and drop the `.tar.gz` file generated in Step 3 into the "Attach binaries..." box.
7.  Click "Publish release".
