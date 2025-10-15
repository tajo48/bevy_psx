use bevy::{
    asset::uuid_handle,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
    shader::ShaderRef,
};

use crate::palette::PaletteManager;

pub const PSX_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("12345678-1234-5678-90ab-cdef01234567");

/// Combined PSX material extension that includes both vertex snapping and unified shader effects
///
/// This extension combines all PSX rendering effects into a single material:
/// - Vertex snapping (creates characteristic PSX vertex jittering)
/// - Basic color quantization (reduces color depth)
/// - Palette quantization (maps colors to a limited palette)
/// - Multiple dithering patterns (Bayer 4x4/8x8, blue noise, random)
/// - Different color space calculations (RGB, HSV, LAB approximation)
/// - Error diffusion and blending modes
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PsxMaterialExtension {
    // Vertex snapping uniforms
    /// Controls how much vertex snapping occurs.
    /// Higher values = less snapping (smoother), lower values = more snapping (jittery)
    /// Typical PSX values: 64.0 - 256.0
    #[uniform(100)]
    pub snap_amount: f32,

    /// Whether vertex snapping is enabled (1 to enable, 0 to disable)
    #[uniform(100)]
    pub snap_enabled: u32,

    // Unified shader uniforms
    /// Number of quantization steps for basic color reduction (0 to disable)
    /// Higher values = smoother gradients, lower values = more posterized
    /// Typical PSX values: 16 - 64
    #[uniform(100)]
    pub quantize_steps: u32,

    /// Whether basic quantization is enabled (1 to enable, 0 to disable)
    #[uniform(100)]
    pub quantize_enabled: u32,

    /// Whether to use the PSX palette for color quantization (1 to enable, 0 to disable)
    #[uniform(100)]
    pub use_palette: u32,

    /// Number of colors in the current palette (for shader optimization)
    #[uniform(100)]
    pub palette_size: u32,

    /// The actual palette colors (up to 256 colors supported)
    #[uniform(100)]
    pub palette_colors: [Vec3; 256],

    /// Whether dithering is enabled (1 to enable, 0 to disable)
    #[uniform(100)]
    pub dither_enabled: u32,

    /// Dithering strength (0.0 = no dithering, 1.0 = full dithering)
    /// Typical values: 0.1 - 0.5
    #[uniform(100)]
    pub dither_strength: f32,

    /// Dither pattern selection (0 = Bayer 4x4, 1 = Bayer 8x8, 2 = Blue noise, 3 = Random)
    #[uniform(100)]
    pub dither_pattern: u32,

    /// Color space for distance calculations (0 = RGB, 1 = HSV, 2 = LAB)
    #[uniform(100)]
    pub color_space: u32,

    /// Error diffusion mode (0 = disabled, 1 = Floyd-Steinberg approximation)
    #[uniform(100)]
    pub error_diffusion: u32,

    /// Blend mode (0 = replace, 1 = blend with original)
    #[uniform(100)]
    pub blend_mode: u32,

    /// Blend factor when blend_mode = 1 (0.0 = original color, 1.0 = quantized color)
    #[uniform(100)]
    pub blend_factor: f32,
}

/// Dither pattern options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DitherPattern {
    Bayer4x4 = 0,
    Bayer8x8 = 1,
    BlueNoise = 2,
    Random = 3,
}

/// Color space options for palette matching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    RGB = 0,
    HSV = 1,
    LAB = 2,
}

/// Blend mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Replace = 0,
    Blend = 1,
}

impl PsxMaterialExtension {
    /// Create a new PSX material extension with default PSX-like settings
    pub fn new() -> Self {
        Self::default()
    }

    // Vertex snapping methods
    /// Enable/disable vertex snapping
    pub fn set_snap_enabled(&mut self, enabled: bool) {
        self.snap_enabled = if enabled { 1 } else { 0 };
    }

    /// Get vertex snapping enabled state
    pub fn get_snap_enabled(&self) -> bool {
        self.snap_enabled != 0
    }

    // Quantization methods
    /// Enable/disable basic quantization
    pub fn set_quantize_enabled(&mut self, enabled: bool) {
        self.quantize_enabled = if enabled { 1 } else { 0 };
    }

    /// Get basic quantization enabled state
    pub fn get_quantize_enabled(&self) -> bool {
        self.quantize_enabled != 0
    }

    /// Set palette enabled state
    pub fn set_use_palette(&mut self, enabled: bool) {
        self.use_palette = if enabled { 1 } else { 0 };
    }

