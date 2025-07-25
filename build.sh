# build.sh
#!/bin/bash

# A script to build release binaries for Linux and Windows.
# Must be run from a Linux environment or WSL on Windows.

# Exit immediately if a command exits with a non-zero status.
set -e

# --- Configuration ---
PROJECT_NAME="universal-android-debloater"
# Get version from Cargo.toml to avoid hardcoding it
VERSION=$(grep '^version =' Cargo.toml | head -n 1 | cut -d '"' -f 2)
RELEASE_DIR="releases"

# --- Target Definitions ---
LINUX_TARGET="x86_64-unknown-linux-gnu"
WINDOWS_TARGET="x86_64-pc-windows-gnu"

echo "Starting release build for $PROJECT_NAME v$VERSION..."

# --- 1. Prerequisites Check ---
echo "Checking for required Rust targets..."
rustup target add $LINUX_TARGET
rustup target add $WINDOWS_TARGET
echo "Targets are ready."

# --- 2. Cleanup and Setup ---
echo "Cleaning up old release directory..."
rm -rf $RELEASE_DIR
mkdir -p $RELEASE_DIR
echo "Created fresh release directory: $RELEASE_DIR"

# --- 3. Build for Linux ---
echo ""
echo "=============================="
echo "Building for Linux (x86_64)..."
echo "=============================="
cargo build --target=$LINUX_TARGET --release

# --- 4. Package for Linux ---
echo "Packaging Linux artifact..."
LINUX_BUILD_DIR="$RELEASE_DIR/linux_x86_64"
mkdir -p "$LINUX_BUILD_DIR"

cp "target/$LINUX_TARGET/release/$PROJECT_NAME" "$LINUX_BUILD_DIR/"
cp README.md LICENSE.md "$LINUX_BUILD_DIR/"

LINUX_ARCHIVE_NAME="${PROJECT_NAME}-v${VERSION}-linux-x86_64.tar.gz"
# The -C flag is important. It creates the archive without the parent directory structure.
tar -czf "$RELEASE_DIR/$LINUX_ARCHIVE_NAME" -C "$LINUX_BUILD_DIR" .

echo "Linux package created: $RELEASE_DIR/$LINUX_ARCHIVE_NAME"


# --- 5. Build for Windows ---
echo ""
echo "=============================="
echo "Building for Windows (x86_64)..."
echo "=============================="
cargo build --target=$WINDOWS_TARGET --release

# --- 6. Package for Windows ---
echo "Packaging Windows artifact..."
WINDOWS_BUILD_DIR="$RELEASE_DIR/windows_x86_64"
mkdir -p "$WINDOWS_BUILD_DIR"

cp "target/$WINDOWS_TARGET/release/${PROJECT_NAME}.exe" "$WINDOWS_BUILD_DIR/"
cp README.md LICENSE.md "$WINDOWS_BUILD_DIR/"

WINDOWS_ARCHIVE_NAME="${PROJECT_NAME}-v${VERSION}-windows-x86_64.zip"
# Create a subshell to change directory, create the zip, then return.
(cd "$WINDOWS_BUILD_DIR" && zip -r "../$WINDOWS_ARCHIVE_NAME" .)

echo "Windows package created: $RELEASE_DIR/$WINDOWS_ARCHIVE_NAME"


# --- 7. Cleanup Intermediate Files ---
echo ""
echo "Cleaning up intermediate build directories..."
rm -rf "$LINUX_BUILD_DIR"
rm -rf "$WINDOWS_BUILD_DIR"
echo "Cleanup complete."


# --- 8. Final Summary ---
echo ""
echo "=============================="
echo "All builds completed successfully!"
echo "Release artifacts are in the '$RELEASE_DIR' directory."
echo "=============================="
ls -l $RELEASE_DIR