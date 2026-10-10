pkgname=cheat
pkgver=0.1.0
pkgrel=1
pkgdesc='Terminal cheatsheet viewer for text-based cheatsheets'
license=('MIT')
makedepends=('cargo')
arch=('i686' 'x86_64')

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --workspace --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "../target/release/$pkgname"
}