    /// Get palette enabled state
    pub fn get_use_palette(&self) -> bool {
        self.use_palette != 0
    }

    // Dithering methods
    /// Enable/disable dithering
    pub fn set_dither_enabled(&mut self, enabled: bool) {
        self.dither_enabled = if enabled { 1 } else { 0 };
    }

    /// Get dithering enabled state
    pub fn get_dither_enabled(&self) -> bool {
        self.dither_enabled != 0
    }

    /// Set dither pattern
    pub fn set_dither_pattern(&mut self, pattern: DitherPattern) {
        self.dither_pattern = pattern as u32;
    }

    /// Get dither pattern
    pub fn get_dither_pattern(&self) -> DitherPattern {
        match self.dither_pattern {
            1 => DitherPattern::Bayer8x8,
            2 => DitherPattern::BlueNoise,
            3 => DitherPattern::Random,
            _ => DitherPattern::Bayer4x4,
        }
    }

    // Color space methods
    /// Set color space for palette matching
    pub fn set_color_space(&mut self, color_space: ColorSpace) {
        self.color_space = color_space as u32;
    }

    /// Get color space
    pub fn get_color_space(&self) -> ColorSpace {
        match self.color_space {
            1 => ColorSpace::HSV,
            2 => ColorSpace::LAB,
            _ => ColorSpace::RGB,
        }
    }

    // Error diffusion methods
    /// Enable/disable error diffusion
    pub fn set_error_diffusion(&mut self, enabled: bool) {
        self.error_diffusion = if enabled { 1 } else { 0 };
    }

    /// Get error diffusion enabled state
    pub fn get_error_diffusion(&self) -> bool {
        self.error_diffusion != 0
    }

    // Blend mode methods
    /// Set blend mode
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode as u32;
    }

    /// Get blend mode
    pub fn get_blend_mode(&self) -> BlendMode {
        match self.blend_mode {
            1 => BlendMode::Blend,
            _ => BlendMode::Replace,
        }
    }

    /// Update palette from a Palette object
    pub fn update_palette(&mut self, palette: &crate::palette::Palette) {
        let palette_array = palette.to_shader_array(256);
        self.palette_size = palette_array.len() as u32;

        for (i, &color) in palette_array.iter().enumerate() {
            self.palette_colors[i] = color;
        }
    }

    /// Quick PSX-like preset with dithering and vertex snapping
    pub fn psx_preset(&mut self) {
        // Vertex snapping
        self.set_snap_enabled(true);
        self.snap_amount = 64.0;

        // Fragment effects
        self.quantize_steps = 32;
        self.set_quantize_enabled(true);
        self.set_use_palette(true);
        self.set_dither_enabled(true);
        self.dither_strength = 0.2;
        self.set_dither_pattern(DitherPattern::Bayer4x4);
        self.set_color_space(ColorSpace::RGB);
        self.set_error_diffusion(false);
        self.set_blend_mode(BlendMode::Replace);
        self.blend_factor = 1.0;
    }

    /// Game Boy-like preset
    pub fn gameboy_preset(&mut self) {
        // Vertex snapping
        self.set_snap_enabled(true);
        self.snap_amount = 32.0;

        // Fragment effects
        self.quantize_steps = 4;
        self.set_quantize_enabled(true);
        self.set_use_palette(true);
        self.set_dither_enabled(true);
        self.dither_strength = 0.3;
        self.set_dither_pattern(DitherPattern::Bayer4x4);
        self.set_color_space(ColorSpace::RGB);
        self.set_error_diffusion(false);
        self.set_blend_mode(BlendMode::Replace);
        self.blend_factor = 1.0;
    }

    /// High quality preset with error diffusion
    pub fn high_quality_preset(&mut self) {
        // Vertex snapping
        self.set_snap_enabled(true);
        self.snap_amount = 128.0;

        // Fragment effects
        self.quantize_steps = 64;
        self.set_quantize_enabled(true);
        self.set_use_palette(true);
        self.set_dither_enabled(true);
        self.dither_strength = 0.15;
        self.set_dither_pattern(DitherPattern::BlueNoise);
        self.set_color_space(ColorSpace::LAB);
        self.set_error_diffusion(true);
        self.set_blend_mode(BlendMode::Replace);
        self.blend_factor = 1.0;
    }
}

