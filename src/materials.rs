use bevy::{
    asset::uuid_handle,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
    shader::ShaderRef,
};

use crate::palette::PaletteManager;
pub const PSX_VERTEX_SNAP_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("23456789-1234-5678-90ab-cdef01234567");

pub const PSX_PALETTE_QUANTIZE_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("34567890-1234-5678-90ab-cdef01234567");

pub const PSX_LIGHT_BANDING_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("45678901-1234-5678-90ab-cdef01234567");

/// PSX vertex snapping material extension
///
/// This extension adds vertex snapping to any standard material, creating the
/// characteristic PSX vertex jittering effect.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PsxVertexSnapExtension {
    /// Controls how much vertex snapping occurs.
    /// Higher values = less snapping (smoother), lower values = more snapping (jittery)
    /// Typical PSX values: 64.0 - 256.0
    #[uniform(100)]
    pub snap_amount: f32,
}

/// PSX palette quantization material extension
///
/// This extension adds color palette quantization to any standard material, creating the
/// characteristic PSX limited color palette effect.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PsxPaletteExtension {
    /// Number of quantization steps for basic color reduction (0 to disable basic quantization)
    /// Higher values = smoother gradients, lower values = more posterized
    /// Typical PSX values: 16 - 64
    #[uniform(100)]
    pub quantize_steps: u32,

    /// Whether to use the PSX palette for color quantization (1 to enable, 0 to disable)
    /// When enabled, colors are mapped to the nearest color in the current palette
    #[uniform(100)]
    pub use_palette: u32,

    /// Number of colors in the current palette (for shader optimization)
    #[uniform(100)]
    pub palette_size: u32,

    /// The actual palette colors (up to 256 colors supported)
    #[uniform(100)]
    pub palette_colors: [Vec3; 256],

    /// Number of light bands (higher = more bands = smoother lighting)
    /// Typical PSX values: 4-16 bands
    #[uniform(100)]
    pub bands: u32,

    /// Whether light banding is enabled (1 to enable, 0 to disable)
    #[uniform(100)]
    pub light_banding_enabled: u32,

    /// Strength of dithering between bands (0.0 = no dither, 1.0 = full dither)
    #[uniform(100)]
    pub dither_strength: f32,

    /// Smoothness of band transitions (0.0 = sharp bands, 1.0 = smooth)
    #[uniform(100)]
    pub band_smoothness: f32,
}

/// PSX light banding material extension
///
/// This extension adds light banding to any standard material, creating the
/// characteristic PSX stepped lighting effect.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PsxLightBandingExtension {
    /// Number of light bands (higher = more bands = smoother lighting)
    /// Typical PSX values: 4-16 bands
    #[uniform(100)]
    pub bands: u32,

    /// Whether light banding is enabled (1 to enable, 0 to disable)
    #[uniform(100)]
    pub enabled: u32,

    /// Strength of dithering between bands (0.0 = no dither, 1.0 = full dither)
    #[uniform(100)]
    pub dither_strength: f32,

    /// Smoothness of band transitions (0.0 = sharp bands, 1.0 = smooth)
    #[uniform(100)]
    pub band_smoothness: f32,
}

impl PsxPaletteExtension {
    /// Set palette enabled state using a boolean
    pub fn set_use_palette(&mut self, enabled: bool) {
        self.use_palette = if enabled { 1 } else { 0 };
    }

    /// Get palette enabled state as a boolean
    pub fn get_use_palette(&self) -> bool {
        self.use_palette != 0
    }

    /// Set light banding enabled state using a boolean
    pub fn set_light_banding_enabled(&mut self, enabled: bool) {
        self.light_banding_enabled = if enabled { 1 } else { 0 };
    }

    /// Get light banding enabled state as a boolean
    pub fn get_light_banding_enabled(&self) -> bool {
        self.light_banding_enabled != 0
    }
}

impl PsxLightBandingExtension {
    /// Set light banding enabled state using a boolean
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = if enabled { 1 } else { 0 };
    }

    /// Get light banding enabled state as a boolean
    pub fn get_enabled(&self) -> bool {
        self.enabled != 0
    }
}

