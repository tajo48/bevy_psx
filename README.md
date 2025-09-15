# Bevy PSX

A Bevy plugin that provides authentic PSX-style low resolution rendering with vertex snapping, palette quantization, and retro visual effects.

## Features

🎮 **Authentic PSX Rendering**
- Low resolution rendering with automatic aspect ratio matching
- Maintains retro aesthetic while preventing stretching
- Automatic MSAA disabling for pixel-perfect rendering
- Configurable render resolutions and base resolutions

✨ **Visual Effects**
- **Vertex Snapping**: Characteristic PSX vertex jittering effect
- **Palette Quantization**: Color reduction with custom palettes
- **Pixelated Upscaling**: Nearest-neighbor filtering for sharp pixels
- **Aspect Ratio Matching**: Automatically adjusts resolution to match window aspect ratio

🔧 **Easy Integration**
- Drop-in component system - just add `PsxCamera` to any camera
- Automatic material conversion from `StandardMaterial` to PSX materials
- Runtime configuration through resources

🎨 **Palette System**
- Load custom color palettes from `.hex` files
- Built-in palettes (Game Boy, Lospec 2000, etc.)
- Embedded palette support for easy distribution

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.16"
bevy_psx = "0.1"
```

Basic setup:

```rust
use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)  // Add the PSX plugin
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Just add PsxCamera component to any camera!
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,  // This enables PSX rendering automatically
    ));

    // Your regular 3D scene setup...
    // All StandardMaterial meshes will automatically get PSX vertex snapping!
}
```

That's it! Your camera now renders at PSX resolution with vertex snapping applied to all 3D models.

## Examples

Run the examples to see the plugin in action:

```bash
# Basic PSX rendering with interactive controls
cargo run --example simple_psx