impl Default for PsxMaterialExtension {
    fn default() -> Self {
        // Create a default PSX-style palette
        let mut palette_colors = [Vec3::ZERO; 256];
        let default_colors = [
            Vec3::new(0.0, 0.0, 0.0),    // Black
            Vec3::new(1.0, 1.0, 1.0),    // White
            Vec3::new(1.0, 0.0, 0.0),    // Red
            Vec3::new(0.0, 1.0, 0.0),    // Green
            Vec3::new(0.0, 0.0, 1.0),    // Blue
            Vec3::new(1.0, 1.0, 0.0),    // Yellow
            Vec3::new(1.0, 0.0, 1.0),    // Magenta
            Vec3::new(0.0, 1.0, 1.0),    // Cyan
            Vec3::new(0.5, 0.5, 0.5),    // Gray
            Vec3::new(0.25, 0.25, 0.25), // Dark Gray
            Vec3::new(0.75, 0.75, 0.75), // Light Gray
            Vec3::new(0.5, 0.0, 0.0),    // Dark Red
            Vec3::new(0.0, 0.5, 0.0),    // Dark Green
            Vec3::new(0.0, 0.0, 0.5),    // Dark Blue
            Vec3::new(0.5, 0.25, 0.0),   // Brown
            Vec3::new(0.25, 0.5, 0.25),  // Dark Green
        ];

        for (i, &color) in default_colors.iter().enumerate() {
            palette_colors[i] = color;
        }

        Self {
            // Vertex snapping defaults
            snap_amount: 64.0,
            snap_enabled: 1,

            // Fragment shader defaults
            quantize_steps: 32,
            quantize_enabled: 1,
            use_palette: 0,
            palette_size: default_colors.len() as u32,
            palette_colors,
            dither_enabled: 1,
            dither_strength: 0.2,
            dither_pattern: DitherPattern::Bayer4x4 as u32,
            color_space: ColorSpace::RGB as u32,
            error_diffusion: 0,
            blend_mode: BlendMode::Replace as u32,
            blend_factor: 1.0,
        }
    }
}

impl MaterialExtension for PsxMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        PSX_MATERIAL_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        PSX_MATERIAL_SHADER_HANDLE.into()
    }
}

/// Type alias for PSX materials with all effects combined
pub type PsxMaterial = ExtendedMaterial<StandardMaterial, PsxMaterialExtension>;

/// Resource to configure PSX material settings globally
#[derive(Resource, Debug, Clone)]
pub struct PsxSettings {
    // Vertex snapping settings
    /// Global snap amount for all PSX materials
    pub snap_amount: f32,
    /// Whether vertex snapping is enabled
    pub snap_enabled: bool,

    // Fragment shader settings
    /// Global quantization steps for all PSX materials
    pub quantize_steps: u32,
    /// Whether basic quantization is enabled
    pub quantize_enabled: bool,
    /// Whether palette quantization is enabled
    pub use_palette: bool,
    /// Whether dithering is enabled
    pub dither_enabled: bool,
    /// Dithering strength (0.0 - 1.0)
    pub dither_strength: f32,
    /// Dither pattern selection
    pub dither_pattern: DitherPattern,
    /// Color space for palette matching
    pub color_space: ColorSpace,
    /// Whether error diffusion is enabled
    pub error_diffusion: bool,
    /// Blend mode for final output
    pub blend_mode: BlendMode,
    /// Blend factor when blend_mode is Blend
    pub blend_factor: f32,
}

impl Default for PsxSettings {
    fn default() -> Self {
        Self {
            // Vertex snapping defaults
            snap_amount: 64.0,
            snap_enabled: true,

            // Fragment shader defaults
            quantize_steps: 32,
            quantize_enabled: true,
            use_palette: false,
            dither_enabled: true,
            dither_strength: 0.2,
            dither_pattern: DitherPattern::Bayer4x4,
            color_space: ColorSpace::RGB,
            error_diffusion: false,
            blend_mode: BlendMode::Replace,
            blend_factor: 1.0,
        }
    }
}

// Legacy type aliases for backwards compatibility
pub type PsxVertexSnapSettings = PsxSettings;
pub type PsxUnifiedSettings = PsxSettings;
pub type PsxUnifiedMaterial = PsxMaterial;
pub type PsxUnifiedShaderExtension = PsxMaterialExtension;
pub type PsxVertexSnapExtension = PsxMaterialExtension;

