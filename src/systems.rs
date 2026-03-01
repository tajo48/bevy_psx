use bevy::{
    camera::RenderTarget,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::WindowResized,
};

use crate::{
    components::{PsxCamera, PsxCameraConfigured, UpscaleCamera, UpscaleQuad},
    resources::{PsxRenderSettings, RenderTargetHandle},
};

pub(crate) fn setup_psx_render_targets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    psx_settings: Res<PsxRenderSettings>,
) {
    // Create a render target texture at PSX resolution
    let size = Extent3d {
        width: psx_settings.render_resolution.x,
        height: psx_settings.render_resolution.y,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("psx_render_target"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(),
        ..default()
    };

    // Fill with data to ensure proper initialization
    image.data = Some(vec![0u8; (size.width * size.height * 4) as usize]);

    // Note: Texture filtering is controlled by the material/sprite settings in Bevy
    // The actual filtering happens during the upscaling phase

    let image_handle = images.add(image);

    // Store the render target handle
    commands.insert_resource(RenderTargetHandle(image_handle.clone()));

    // Create a camera that renders to the screen
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        UpscaleCamera,
    ));

    // Create a sprite that displays the render target
    // We'll use a basic sprite and scale it appropriately
    let sprite = if psx_settings.pixelated {
        // For pixelated mode, use the default sprite settings which should use nearest filtering
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(
                psx_settings.render_resolution.x as f32,
                psx_settings.render_resolution.y as f32,
            )),
            ..default()
        }
    } else {
        Sprite {
            image: image_handle,
            custom_size: Some(Vec2::new(
                psx_settings.render_resolution.x as f32,
                psx_settings.render_resolution.y as f32,
            )),
            ..default()
        }
    };

    commands.spawn((sprite, Transform::from_xyz(0.0, 0.0, 0.0), UpscaleQuad));
}

pub(crate) fn handle_psx_camera_spawn(
    mut commands: Commands,
    query: Query<(Entity, &Camera), (With<PsxCamera>, Without<PsxCameraConfigured>)>,
    render_target: Option<Res<RenderTargetHandle>>,
) {
    if let Some(render_target) = render_target {
        for (entity, camera) in query.iter() {
            let mut new_camera = camera.clone();
            new_camera.order = 0;

            commands.entity(entity).insert((
                new_camera,
                RenderTarget::Image(render_target.0.clone().into()),
                PsxCameraConfigured,
                Msaa::Off,
            ));
        }
    }
}

pub(crate) fn update_render_target_size(
    psx_settings: Res<PsxRenderSettings>,
    mut images: ResMut<Assets<Image>>,
    render_target: Option<Res<RenderTargetHandle>>,
    mut sprites: Query<&mut Sprite, With<UpscaleQuad>>,
) {
    if !psx_settings.is_changed() {
        return;
    }

    if let Some(render_target) = render_target {
        if let Some(image) = images.get_mut(&render_target.0) {
            let new_size = Extent3d {
                width: psx_settings.render_resolution.x,
                height: psx_settings.render_resolution.y,
                depth_or_array_layers: 1,
            };

            image.resize(new_size);
        }

        // Update sprite size
        for mut sprite in sprites.iter_mut() {
            sprite.custom_size = Some(Vec2::new(
                psx_settings.render_resolution.x as f32,
                psx_settings.render_resolution.y as f32,
            ));
        }
    }
}

pub(crate) fn update_aspect_ratio_matching(
    windows: Query<&Window>,
    mut resize_events: MessageReader<WindowResized>,
    mut psx_settings: ResMut<PsxRenderSettings>,
) {
    if !psx_settings.aspect_ratio_matching {
        return;
    }

    let mut should_update = false;
    let mut window_size = Vec2::ZERO;

    // Check for window resize events
    for event in resize_events.read() {
        window_size = Vec2::new(event.width, event.height);
        should_update = true;
    }

    // Also check on first frame or when aspect ratio matching is first enabled
    if !should_update && psx_settings.is_changed() {
        if let Ok(window) = windows.single() {
            window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 {
                should_update = true;
            }
        }
    }

    if should_update && window_size.x > 0.0 && window_size.y > 0.0 {
        // Calculate aspect ratios
        let window_aspect = window_size.x / window_size.y;
        let base_aspect =
            psx_settings.base_resolution.x as f32 / psx_settings.base_resolution.y as f32;

        let new_resolution = if window_aspect > base_aspect {
            // Window is wider than base aspect ratio
            // Keep height constant, adjust width
            UVec2::new(
                (psx_settings.base_resolution.y as f32 * window_aspect).round() as u32,
                psx_settings.base_resolution.y,
            )
        } else {
            // Window is taller than base aspect ratio
            // Keep width constant, adjust height
            UVec2::new(
                psx_settings.base_resolution.x,
                (psx_settings.base_resolution.x as f32 / window_aspect).round() as u32,
            )
        };

        // Only update if the resolution actually changed
        if new_resolution != psx_settings.render_resolution {
            psx_settings.render_resolution = new_resolution;
        }
    }
}

pub(crate) fn update_upscale_quad_size(
    windows: Query<&Window>,
    mut resize_events: MessageReader<WindowResized>,
    mut quad_query: Query<&mut Transform, With<UpscaleQuad>>,
    psx_settings: Res<PsxRenderSettings>,
) {
    let mut update_size = false;
    let mut window_size = Vec2::ZERO;

    // Check for window resize events
    for event in resize_events.read() {
        window_size = Vec2::new(event.width, event.height);
        update_size = true;
    }

    // If no resize event, check window on first frame
    if !update_size {
        if let Ok(window) = windows.single() {
            window_size = Vec2::new(window.width(), window.height());
            if window_size.x > 0.0 && window_size.y > 0.0 {
                update_size = true;
            }
        }
    }

    if update_size && window_size.x > 0.0 && window_size.y > 0.0 {
        // Update the quad to fill the screen
        for mut transform in quad_query.iter_mut() {
            // Calculate the scale needed to fill the window while maintaining aspect ratio
            let render_aspect =
                psx_settings.render_resolution.x as f32 / psx_settings.render_resolution.y as f32;
            let window_aspect = window_size.x / window_size.y;

            let scale = if window_aspect > render_aspect {
                // Window is wider, fit to height
                window_size.y / psx_settings.render_resolution.y as f32
            } else {
                // Window is taller, fit to width
                window_size.x / psx_settings.render_resolution.x as f32
            };

            transform.scale = Vec3::new(scale, scale, 1.0);
        }
    }
}
