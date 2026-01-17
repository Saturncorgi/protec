pkgname=protec
pkgver=0.1.0
pkgrel=1
pkgdesc="protec"
arch=(x86_64)
depends=('cargo')
license=(custom:for_good_eyes_only)
prepare() {
    cd "$srcdir"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}
build(){
    cd "$srcdir"
    cargo build --release --offline
}
package(){
    cd "$srcdir/.."
    install -Dm744 -t "$pkgdir"/usr/bin target/release/$pkgname
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/fezprotec.png
}