impl Default for PsxVertexSnapExtension {
    fn default() -> Self {
        Self {
            // Default PSX-like vertex snapping amount
            snap_amount: 64.0,
        }
    }
}

impl Default for PsxPaletteExtension {
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
            quantize_steps: 64,
            use_palette: 0,
            palette_size: default_colors.len() as u32,
            palette_colors,
            bands: 8,
            light_banding_enabled: 0,
            dither_strength: 0.3,
            band_smoothness: 0.0,
        }
    }
}

impl MaterialExtension for PsxVertexSnapExtension {
    fn vertex_shader() -> ShaderRef {
        PSX_VERTEX_SNAP_SHADER_HANDLE.into()
    }
}

impl MaterialExtension for PsxPaletteExtension {
    fn fragment_shader() -> ShaderRef {
        PSX_PALETTE_QUANTIZE_SHADER_HANDLE.into()
    }
}

impl Default for PsxLightBandingExtension {
    fn default() -> Self {
        Self {
            bands: 8,
            enabled: 1,
            dither_strength: 0.3,
            band_smoothness: 0.0,
        }
    }
}

impl MaterialExtension for PsxLightBandingExtension {
    fn fragment_shader() -> ShaderRef {
        PSX_LIGHT_BANDING_SHADER_HANDLE.into()
    }
}

/// Type alias for PSX materials with vertex snapping
pub type PsxMaterial = ExtendedMaterial<StandardMaterial, PsxVertexSnapExtension>;

/// Type alias for PSX materials with palette quantization
pub type PsxPaletteMaterial = ExtendedMaterial<StandardMaterial, PsxPaletteExtension>;

/// Type alias for PSX materials with light banding
pub type PsxLightBandingMaterial = ExtendedMaterial<StandardMaterial, PsxLightBandingExtension>;

/// Resource to configure PSX vertex snapping globally
#[derive(Resource, Debug, Clone)]
pub struct PsxVertexSnapSettings {
    /// Global snap amount for all PSX materials
    pub snap_amount: f32,
    /// Whether vertex snapping is enabled
    pub enabled: bool,
}

/// Resource to configure PSX palette quantization globally
#[derive(Resource, Debug, Clone)]
pub struct PsxPaletteSettings {
    /// Global quantization steps for all PSX palette materials
    pub quantize_steps: u32,
    /// Whether palette quantization is enabled
    pub use_palette: bool,
}

/// Resource to configure PSX light banding globally
#[derive(Resource, Debug, Clone)]
pub struct PsxLightBandingSettings {
    /// Global number of light bands for all PSX light banding materials
    pub bands: u32,
    /// Whether light banding is enabled
    pub enabled: bool,
    /// Global dithering strength between bands
    pub dither_strength: f32,
    /// Global smoothness of band transitions
    pub band_smoothness: f32,
}

impl Default for PsxVertexSnapSettings {
    fn default() -> Self {
        Self {
            snap_amount: 64.0,
            enabled: true,
        }
    }
}

impl Default for PsxPaletteSettings {
    fn default() -> Self {
        Self {
            quantize_steps: 32,
            use_palette: false,
        }
    }
}

impl Default for PsxLightBandingSettings {
    fn default() -> Self {
        Self {
            bands: 8,
            enabled: false,
            dither_strength: 0.3,
            band_smoothness: 0.0,
        }
    }
}

