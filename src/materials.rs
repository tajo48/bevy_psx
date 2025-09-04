use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
};

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
    /// When enabled, colors are mapped to the nearest color in the predefined PSX palette
    #[uniform(100)]
    pub use_palette: u32,
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
        Self {
            // Default PSX-like quantization settings
            quantize_steps: 32,
            use_palette: 1, // Enable palette quantization by default
        }
    }
}

impl MaterialExtension for PsxVertexSnapExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/psx_vertex_snap.wgsl".into()
    }
}

impl MaterialExtension for PsxPaletteExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/psx_palette_quantize.wgsl".into()
    }
}

/// Type alias for PSX materials with vertex snapping
pub type PsxMaterial = ExtendedMaterial<StandardMaterial, PsxVertexSnapExtension>;

/// Type alias for PSX materials with palette quantization
pub type PsxPaletteMaterial = ExtendedMaterial<StandardMaterial, PsxPaletteExtension>;

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
    /// Whether palette quantization is globally enabled
    pub enabled: bool,
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
            use_palette: true,
            enabled: true,
        }
    }
}

/// System to automatically convert StandardMaterial to PsxPaletteMaterial
pub fn convert_standard_materials_to_psx(
    mut commands: Commands,
    meshes_with_standard_materials: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        (
            Without<MeshMaterial3d<PsxMaterial>>,
            Without<MeshMaterial3d<PsxPaletteMaterial>>,
        ),
    >,
    standard_material_assets: Res<Assets<StandardMaterial>>,
    mut psx_palette_material_assets: ResMut<Assets<PsxPaletteMaterial>>,
    psx_palette_settings: Res<PsxPaletteSettings>,
) {
    if !psx_palette_settings.enabled {
        return;
    }

    for (entity, material_handle) in meshes_with_standard_materials.iter() {
        if let Some(standard_material) = standard_material_assets.get(&material_handle.0) {
            // Create PSX palette material with the same base properties
            let psx_palette_material = PsxPaletteMaterial {
                base: standard_material.clone(),
                extension: PsxPaletteExtension {
                    quantize_steps: psx_palette_settings.quantize_steps,
                    use_palette: if psx_palette_settings.use_palette {
                        1
                    } else {
                        0
                    },
                },
            };

            let psx_palette_material_handle = psx_palette_material_assets.add(psx_palette_material);

            // Replace the material on the entity
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>();
            commands
                .entity(entity)
                .insert(MeshMaterial3d(psx_palette_material_handle));
        }
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

/// System to update PSX palette material settings when settings change
pub fn update_psx_palette_material_settings(
    psx_palette_settings: Res<PsxPaletteSettings>,
    mut psx_palette_materials: ResMut<Assets<PsxPaletteMaterial>>,
) {
    if !psx_palette_settings.is_changed() {
        return;
    }

    for (_handle, material) in psx_palette_materials.iter_mut() {
        material.extension.quantize_steps = psx_palette_settings.quantize_steps;
        material.extension.use_palette = if psx_palette_settings.use_palette {
            1
        } else {
            0
        };
    }
}