/// System to update PSX material settings when settings change
pub fn update_psx_material_settings(
    psx_settings: Res<PsxSettings>,
    palette_manager: Option<Res<PaletteManager>>,
    mut psx_materials: ResMut<Assets<PsxMaterial>>,
) {
    let settings_changed = psx_settings.is_changed();
    let palette_changed = palette_manager.as_ref().map_or(false, |pm| pm.is_changed());

    if !settings_changed && !palette_changed {
        return;
    }

    for (_handle, material) in psx_materials.iter_mut() {
        if settings_changed {
            // Update vertex snapping settings
            material.extension.snap_amount = psx_settings.snap_amount;
            material
                .extension
                .set_snap_enabled(psx_settings.snap_enabled);

            // Update fragment shader settings
            material.extension.quantize_steps = psx_settings.quantize_steps;
            material
                .extension
                .set_quantize_enabled(psx_settings.quantize_enabled);
            material.extension.set_use_palette(psx_settings.use_palette);
            material
                .extension
                .set_dither_enabled(psx_settings.dither_enabled);
            material.extension.dither_strength = psx_settings.dither_strength;
            material
                .extension
                .set_dither_pattern(psx_settings.dither_pattern);
            material.extension.set_color_space(psx_settings.color_space);
            material
                .extension
                .set_error_diffusion(psx_settings.error_diffusion);
            material.extension.set_blend_mode(psx_settings.blend_mode);
            material.extension.blend_factor = psx_settings.blend_factor;
        }

        if palette_changed {
            if let Some(palette_manager) = &palette_manager {
                if let Some(current_palette) = palette_manager.current_palette() {
                    material.extension.update_palette(current_palette);
                }
            }
        }
    }
}

/// System to show palette loading information
pub fn show_palette_info(palette_manager: Res<PaletteManager>, mut has_shown: Local<bool>) {
    // Only show info if palettes are loaded and we haven't shown it yet
    if *has_shown || palette_manager.len() == 0 {
        return;
    }

    *has_shown = true;

    info!("=== PSX Palette System ===");

    if let Some(current) = palette_manager.current_palette() {
        let name = current.name.as_deref().unwrap_or("Unknown");
        info!("Loaded palette: {} ({} colors)", name, current.len());
        info!(
            "Using palette with {} colors for quantization",
            current.len()
        );
    }
    info!("==========================");
}

/// System to convert StandardMaterials to PsxMaterials for entities that don't already have PSX materials
pub fn convert_standard_materials_to_psx(
    mut commands: Commands,
    meshes_with_standard_materials: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Without<MeshMaterial3d<PsxMaterial>>,
    >,
    standard_material_assets: Res<Assets<StandardMaterial>>,
    mut psx_material_assets: ResMut<Assets<PsxMaterial>>,
    psx_settings: Res<PsxSettings>,
    palette_manager: Option<Res<PaletteManager>>,
) {
    let use_psx = psx_settings.snap_enabled
        || psx_settings.quantize_enabled
        || psx_settings.use_palette
        || psx_settings.dither_enabled;

    // Exit early if PSX effects are not enabled
    if !use_psx {
        return;
    }

    for (entity, material_handle) in meshes_with_standard_materials.iter() {
        if let Some(standard_material) = standard_material_assets.get(&material_handle.0) {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>();

            let mut extension = PsxMaterialExtension::default();

            // Apply global settings
            extension.snap_amount = psx_settings.snap_amount;
            extension.set_snap_enabled(psx_settings.snap_enabled);
            extension.quantize_steps = psx_settings.quantize_steps;
            extension.set_quantize_enabled(psx_settings.quantize_enabled);
            extension.set_use_palette(psx_settings.use_palette);
            extension.set_dither_enabled(psx_settings.dither_enabled);
            extension.dither_strength = psx_settings.dither_strength;
            extension.set_dither_pattern(psx_settings.dither_pattern);
            extension.set_color_space(psx_settings.color_space);
            extension.set_error_diffusion(psx_settings.error_diffusion);
            extension.set_blend_mode(psx_settings.blend_mode);
            extension.blend_factor = psx_settings.blend_factor;

            // Update with current palette if available
            if let Some(palette_manager) = &palette_manager {
                if let Some(current_palette) = palette_manager.current_palette() {
                    extension.update_palette(current_palette);
                }
            }

            let psx_material = PsxMaterial {
                base: standard_material.clone(),
                extension,
            };

            let handle = psx_material_assets.add(psx_material);
            commands.entity(entity).insert(MeshMaterial3d(handle));
        }
    }
}

