# Ray Tracing Project Instructions

## Project Overview
This is a Ray Tracing implementation in Rust based on "Ray Tracing from the Ground Up" by Kevin Suffern. The project focuses on performance optimization and implementing interesting ray tracing concepts.

## Key Commands
- Build and run: `cargo run --release && open output.png`
- Development watch: `cargo watch -x clippy -x 'run --release' -s 'open output.png'`
- **Ready for commit**: `just ready` (runs fmt + clippy + tests)
- Format code: `cargo fmt`
- Check lints: `cargo clippy`
- Run tests: `cargo test`
- Run benchmarks: `cargo bench --features codspeed`

## Code Structure
- `src/` - Core ray tracing implementation
  - `accelerator/` - BVH acceleration structures
  - `brdf/` - Bidirectional Reflectance Distribution Functions
  - `camera/` - Camera implementations
  - `geometric_object/` - Geometric primitives (spheres, meshes, etc.)
  - `light/` - Lighting implementations
  - `material/` - Material definitions
  - `scene/` - Scene management
  - `renderer.rs` - Main rendering logic
  - `sampler.rs` - Sampling strategies
  - `ray.rs` - Ray definition and operations

## Development Guidelines
1. Performance is a priority - use release builds for testing rendering
2. The project uses strict Clippy lints - ensure code passes `cargo clippy`
3. **Always run `cargo fmt` after making code changes** to maintain consistent formatting
4. Linear algebra operations use nalgebra
5. Parallel processing uses rayon
6. Output is always `output.png` in the project root

## Testing Approach
- Unit tests are located alongside source files
- Integration tests for complete rendering pipelines
- Benchmark tests in `benches/` directory

## Common Tasks
- Adding new geometric objects: Implement in `src/geometric_object/`
- Adding new materials: Implement in `src/material/`
- Adding new scenes: Modify `src/scene/`
- Optimizing performance: Consider using rayon for parallelization, check BVH implementation

## Important Notes
- The project uses custom allocators (jemalloc on Unix, mimalloc on Windows) for performance
- Release profile is optimized with LTO and single codegen unit
- Rendering artifacts and progress are documented in GitHub issue #1