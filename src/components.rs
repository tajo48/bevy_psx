use bevy::prelude::*;

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

/// Marker component for PSX cameras that have been configured
#[derive(Component)]
pub(crate) struct PsxCameraConfigured;

/// Marker component for the upscale quad
#[derive(Component)]
pub(crate) struct UpscaleQuad;

/// Marker component for the upscale camera
#[derive(Component)]
pub(crate) struct UpscaleCamera;
