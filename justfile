set dotenv-load := false
set positional-arguments := false

#export RUST_BACKTRACE := "full"
#export RUST_BACKTRACE := "1"

_default:
    @just --list

# Create an optimized 'release' build
@build:
    cargo build --release

# Format, lint and check that project compiles
@compile:
    cargo test --no-run --locked

# Format the project with rustfmt
@format:
    cargo fix
    cargo clippy --fix
    cargo fmt --all

# Quickly format and run linter
@lint:
    cargo fmt --all
    cargo clippy

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test

# Run tests with the 'nocapture' and 'quiet' flags enabled
@test:
    cargo test -- --nocapture --quiet

# Run tests without parallelism to assist debugging concurrency issues
@test-debug:
    cargo test -- --test-threads=1 --nocapture

