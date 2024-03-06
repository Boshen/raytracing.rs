# Raytracing in Rust 🦀

[![ci](https://github.com/Boshen/raytracing.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Boshen/raytracing.rs/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Boshen/raytracing.rs/branch/main/graph/badge.svg?token=EG84H9PRFO)](https://codecov.io/gh/Boshen/raytracing.rs)

Implementing ray tracing from the book [Ray Tracing from the Ground Up](https://www.amazon.com/Ray-Tracing-Ground-Kevin-Suffern/dp/1568812728)

Rendering artifects can be seen [here](https://github.com/Boshen/raytracing.rs/issues/1)

The primary goal of this project is to:

* toy with rust concepts and increase performance
* implement interesting concepts from the book

## Development

```bash
cargo watch -x clippy -x 'run --release ' -s 'open output.png'
```

## Release

```bash
cargo run --release && open output.png
```

## Future Work

* Separate Out Tracers (Chapter 14.5)
* Add more scenes: a lot of spheres
* Implement Regular Grids (Chapter 22)
* Implement static dispatch for Bvh. We need macro to implement `Bvh<L, R> { left: Bvh<L>, right: Bvh<R> }`

# Progress Report

https://github.com/Boshen/raytracing.rs/issues/1
