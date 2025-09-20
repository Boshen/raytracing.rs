# Raytracing development tasks

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# List all available tasks
default:
    @just --list --unsorted

# Initialize the development environment
init:
    cargo install cargo-watch cargo-llvm-cov cargo-codspeed
    git config core.hooksPath .githooks

# Install git pre-commit hooks
install-hook:
    #!/usr/bin/env bash
    set -euxo pipefail
    mkdir -p .githooks
    echo '#!/bin/sh' > .githooks/pre-commit
    echo 'just fmt-check' >> .githooks/pre-commit
    chmod +x .githooks/pre-commit

# Run all checks (ready for commit)
ready: fmt lint test

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format all code
fmt:
    cargo fmt --all

# Run clippy
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test --release

# Run tests with coverage
coverage:
    cargo llvm-cov --lcov --output-path lcov.info

# Watch for changes and run tests
watch-test:
    cargo watch -x test

# Watch for changes and run clippy
watch-clippy:
    cargo watch -x clippy

# Watch for changes and render
watch-render:
    cargo watch -x clippy -x 'run --release' -s 'open output.png'

# Run benchmarks
bench:
    cargo bench --features codspeed

# Run a quick preview render
preview width="100" height="100":
    cargo run --release -- --preview --width {{width}} --height {{height}}
    @echo "Output saved to output.png"

# Run a production quality render
render width="500" height="500" samples="16":
    cargo run --release -- --width {{width}} --height {{height}} --samples {{samples}}
    @echo "Output saved to output.png"

# Run a high quality render
high-quality width="1000" height="1000" samples="64":
    cargo run --release -- --width {{width}} --height {{height}} --samples {{samples}}
    @echo "High quality render saved to output.png"

# Profile a render (macOS)
profile-render width="200" height="200" samples="4":
    #!/usr/bin/env bash
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sudo cargo run --release -- --width {{width}} --height {{height}} --samples {{samples}}
        echo "Use Instruments.app to profile the process"
    else
        echo "Profiling recipe is macOS-specific"
    fi

# Clean build artifacts
clean:
    cargo clean

# Clean and rebuild
fresh: clean build

# Check project health
check:
    cargo check --all-targets --all-features

# Update dependencies
update:
    cargo update

# Generate documentation
docs:
    cargo doc --no-deps --open

# Run specific benchmark
bench-specific name:
    cargo bench --features codspeed --bench {{name}}

# Test with different sample counts
test-samples:
    #!/usr/bin/env bash
    echo "Testing different sample counts..."
    for samples in 1 4 16; do
        echo "=== Testing with $samples samples ==="
        time cargo run --release -- --preview --width 100 --height 100 --samples $samples
    done

# Compare render times
compare-performance:
    #!/usr/bin/env bash
    echo "=== Performance Comparison ==="
    echo "Preview mode (1 sample):"
    time cargo run --release -- --preview --width 200 --height 200
    echo
    echo "Production mode (4 samples):"
    time cargo run --release -- --width 200 --height 200 --samples 4
    echo
    echo "High quality (16 samples):"
    time cargo run --release -- --width 200 --height 200 --samples 16

# Install system dependencies (Ubuntu/Debian)
install-deps-ubuntu:
    sudo apt-get update
    sudo apt-get install -y pkg-config libssl-dev

# Install system dependencies (macOS)
install-deps-macos:
    brew install openssl pkg-config

# Create a development build and run
dev-render width="200" height="200":
    cargo build
    cargo run -- --preview --width {{width}} --height {{width}}

# Package for release
package:
    cargo build --release
    strip target/release/raytracing 2>/dev/null || true
    @echo "Release binary: target/release/raytracing"

# Show current performance on standard test
perf-test:
    #!/usr/bin/env bash
    echo "Running standard performance test..."
    echo "Image: 300x300, Samples: 4"
    time cargo run --release -- --width 300 --height 300 --samples 4
    ls -lh output.png