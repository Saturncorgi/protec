makepkg --printsrcinfo > .SRCINFO
git stage *
git commit
git switch master
git checkout main -- PKGBUILD .SRCINFO
git stage PKGBUILD .SRCINFO
git commit