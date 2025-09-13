use bevy::prelude::*;

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
    /// Creates default PSX render settings with authentic PSX resolution (427,240)
    /// and pixelated filtering enabled.
    fn default() -> Self {
        Self {
            render_resolution: UVec2::new(427, 240),
            pixelated: true,
        }
    }
}

/// Internal resource to hold the render target image handle
#[derive(Resource)]
pub(crate) struct RenderTargetHandle(pub(crate) Handle<Image>);
