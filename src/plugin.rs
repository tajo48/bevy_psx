use bevy::{pbr::MaterialPlugin, prelude::*};

use crate::{
    materials::{
        auto_load_palettes, convert_standard_materials_to_psx, show_palette_info,
        update_psx_material_snap_amounts, update_psx_palette_material_settings, PsxMaterial,
        PsxPaletteMaterial, PsxPaletteSettings, PsxVertexSnapSettings,
    },
    palette::PaletteManager,
    resources::PsxRenderSettings,
    systems::{
        handle_psx_camera_spawn, setup_psx_render_targets, update_render_target_size,
        update_upscale_quad_size,
    },
};

/// Plugin that adds PSX-style low resolution rendering and vertex snapping support to Bevy.
///
/// This plugin sets up a rendering pipeline that:
/// - Renders your 3D scene to a low-resolution texture
/// - Upscales that texture to fill the window
/// - Optionally applies nearest-neighbor filtering for a pixelated look
/// - Automatically disables MSAA for authentic PSX rendering
/// - Automatically applies PSX vertex snapping to all 3D models
///
/// # Example
/// ```ignore
/// use bevy::prelude::*;
/// use bevy_psx::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(PsxCameraPlugin)
///     .run();
/// ```
pub struct PsxCameraPlugin;

impl Plugin for PsxCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PsxRenderSettings>()
            .init_resource::<PsxVertexSnapSettings>()
            .init_resource::<PsxPaletteSettings>()
            .init_resource::<PaletteManager>()
            .add_plugins(MaterialPlugin::<PsxMaterial>::default())
            .add_plugins(MaterialPlugin::<PsxPaletteMaterial>::default())
            .add_systems(Startup, (setup_psx_render_targets, auto_load_palettes))
            .add_systems(
                Update,
                (
                    handle_psx_camera_spawn,
                    update_render_target_size,
                    update_upscale_quad_size,
                    convert_standard_materials_to_psx,
                    update_psx_material_snap_amounts,
                    update_psx_palette_material_settings,
                    show_palette_info,
                ),
            );
    }
}
