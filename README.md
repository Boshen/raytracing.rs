# Raytracing in Rust 🦀
<a href="https://github.com/Boshen/raytracing.rs/actions/workflows/rust.yml">
  <img src="https://github.com/Boshen/raytracing.rs/actions/workflows/rust.yml/badge.svg"/>
</a>

Implementing ray tracing from the book [Ray Tracing from the Ground Up](https://www.amazon.com/Ray-Tracing-Ground-Kevin-Suffern/dp/1568812728)

Rendering artifects can be seen [here](https://github.com/Boshen/raytracing.rs/issues/1)

The primary goal of this project is to:
* toy with rust concepts while increasing performance
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

* Add Dependent Bot
* Separate Out Tracers (Chapter 14.5)
* Add more scenes: a lot of spheres
* Implement Regular Grids (Chapter 22)
* Implement static dispatch for Bvh. We need macro to implement `Bvh<L, R> { left: Bvh<L>, right: Bvh<R> }`
