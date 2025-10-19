# Bevy PSX

A Bevy plugin that provides authentic PlayStation 1 (PSX) style rendering capabilities, including low-resolution rendering, vertex snapping, and palette quantization.

## Features

- **Low-Resolution Rendering**: Classic PSX resolutions (320x240, 512x448, etc.)
- **Vertex Snapping**: Authentic PSX-style geometry jitter
- **Palette Quantization**: Optional retro color limitations (off by default)
- **Pixelated Upscaling**: Nearest-neighbor or linear filtering
- **Automatic Material Conversion**: Works seamlessly with standard Bevy materials

## Quick Start

```rust
use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,  // Enables PSX rendering!
    ));
}
```

## Configuration

### Render Settings

```rust
fn configure_psx(mut settings: ResMut<PsxRenderSettings>) {
    settings.render_resolution = UVec2::new(320, 240);
    settings.pixelated = true;
    settings.aspect_ratio_matching = true;
}
```

### Vertex Snapping

```rust
fn configure_vertex_snapping(mut settings: ResMut<PsxSettings>) {
    settings.snap_enabled = true;
    settings.snap_amount = 64.0;  // Lower = more jittery
}
```

**Snap Amount Guide:**
- `8.0-32.0`: Extreme jitter (very wobbly)
- `64.0`: Classic PSX feel (recommended)
- `128.0+`: Subtle effect

### Palette Quantization (Optional)

```rust
fn configure_palette(mut settings: ResMut<PsxSettings>) {
    settings.use_palette = true;
    settings.quantize_steps = 32;
}
```

## Examples

### Basic Demo
```bash
cargo run --example simple_psx
```

### Advanced Lighting
```bash
cargo run --example lights
```
The most comprehensive example with physical camera, multiple light types, and all PSX features.

### Performance Test
```bash
cargo run --example object_spawner
```
Stress test with up to 1000 objects.

## Common Controls

**Rendering:**
- `1-4`: Switch resolutions
- `A`: Toggle aspect ratio matching
- `F`: Toggle pixelated filtering

**PSX Effects:**
- `V/B`: Adjust vertex snapping
- `P`: Toggle palette quantization
- `N/M`: Switch palettes
- `T`: Toggle vertex snapping

*Note: Each example may have slightly different controls - check console output when running.*

## Common Resolutions

- **PSX**: 320×240 (4:3)
- **PS2**: 512×448 (8:7)
- **SNES**: 256×224 (8:7)

## License

MIT License

## Compatibility

- **Bevy**: 0.17.2
- **Platforms**: All Bevy-supported platforms