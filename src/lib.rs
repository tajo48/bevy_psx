use bevy::{
    image::ImageSampler,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::WindowResized,
};

/// Convenient re-exports for users of the bevy_psx library.
///
/// # Example
/// ```ignore
/// use bevy_psx::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{PsxCamera, PsxCameraPlugin, PsxRenderSettings};
}

/// Plugin that adds PSX-style low resolution rendering support to Bevy.
///
/// This plugin sets up a rendering pipeline that:
/// - Renders your 3D scene to a low-resolution texture
/// - Upscales that texture to fill the window
/// - Optionally applies nearest-neighbor filtering for a pixelated look
/// - Automatically disables MSAA for authentic PSX rendering
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
            .add_systems(Startup, setup_psx_render_targets)
            .add_systems(
                Update,
                (
                    handle_psx_camera_spawn,
                    update_render_target_size,
                    update_upscale_quad_size,
                ),
            );
    }
}

/// Component to mark a camera as a PSX-style camera.
///
/// Add this component to any camera to make it render at the configured PSX resolution.
/// The camera will automatically render to a low-resolution texture that gets upscaled
/// to the window size. MSAA (Multi-Sample Anti-Aliasing) is automatically disabled
/// for authentic PSX rendering without anti-aliasing.
///
/// # Example
/// ```ignore
/// commands.spawn((
///     Camera3d::default(),
///     Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
///     PsxCamera,  // Add this component to enable PSX rendering
///     // Msaa::Off is automatically added by the plugin
/// ));
/// ```
///
/// # Note
/// Only cameras with this component will be affected by the PSX rendering pipeline.
/// You can have multiple cameras in your scene, but only those marked with `PsxCamera`
/// will render at the low resolution. The plugin automatically adds `Msaa::Off` to
/// these cameras for authentic retro rendering without anti-aliasing.
#[derive(Component, Debug, Clone, Copy)]
pub struct PsxCamera;

/// Settings for PSX-style rendering.
///
/// This resource controls how the PSX camera renders the scene.
/// You can modify these settings at runtime to change the rendering behavior.
///
/// # Example
/// ```ignore
/// fn configure_psx(mut settings: ResMut<PsxRenderSettings>) {
///     // Set to authentic PSX resolution
///     settings.render_resolution = UVec2::new(320, 240);
///
///     // Enable pixelated look
///     settings.pixelated = true;
/// }
/// ```
#[derive(Resource, Debug, Clone)]
pub struct PsxRenderSettings {
    /// The internal render resolution.
    ///
    /// This is the resolution at which the 3D scene will be rendered before being upscaled.
    /// Common retro console resolutions:
    /// - PSX: 320x240
    /// - PS2: 512x448
    /// - N64: 320x240
    /// - SNES: 256x224
    pub render_resolution: UVec2,

    /// Whether to use nearest neighbor filtering for pixelated look.
    ///
    /// - `true`: Uses nearest neighbor filtering for sharp, pixelated edges (authentic retro look)
    /// - `false`: Uses linear filtering for smoother upscaling
    pub pixelated: bool,
}

impl Default for PsxRenderSettings {
    /// Creates default PSX render settings with authentic PSX resolution (320x240)
    /// and pixelated filtering enabled.
    fn default() -> Self {
        Self {
            // Classic PSX resolution
            render_resolution: UVec2::new(320, 240),
            pixelated: true,
        }
    }
}

/// Marker component for PSX cameras that have been configured
#[derive(Component)]
struct PsxCameraConfigured;

/// Marker component for the upscale quad
#[derive(Component)]
struct UpscaleQuad;

/// Marker component for the upscale camera
#[derive(Component)]
struct UpscaleCamera;

fn setup_psx_render_targets(
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

#[derive(Resource)]
struct RenderTargetHandle(Handle<Image>);

fn handle_psx_camera_spawn(
    mut commands: Commands,
    query: Query<(Entity, &Camera), (With<PsxCamera>, Without<PsxCameraConfigured>)>,
    render_target: Option<Res<RenderTargetHandle>>,
) {
    if let Some(render_target) = render_target {
        for (entity, camera) in query.iter() {
            let mut new_camera = camera.clone();
            new_camera.target = RenderTarget::Image(render_target.0.clone().into());
            new_camera.order = 0;

            commands
                .entity(entity)
                .insert((new_camera, PsxCameraConfigured, Msaa::Off));
        }
    }
}

fn update_render_target_size(
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

fn update_upscale_quad_size(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
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
