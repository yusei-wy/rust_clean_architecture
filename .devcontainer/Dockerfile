# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld cmake \
   && apt-get autoremove -y && apt-get clean -y

# Add vscode user before `cargo install` or error will occur
# https://github.com/rust-lang/cargo/issues/6513#issuecomment-1133415695
USER vscode

RUN cargo install diesel_cli
