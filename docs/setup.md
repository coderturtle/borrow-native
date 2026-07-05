# Setup

This project follows the Hekton reproducible setup standard:

```text
documented -> scripted -> idempotent-ish -> logged -> reproducible on a blank machine
```

## Intended Flow

```bash
./scripts/install-rust-toolchain.sh   # cargo/rustc/clippy/rustfmt via Homebrew
./scripts/check-prereqs.sh
./scripts/bootstrap-project.sh --dry-run
./scripts/bootstrap-project.sh
./scripts/verify-project.sh
```

## Project-Specific Steps

- **Rust toolchain required.** `scripts/install-rust-toolchain.sh` installs cargo/rustc/clippy/rustfmt via `brew install rust` (this factory's install policy: prefer Homebrew, avoid opaque installers like `curl | sh`). `scripts/check-prereqs.sh` verifies all four are present. This script is scoped to this project for now; it's written portably enough to promote to `~/hekton/bootstrap/` if a second Hekton project ever needs a Rust toolchain too.
- Each module's exercise, once authored, is a real `cargo` crate under that module's directory, verified by `cargo test` and `cargo clippy` (the deterministic gate — see `docs/workshop-design.md`).

