# Maintainer: Md. Siam Mia

pkgname=purge
pkgver=1.2.0
pkgrel=1
pkgdesc="A fast, native GUI for removing bloatware from Android devices."
arch=('x86_64')
url="https://github.com/Md-Siam-Mia-Man/UAD-Universal-Android-Debloater"
license=('MIT')
depends=('gtk3' 'libappindicator-gtk3')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP') # Run 'updpkgsums' to generate this

prepare() {
    cd "$pkgname-$pkgver"
}

build() {
    cargo build --release --locked
}

package() {
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "LICENSE.md" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}