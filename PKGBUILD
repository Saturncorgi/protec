pkgname=protec
pkgver=0.1.0
pkgrel=1
pkgdesc="protec"
arch=(x86_64)
depends=('cargo' 'libinput' 'evtest' 'alsa-lib')
license=(custom:for_good_eyes_only)
prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}
build(){
cargo build --release --offline
}
package(){
    cd "$srcdir/.."
    install -Dm711 -t "$pkgdir"/usr/bin target/release/$pkgname
    install -Dm644 -t "$pkgdir"/usr/share/licenses/$pkgname $srcdir/../LICENSE.md
}