// Macro to generate the palette loading code
macro_rules! define_palettes {
    ($(($id:expr, $name:expr, $path:expr)),* $(,)?) => {
        /// Configuration for available palettes with IDs
        const AVAILABLE_PALETTES: &[(u32, &str, &str)] = &[
            // (id, name, file_path)
            $(($id, $name, $path),)*
        ];

        /// Helper function to get embedded palette data - automatically generated
        fn get_embedded_palette_data(file_path: &str) -> Option<&'static str> {
            match file_path {
                $($path => Some(include_str!($path)),)*
                _ => None,
            }
        }
    };
}

// Define your palettes here - this is the ONLY place you need to edit!
define_palettes!(
    (
        0,
        "Windows 95",
        "../assets/palettes/windows-95-256-colours.hex"
    ),
    (1, "Lospec 2000", "../assets/palettes/lospec-2000.hex"),
    (
        2,
        "Axulart 32",
        "../assets/palettes/axulart-32-color-palette.hex"
    ),
    (
        3,
        "Sirens at night",
        "../assets/palettes/sirens-at-night.hex"
    ),
    (3, "Oil 6", "../assets/palettes/oil-6.hex"),
    (4, "Game Boy", "../assets/palettes/gameboy.hex"),
);

/// System to automatically load configured palettes (silently by default)
pub fn auto_load_palettes(mut palette_manager: ResMut<PaletteManager>) {
    if palette_manager.len() > 0 {
        return;
    }

    let mut loaded_count = 0;
    use std::collections::HashMap;
    let mut palette_groups: HashMap<u32, Vec<(String, &str)>> = HashMap::new();

    // Group palettes by ID
    for &(id, name, file_path) in AVAILABLE_PALETTES {
        palette_groups
            .entry(id)
            .or_default()
            .push((name.to_string(), file_path));
    }

    // Sort palette groups by ID for deterministic loading order
    let mut sorted_groups: Vec<_> = palette_groups.into_iter().collect();
    sorted_groups.sort_by_key(|(id, _)| *id);

    // Load and combine palettes for each ID in sorted order
    for (id, palettes) in sorted_groups {
        let mut combined_palette = crate::palette::Palette::new();
        let mut combined_name = String::new();

        for (i, (name, file_path)) in palettes.iter().enumerate() {
            // Load the embedded palette data using macro-generated function
            let palette_data = get_embedded_palette_data(file_path);

            let palette_data = match palette_data {
                Some(data) => data,
                None => {
                    warn!("Unknown palette file path: {}", file_path);
                    continue;
                }
            };

            match crate::palette::Palette::parse_hex_content(palette_data, Some(name.clone())) {
                Ok(palette) => {
                    // Add colors from this palette to the combined palette
                    for color in palette.colors {
                        combined_palette.add_color(color);
                    }

                    // Build combined name
                    if i == 0 {
                        combined_name = name.clone();
                    } else {
                        combined_name.push_str(" + ");
                        combined_name.push_str(name);
                    }
                }
                Err(e) => {
                    info!("Failed to load palette '{}': {}", name, e);
                }
            }
        }

        if !combined_palette.is_empty() {
            let final_name = if palettes.len() > 1 {
                format!("ID{}: {} (Combined)", id, combined_name)
            } else {
                format!("ID{}: {}", id, combined_name)
            };
            combined_palette.name = Some(final_name.clone());
            let _palette_index = palette_manager.add_palette(combined_palette);

            // Show loading messages
            info!(
                "✓ Loaded palette: {} ({} colors) [Index: {}]",
                final_name,
                palette_manager
                    .get_palette(_palette_index)
                    .map(|p| p.len())
                    .unwrap_or(0),
                _palette_index
            );
            if palettes.len() > 1 {
                info!(
                    "  Combined {} individual palettes with ID {}",
                    palettes.len(),
                    id
                );
            }
            loaded_count += 1;
        }
    }

    // Ensure deterministic palette selection - always start with first palette
    if loaded_count > 0 {
        palette_manager.reset_to_first_palette();

        // Show summary of loaded palettes
        if let Some(current_palette) = palette_manager.current_palette() {
            if let Some(name) = &current_palette.name {
                info!(
                    "🎨 Active palette: {} ({} colors)",
                    name,
                    current_palette.len()
                );
            } else {
                info!("🎨 Active palette ({} colors)", current_palette.len());
            }
        }
        info!("📦 Loaded {} palette group(s) total", loaded_count);
        info!("🎮 Use N/M keys to switch between palettes during gameplay");
    } else if loaded_count == 0 {
        info!("❌ No palettes were loaded. Check AVAILABLE_PALETTES configuration.");
    }
}
