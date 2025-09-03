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

impl Default for PsxVertexSnapExtension {
    fn default() -> Self {
        Self {
            // Default PSX-like vertex snapping amount
            snap_amount: 64.0,
        }
    }
}

impl MaterialExtension for PsxVertexSnapExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/psx_vertex_snap.wgsl".into()
    }
}

/// Type alias for PSX materials
pub type PsxMaterial = ExtendedMaterial<StandardMaterial, PsxVertexSnapExtension>;

/// Resource to configure PSX vertex snapping globally
#[derive(Resource, Debug, Clone)]
pub struct PsxVertexSnapSettings {
    /// Global snap amount for all PSX materials
    pub snap_amount: f32,
    /// Whether vertex snapping is enabled
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

/// System to automatically convert StandardMaterial to PsxMaterial
pub fn convert_standard_materials_to_psx(
    mut commands: Commands,
    meshes_with_standard_materials: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Without<MeshMaterial3d<PsxMaterial>>,
    >,
    standard_material_assets: Res<Assets<StandardMaterial>>,
    mut psx_material_assets: ResMut<Assets<PsxMaterial>>,
    psx_settings: Res<PsxVertexSnapSettings>,
) {
    if !psx_settings.enabled {
        return;
    }

    for (entity, material_handle) in meshes_with_standard_materials.iter() {
        if let Some(standard_material) = standard_material_assets.get(&material_handle.0) {
            // Create PSX material with the same base properties
            let psx_material = PsxMaterial {
                base: standard_material.clone(),
                extension: PsxVertexSnapExtension {
                    snap_amount: psx_settings.snap_amount,
                },
            };

            let psx_material_handle = psx_material_assets.add(psx_material);

            // Replace the material on the entity
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>();
            commands
                .entity(entity)
                .insert(MeshMaterial3d(psx_material_handle));
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
