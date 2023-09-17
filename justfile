set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# lints excessively with clippy
lint:
    @cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used
