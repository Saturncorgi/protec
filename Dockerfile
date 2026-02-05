# Base image
FROM docker.io/library/archlinux:multilib-devel

# Install dependencies
RUN pacman -Syu --needed --noconfirm pacman-contrib namcap git libinput alsa-lib rust evtest

# Setup user
RUN useradd -m builder && \
    echo 'builder ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers

WORKDIR /home/builder
USER builder

COPY --chmod=777 .github/workflows/validate.sh /validate.sh
RUN mkdir data
COPY --chmod=777 src data/src
COPY --chmod=777 assets data/assets
COPY --chmod=777 Cargo.toml data/Cargo.toml
COPY --chmod=777 Cargo.lock data/Cargo.lock
COPY --chmod=777 PKGBUILD.local data/PKGBUILD.local
COPY --chmod=777 LICENSE.md data/LICENSE.md
USER root
RUN chown -R builder  *
USER builder
# Set entrypoint
ENTRYPOINT ["/validate.sh"]