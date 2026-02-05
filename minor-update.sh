makepkg --printsrcinfo > .SRCINFO
git stage *
git switch master
git checkout main -- PKGBUILD .SRCINFO
git stage PKGBUILD .SRCINFO
git commit
git push