# Pao
Fast GPU-accelerated 2D renderer built on top of wgpu

> [!WARNING]
> Pao is still a work in progress. The API may change.

[![Crates.io](https://img.shields.io/crates/v/pao.svg)](https://crates.io/crates/pao)
[![Documentation](https://docs.rs/pao/badge.svg)](https://docs.rs/pao)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- [x] GPU-accelerated rendering with wgpu
- [x] Extensible feature system
- [x] Cross-platform
- [ ] Shape rendering (rectangles, circles) - Coming in v0.1.0
- [ ] Text rendering support - Coming in v0.2.0
- [ ] Gradients and advanced effects - Coming in v0.3.0

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
pao = "0.1"
```

## The Feature System

Pao doesn't have everything built in, so if you are missing something you can create custom feature

### Creating a Custom Feature
```rust
use pao::features::RenderFeature;

pub struct TriangleFeature {
    pipeline: pao::wgpu::RenderPipeline,
}

impl RenderFeature for TriangleFeature {
    fn prepare(&mut self, device: &pao::wgpu::Device, queue: &pao::wgpu::Queue) {
    
    }
    
    fn render(&mut self, pass: &mut pao::wgpu::RenderPass) {
        pass.set_pipeline(&self.pipeline);
        pass.draw(0..3, 0..1);
    }
}

// Use it:
canvas.draw_feature(Box::new(TriangleFeature::new(
    canvas.get_device(),
    canvas.get_surface_config(),
)));
```

## Examples

Check out the [examples](https://github.com/dest-hq/Pao/tree/main/examples/) directory:

- [basic_winit](https://github.com/dest-hq/Pao/tree/main/examples/basic_winit/) - Simple colored window
- [triangle](https://github.com/dest-hq/Pao/tree/main/examples/triangle/) - Custom triangle feature with shaders and anti-aliasing (MSAA 4X)

Run an example:
```bash
cargo run --release -p <the example>
```

## Roadmap

### v0.1.0
- [x] Canvas + wgpu setup
- [x] Feature system
- [x] Basic examples

### v0.2.0
- [ ] Rectangle rendering
- [ ] Circle rendering
- [ ] Rounded rectangles
- [ ] Lines and strokes
- [ ] Text rendering
- [ ] Gradients

### v0.3.0
- [ ] Images and textures
- [ ] Clipping and masking
- [ ] Transforms (rotate, scale, translate)
- [ ] Performance optimizations

You can find out more here [Roadmap](https://github.com/orgs/dest-hq/projects/3/)

## Comparison

| Feature | Pao | Vello | Skia | tiny_pao |
|---------|-----|-------|------|----------|
| Backend | GPU | GPU | GPU/CPU | CPU only |
| Memory | Medium | High on Windows | Low | Very low |
| Extensible | Yes | No | No | Yes |

## Contributing

Contributions are welcome! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where help is needed:
- Shape rendering implementations
- Performance optimizations
- Documentation and examples
- Testing on different platforms

## License

MIT License - see [LICENSE](LICENSE) for details.
