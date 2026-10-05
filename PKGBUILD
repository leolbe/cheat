pkgname=cheat
pkgver=0.1.0
pkgrel=1
pkgdesc=''
license=('MIT')
makedepends=('cargo')
depends=('libgcc' 'glibc')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target host-tuple
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
