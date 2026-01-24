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
    cargo build --release
}

package(){
    cd "$srcdir/.."
    install -Dm744 -t "$pkgdir"/usr/bin target/release/$pkgname
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/fezaaa.jpg
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/fezprotec.png
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/NOOOOOOO.wav
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/AAA.wav
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/what.wav
    install -Dm644 -t "$pkgdir/etc/$pkgname/" assets/yournotagoodperson.wav
}