/// System to automatically convert StandardMaterial to PsxLightBandingMaterial
/// Unified system to convert standard materials to appropriate PSX materials based on enabled settings
pub fn convert_standard_materials_to_psx_unified(
    mut commands: Commands,
    meshes_with_standard_materials: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        (
            Without<MeshMaterial3d<PsxMaterial>>,
            Without<MeshMaterial3d<PsxPaletteMaterial>>,
            Without<MeshMaterial3d<PsxLightBandingMaterial>>,
        ),
    >,
    standard_material_assets: Res<Assets<StandardMaterial>>,
    mut psx_palette_material_assets: ResMut<Assets<PsxPaletteMaterial>>,
    mut psx_light_banding_material_assets: ResMut<Assets<PsxLightBandingMaterial>>,
    psx_palette_settings: Res<PsxPaletteSettings>,
    psx_light_banding_settings: Res<PsxLightBandingSettings>,
    palette_manager: Option<Res<PaletteManager>>,
) {
    let use_palette = psx_palette_settings.use_palette;
    let use_light_banding = psx_light_banding_settings.enabled;

    // Exit early if neither effect is enabled
    if !use_palette && !use_light_banding {
        return;
    }

    for (entity, material_handle) in meshes_with_standard_materials.iter() {
        if let Some(standard_material) = standard_material_assets.get(&material_handle.0) {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>();

            match (use_palette, use_light_banding) {
                // Both palette and light banding enabled - use palette material (which has dithering)
                (true, true) => {
                    let mut extension = PsxPaletteExtension::default();
                    extension.quantize_steps = psx_palette_settings.quantize_steps;
                    extension.set_use_palette(true);
                    // Enable light banding in the palette extension
                    extension.bands = psx_light_banding_settings.bands;
                    extension.set_light_banding_enabled(true);
                    extension.dither_strength = psx_light_banding_settings.dither_strength;
                    extension.band_smoothness = psx_light_banding_settings.band_smoothness;

                    // Update with current palette if available
                    if let Some(palette_manager) = &palette_manager {
                        if let Some(current_palette) = palette_manager.current_palette() {
                            let palette_array = current_palette.to_shader_array(256);
                            extension.palette_size = palette_array.len() as u32;

                            for (i, &color) in palette_array.iter().enumerate() {
                                extension.palette_colors[i] = color;
                            }
                        }
                    }

                    let psx_palette_material = PsxPaletteMaterial {
                        base: standard_material.clone(),
                        extension,
                    };

                    let handle = psx_palette_material_assets.add(psx_palette_material);
                    commands.entity(entity).insert(MeshMaterial3d(handle));
                }
                // Only palette enabled
                (true, false) => {
                    let mut extension = PsxPaletteExtension::default();
                    extension.quantize_steps = psx_palette_settings.quantize_steps;
                    extension.set_use_palette(true);

                    // Update with current palette if available
                    if let Some(palette_manager) = &palette_manager {
                        if let Some(current_palette) = palette_manager.current_palette() {
                            let palette_array = current_palette.to_shader_array(256);
                            extension.palette_size = palette_array.len() as u32;

                            for (i, &color) in palette_array.iter().enumerate() {
                                extension.palette_colors[i] = color;
                            }
                        }
                    }

                    let psx_palette_material = PsxPaletteMaterial {
                        base: standard_material.clone(),
                        extension,
                    };

                    let handle = psx_palette_material_assets.add(psx_palette_material);
                    commands.entity(entity).insert(MeshMaterial3d(handle));
                }
                // Only light banding enabled
                (false, true) => {
                    let mut extension = PsxLightBandingExtension::default();
                    extension.bands = psx_light_banding_settings.bands;
                    extension.set_enabled(true);
                    extension.dither_strength = psx_light_banding_settings.dither_strength;
                    extension.band_smoothness = psx_light_banding_settings.band_smoothness;

                    let psx_light_banding_material = PsxLightBandingMaterial {
                        base: standard_material.clone(),
                        extension,
                    };

                    let handle = psx_light_banding_material_assets.add(psx_light_banding_material);
                    commands.entity(entity).insert(MeshMaterial3d(handle));
                }
                // Neither enabled (already handled by early return)
                (false, false) => {}
            }
        }
    }
}

