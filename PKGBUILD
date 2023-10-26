# Maintainer: Brod8362 <brod8362@gmail.com>

pkgname=upowerchart-git
_pkgname=${pkgname%-git}
pkgver=0.0.0
pkgrel=00
pkgdesc="Simple upower chart"
arch=('any')
license=('MIT')
url='https://github.com/brod8362/upowerchart'
source=("$pkgname::git+$url.git")
sha256sums=('SKIP')
makedepends=(
    'cargo'
)
provides=(
    'upowerchart'
)

pkgver() {
  cd "$srcdir/$pkgname"

  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$srcdir/$pkgname"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/upowerchart"
}