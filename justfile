set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# build aspect
build:
    cargo build --release

# install or update aspect to default cargo location
install:
    cargo install --path .

# lints excessively with clippy
lint:
    @cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
