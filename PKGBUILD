# Maintainer: SpikyGames <spikygames123@gmail.com>
pkgname=calcula2r
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple calculator written in Rust with GTK4."
arch=('x86_64')
url="https://github.com/Squar-DE/calcula2r"
license=('GPL3')
depends=('gtk4' 'rust' 'cargo')
makedepends=('git')
source=("git+https://github.com/Squar-DE/calcula2r.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 "target/release/calcula2r" "$pkgdir/usr/bin/calcula2r"
}