/// System to update PSX light banding material settings when settings change
pub fn update_psx_light_banding_material_settings(
    psx_light_banding_settings: Res<PsxLightBandingSettings>,
    mut psx_light_banding_materials: ResMut<Assets<PsxLightBandingMaterial>>,
) {
    if !psx_light_banding_settings.is_changed() {
        return;
    }

    for (_handle, material) in psx_light_banding_materials.iter_mut() {
        material.extension.bands = psx_light_banding_settings.bands;
        material
            .extension
            .set_enabled(psx_light_banding_settings.enabled);
        material.extension.dither_strength = psx_light_banding_settings.dither_strength;
        material.extension.band_smoothness = psx_light_banding_settings.band_smoothness;
    }
}

/// System to update PSX material snap amounts when settings change
pub fn update_psx_material_snap_amounts(
    psx_settings: Res<PsxVertexSnapSettings>,
    mut psx_materials: ResMut<Assets<PsxMaterial>>,
) {
    if !psx_settings.is_changed() {
        return;
    }

    for (_handle, material) in psx_materials.iter_mut() {
        material.extension.snap_amount = psx_settings.snap_amount;
    }
}

/// System to show palette loading information (only when palettes are enabled)
pub fn show_palette_info(
    palette_manager: Res<PaletteManager>,
    palette_settings: Res<PsxPaletteSettings>,
    mut has_shown: Local<bool>,
) {
    // Only show info if palettes are enabled and we haven't shown it yet
    if *has_shown || palette_manager.len() == 0 || !palette_settings.use_palette {
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

/// System to update PSX palette material settings when settings change
pub fn update_psx_palette_material_settings(
    psx_palette_settings: Res<PsxPaletteSettings>,
    psx_light_banding_settings: Res<PsxLightBandingSettings>,
    palette_manager: Option<Res<PaletteManager>>,
    mut psx_palette_materials: ResMut<Assets<PsxPaletteMaterial>>,
) {
    let palette_settings_changed = psx_palette_settings.is_changed();
    let light_banding_settings_changed = psx_light_banding_settings.is_changed();
    let palette_changed = palette_manager.as_ref().map_or(false, |pm| pm.is_changed());

    if !palette_settings_changed && !light_banding_settings_changed && !palette_changed {
        return;
    }

    for (_handle, material) in psx_palette_materials.iter_mut() {
        if palette_settings_changed {
            material.extension.quantize_steps = psx_palette_settings.quantize_steps;
            material
                .extension
                .set_use_palette(psx_palette_settings.use_palette);
        }

        if light_banding_settings_changed {
            material.extension.bands = psx_light_banding_settings.bands;
            material
                .extension
                .set_light_banding_enabled(psx_light_banding_settings.enabled);
            material.extension.dither_strength = psx_light_banding_settings.dither_strength;
            material.extension.band_smoothness = psx_light_banding_settings.band_smoothness;
        }

        // Update palette data if palette manager changed
        if palette_changed {
            if let Some(palette_manager) = &palette_manager {
                if let Some(current_palette) = palette_manager.current_palette() {
                    let palette_array = current_palette.to_shader_array(256);
                    material.extension.palette_size = palette_array.len() as u32;

                    for (i, &color) in palette_array.iter().enumerate() {
                        material.extension.palette_colors[i] = color;
                    }
                }
            }
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
    (0, "Game Boy", "../assets/palettes/gameboy.hex"),
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
);

/// System to automatically load configured palettes (silently by default)
pub fn auto_load_palettes(
    mut palette_manager: ResMut<PaletteManager>,
    palette_settings: Res<PsxPaletteSettings>,
) {
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
                    if palette_settings.use_palette {
                        warn!("Unknown palette file path: {}", file_path);
                    }
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
                    if palette_settings.use_palette {
                        warn!("Failed to load palette '{}': {}", name, e);
                    }
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

            // Only show loading messages if palettes are enabled
            if palette_settings.use_palette {
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
            }
            loaded_count += 1;
        }
    }

    // Ensure deterministic palette selection - always start with first palette
    if loaded_count > 0 {
        palette_manager.reset_to_first_palette();

        // Only show summary if palettes are enabled
        if palette_settings.use_palette {
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
        }
    } else if loaded_count == 0 && palette_settings.use_palette {
        warn!("❌ No palettes were loaded. Check AVAILABLE_PALETTES configuration.");
    }
}
