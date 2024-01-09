pub mod camera;
pub mod material;

use bevy::render::primitives::Aabb;
use bevy::sprite::Material2dPlugin;
use bevy::{
    asset::{load_internal_asset, load_internal_binary_asset},
    prelude::*,
    render::{
        camera::ScalingMode,
        texture::{CompressedImageFormats, ImageSampler, ImageType},
        view::VisibleEntities,
    },
};

use crate::material::{
    PsxDitherMaterial, PsxMaterial, PSX_DITHER_HANDLE, PSX_DITH_SHADER_HANDLE,
    PSX_FRAG_SHADER_HANDLE, PSX_VERT_SHADER_HANDLE,
};

pub fn image_load(bytes: &[u8],_unused: String) -> Image {
    let image = Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::nearest(),
    )
    .unwrap();
    image
}

pub struct PsxPlugin;

impl Plugin for PsxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PsxMaterial>::default());
        app.add_plugins(Material2dPlugin::<PsxDitherMaterial>::default());
        app.register_type::<Camera>()
            .register_type::<Visibility>()
            .register_type::<InheritedVisibility>()
            .register_type::<ViewVisibility>()
            .register_type::<OrthographicProjection>()
            .register_type::<VisibleEntities>()
            .register_type::<ScalingMode>()
            .register_type::<Aabb>()
            .add_systems(PostUpdate, camera::setup_camera)
            .add_systems(Update,camera::scale_render_image);


        load_internal_binary_asset!(app, PSX_DITHER_HANDLE, "psx-dith.png", image_load);

        load_internal_asset!(
            app,
            PSX_FRAG_SHADER_HANDLE,
            "psx-frag.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PSX_DITH_SHADER_HANDLE,
            "psx-dither.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            PSX_VERT_SHADER_HANDLE,
            "psx-vert.wgsl",
// https://bevyengine.org/learn/migration-guides/0.11-0.12/#pbr-shader-cleanup
            Shader::from_wgsl
        );
    }
}
