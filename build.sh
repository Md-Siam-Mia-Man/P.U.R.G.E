#!/bin/bash
# A single script to build all release artifacts for Linux and Windows.
# Must be run from a Linux environment or WSL on Windows.

set -e

# --- Configuration ---
PROJECT_NAME="purge"
VERSION=$(grep '^version =' Cargo.toml | head -n 1 | cut -d '"' -f 2)
RELEASE_DIR="releases"
echo "Starting unified release build for $PROJECT_NAME v$VERSION..."

# --- 1. Prerequisites ---
echo "Checking for required tools and targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
if ! command -v cargo-deb &> /dev/null; then cargo install cargo-deb; fi
if ! command -v zip &> /dev/null; then
    echo "zip command not found. Please install it (e.g., 'sudo apt-get install zip')."
    exit 1
fi
echo "Prerequisites are ready."

# --- 2. Cleanup and Setup ---
echo "Cleaning up old release directory..."
rm -rf $RELEASE_DIR target/release
mkdir -p $RELEASE_DIR

# --- 3. Build All Targets ---
echo "Building release binaries for all targets..."
cargo build --target x86_64-unknown-linux-gnu --release
cargo build --target x86_64-pc-windows-gnu --release
echo "All binaries built."

# --- 4. Package Linux Artifacts ---
echo ""
echo "=============================="
echo "Packaging Linux Artifacts..."
echo "=============================="

# 4a. Universal Binary Archive (.tar.gz)
LINUX_TAR_DIR="$RELEASE_DIR/linux_tar"
mkdir -p "$LINUX_TAR_DIR"
cp "target/x86_64-unknown-linux-gnu/release/$PROJECT_NAME" "$LINUX_TAR_DIR/"
cp README.md LICENSE.md "$LINUX_TAR_DIR/"
tar -czf "$RELEASE_DIR/${PROJECT_NAME}-v${VERSION}-linux-x86_64.tar.gz" -C "$LINUX_TAR_DIR" .
rm -rf "$LINUX_TAR_DIR"
echo "✔ Created: Universal .tar.gz"

# 4b. Debian Package (.deb)
cargo deb --target x86_64-unknown-linux-gnu --no-build
DEB_FILE=$(find target/x86_64-unknown-linux-gnu/debian/ -name "*.deb")
mv "$DEB_FILE" "$RELEASE_DIR/"
echo "✔ Created: Debian .deb package"

# --- 5. Package Source for Other Distros ---
echo ""
echo "=============================="
echo "Packaging Source Artifacts..."
echo "=============================="

# 5a. Arch Linux (PKGBUILD)
# This isn't a package, but a folder for the AUR maintainer
ARCH_DIR="$RELEASE_DIR/for_arch_aur"
mkdir -p "$ARCH_DIR"
cp PKGBUILD "$ARCH_DIR/"
echo "✔ Created: Arch Linux PKGBUILD"

# 5b. Fedora/RPM-based (.spec)
# This is for RPM Fusion/COPR maintainers
RPM_DIR="$RELEASE_DIR/for_fedora_copr"
mkdir -p "$RPM_DIR"
cp purge.spec "$RPM_DIR/"
echo "✔ Created: Fedora/RPM .spec file"


# --- 6. Package Windows Artifacts ---
echo ""
echo "=============================="
echo "Packaging Windows Artifacts..."
echo "=============================="
WINDOWS_ZIP_DIR="$RELEASE_DIR/windows_zip"
mkdir -p "$WINDOWS_ZIP_DIR"
cp "target/x86_64-pc-windows-gnu/release/${PROJECT_NAME}.exe" "$WINDOWS_ZIP_DIR/"
cp README.md LICENSE.md "$WINDOWS_ZIP_DIR/"
(cd "$WINDOWS_ZIP_DIR" && zip -r "../${PROJECT_NAME}-v${VERSION}-windows-x86_64.zip" .)
rm -rf "$WINDOWS_ZIP_DIR"
echo "✔ Created: Windows .zip package"


# --- 8. Final Summary ---
echo ""
echo "=============================="
echo "All builds completed successfully!"
echo "Release artifacts are in the '$RELEASE_DIR' directory."
echo "=============================="
ls -l $RELEASE_DIR