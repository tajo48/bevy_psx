# Bevy PSX

A Bevy plugin that provides authentic PlayStation 1 (PSX) style rendering capabilities, including low-resolution rendering, vertex snapping, and palette quantization.

## Features

- **Low-Resolution Rendering**: Render your 3D scenes at classic PSX resolutions (320x240, 512x448, etc.)
- **Vertex Snapping**: Authentic PSX-style vertex jittering that recreates the characteristic "wobbly" geometry
- **Palette Quantization**: Apply color palettes to achieve retro color limitations
- **Aspect Ratio Matching**: Automatically adjusts resolution to match your window's aspect ratio while maintaining the retro aesthetic
- **Pixelated Upscaling**: Choose between nearest-neighbor (pixelated) or linear filtering for the final output
- **Multiple Palettes**: Load and switch between different color palettes at runtime
- **Automatic Material Conversion**: Seamlessly converts standard Bevy materials to PSX-enhanced materials

## Quick Start

Add the plugin to your Bevy app:

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
    // Spawn a camera with PSX rendering
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,  // This component enables PSX rendering!
    ));
    
    // Your 3D objects will automatically get PSX vertex snapping and palette quantization
}
```

That's it! Your camera will now render at PSX resolution with automatic vertex snapping and palette quantization applied to all 3D models.

## Configuration

### Render Settings

Control the PSX rendering behavior with `PsxRenderSettings`:

```rust
fn configure_psx(mut settings: ResMut<PsxRenderSettings>) {
    // Set resolution
    settings.render_resolution = UVec2::new(320, 240);  // Classic PSX
    settings.base_resolution = UVec2::new(320, 240);
    
    // Enable/disable features
    settings.pixelated = true;              // Nearest-neighbor filtering
    settings.aspect_ratio_matching = true;  // Adjust to window aspect ratio
}
```

### Vertex Snapping

Control vertex snapping with `PsxVertexSnapSettings`:

```rust
fn configure_vertex_snapping(mut settings: ResMut<PsxVertexSnapSettings>) {
    settings.enabled = true;
    settings.snap_amount = 64.0;  // Lower = more jittery, Higher = smoother
}
```

### Palette Quantization

Control color palettes with `PsxPaletteSettings`:

```rust
fn configure_palette(mut settings: ResMut<PsxPaletteSettings>) {
    settings.enabled = true;
    settings.use_palette = true;      // Use loaded color palettes
    settings.quantize_steps = 32;     // Color reduction steps
}
```

## Controls

The plugin includes built-in keyboard controls (can be seen in the examples):

- **1, 2, 3**: Switch between preset resolutions
- **A**: Toggle aspect ratio matching
- **R**: Toggle pixelated/smooth filtering  
- **V/B**: Increase/decrease vertex snap amount
- **T**: Toggle vertex snapping on/off
- **P**: Toggle palette quantization on/off
- **Q/E**: Decrease/increase quantization steps
- **N/M**: Switch between loaded palettes

## Palettes

The plugin automatically loads color palettes from the `assets/palettes/` directory. Supported formats:

- **Hex files (.hex)**: Each line contains a hex color (with or without #)

Example palette file:
```
# My Custom Palette
000000
ffffff
ff0000
00ff00
0000ff
```

Built-in palettes include:
- Game Boy (4 colors)
- Lospec 2000 (various colors)
- Axulart 32-color palette
- And more!

## Examples

### Simple PSX Scene
```bash
cargo run --example simple_psx
```

### Rotating Scene with Multiple Objects
```bash
cargo run --example rotating_scene  
```

### Stress Test (Performance Testing)
```bash
cargo run --example object_spawner
```

## Technical Details

### Rendering Pipeline

1. Your 3D scene renders to a low-resolution texture
2. Vertex snapping is applied during vertex processing
3. Fragment colors are quantized using the active palette
4. The low-res texture is upscaled to fill your window
5. Filtering (pixelated vs smooth) is applied during upscaling

### Automatic Material Conversion

The plugin automatically converts standard Bevy `StandardMaterial`s to PSX-enhanced materials with:
- Vertex snapping in the vertex shader
- Palette quantization in the fragment shader
- Maintains all original material properties (textures, colors, etc.)

### Aspect Ratio Matching

When enabled, the plugin automatically adjusts the render resolution based on your window's aspect ratio:

- **Wide windows (16:9, 21:9)**: Keeps height at base resolution, adjusts width
- **Tall windows**: Keeps width at base resolution, adjusts height  
- **Square windows**: Uses base resolution as-is

This prevents stretching while maintaining the low-resolution aesthetic.

## Common Resolutions

- **PSX**: 320×240 (4:3)
- **PS2**: 512×448 (8:7, close to 4:3)
- **SNES**: 256×224 (8:7)
- **Custom**: Any resolution you want!

## Performance

The PSX rendering adds minimal overhead:
- Low-resolution rendering actually improves performance
- Vertex snapping is GPU-accelerated
- Palette quantization is done in fragment shaders
- Automatic material conversion happens once per material

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Compatibility

- **Bevy Version**: 0.17.1
- **Platforms**: All platforms supported by Bevy
- **Rendering**: Compatible with Bevy's PBR pipeline

## Acknowledgments

Inspired by the distinctive visual style of original PlayStation games and the technical limitations that created their unique aesthetic.