# Complex scene with multiple objects and animations
cargo run --example rotating_scene
```

### Example Controls (simple_psx)

- **1-3**: Switch between different resolutions (PSX/PS2/High)
- **R**: Toggle pixelated/smooth filtering
- **V/B**: Increase/decrease vertex snapping amount
- **T**: Toggle vertex snapping on/off
- **P**: Toggle palette quantization
- **Q/E**: Adjust quantization steps

## Configuration

### PSX Render Settings

```rust
fn configure_rendering(mut settings: ResMut<PsxRenderSettings>) {
    // Set base PSX resolution (used for aspect ratio calculations)
    settings.base_resolution = UVec2::new(320, 240);
    
    // Enable automatic aspect ratio matching (on by default)
    settings.aspect_ratio_matching = true;

    // Enable pixelated upscaling
    settings.pixelated = true;
}
```

### Vertex Snapping Settings

```rust
fn configure_vertex_snapping(mut settings: ResMut<PsxVertexSnapSettings>) {
    // Control vertex snapping intensity
    settings.snap_amount = 64.0;  // Lower = more jittery, higher = smoother
    settings.enabled = true;
}
```

### Palette Quantization Settings

```rust
fn configure_palette(mut settings: ResMut<PsxPaletteSettings>) {
    // Color quantization steps (lower = more posterized)
    settings.quantize_steps = 32;

    // Enable palette-based color reduction
    settings.use_palette = true;
    settings.enabled = true;
}
```

## Palette System

### Built-in Palettes

The plugin comes with embedded palettes that you can enable/disable:

```rust
// In materials.rs - AVAILABLE_PALETTES configuration
define_palettes!(
    (false, "Game Boy", "../assets/palettes/gameboy.hex"),
    (true, "Lospec 2000", "../assets/palettes/lospec-2000.hex"),
    // Add your own palettes here...
);
```

### Custom Palette Format

Create `.hex` palette files:

```hex
# My Custom PSX Palette
# Comments are supported
000000  # Black
FFFFFF  # White
FF0000  # Red
00FF00  # Green
0000FF  # Blue
# Supports 3-digit (RGB) and 6-digit (RRGGBB) hex colors
```

### Loading Palettes at Runtime

```rust
fn load_custom_palette(mut palette_manager: ResMut<PaletteManager>) {
    // Load from file
    palette_manager.load_palette_from_hex("assets/my_palette.hex").unwrap();

    // Load from embedded string
    let hex_data = "000000\nFFFFFF\nFF0000";
    palette_manager.load_palette_from_embedded_hex(hex_data, "My Palette").unwrap();
}
```

## How It Works

### Rendering Pipeline

1. **PSX Camera Setup**: Cameras with `PsxCamera` component render to a low-resolution texture
2. **Material Conversion**: `StandardMaterial` meshes are automatically converted to `PsxPaletteMaterial`
3. **Vertex Snapping**: Custom vertex shader snaps vertices to create PSX-style jittering
4. **Color Quantization**: Fragment shader reduces colors using quantization and palette mapping
5. **Upscaling**: Low-res texture is upscaled to window size with configurable filtering

### Automatic Features

- **MSAA Disabling**: Automatically disabled on PSX cameras for authentic pixel-perfect rendering
- **Material Conversion**: Standard materials are automatically converted to PSX materials with vertex snapping
- **Palette Loading**: Built-in palettes are loaded automatically at startup

## Technical Details

### Aspect Ratio Matching

The plugin automatically adjusts render resolution to match your window's aspect ratio while maintaining the retro aesthetic:

- **Wide windows** (16:9, 21:9): Keeps height constant, adjusts width (e.g., 320×240 → 427×240)
- **Tall windows** (portrait): Keeps width constant, adjusts height (e.g., 320×240 → 320×427)
- **Square windows** (1:1): Uses base resolution unchanged

This prevents stretching while maintaining low resolution rendering.

### Supported Base Resolutions

Common retro console resolutions for `base_resolution`:
- **PSX**: 320×240 (default)
- **PS2**: 512×448
- **N64**: 320×240
- **SNES**: 256×224
- **Custom**: Any resolution you specify

### Vertex Snapping

The vertex snapping effect works by:
1. Transforming vertices to clip space
2. Converting to normalized device coordinates (NDC)
3. Quantizing X and Y coordinates based on snap amount
4. Converting back to clip space

Higher snap amounts (128-512) produce smoother results, while lower amounts (16-64) create more authentic PSX jittering.

### Aspect Ratio Algorithm

The aspect ratio matching system:
1. Compares window aspect ratio to base resolution aspect ratio
2. If window is wider: `new_width = base_height × window_aspect_ratio`
3. If window is taller: `new_height = base_width ÷ window_aspect_ratio`
4. Automatically updates render targets when window is resized
5. Can be toggled on/off via `PsxRenderSettings.aspect_ratio_matching`

### Palette Quantization

Two modes of color reduction:
1. **Basic Quantization**: Reduces color bit depth (posterization effect)
2. **Palette Mapping**: Maps colors to nearest color in loaded palette

## Performance

The plugin is designed to be performant:
- Low-resolution rendering reduces pixel fill rate (scales with aspect ratio)
- Automatic material batching through Bevy's material system
- Minimal overhead for vertex snapping shader
- Efficient palette lookup in fragment shader
- Aspect ratio calculations only run on window resize events

## Compatibility

- **Bevy Version**: 0.16+
- **Platforms**: All platforms supported by Bevy
- **Renderers**: Compatible with Bevy's default renderer

## Contributing

Contributions are welcome! Areas for improvement:
- Additional built-in palettes
- More PSX-style effects (dithering, texture warping)
- Performance optimizations
- Platform-specific optimizations

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Gallery

*Screenshots and videos of the plugin in action would go here*

## Acknowledgments

- Inspired by the authentic PSX rendering techniques
- Built with the amazing [Bevy Engine](https://bevyengine.org/)
- Palette system inspired by retro graphics communities

---

**Made with ❤️ for the retro game development community**
