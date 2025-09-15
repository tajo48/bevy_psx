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
    /// When aspect ratio matching is enabled, this will be automatically adjusted based on
    /// the window aspect ratio and the base resolution.
    /// Common retro console resolutions:
    /// - PSX: 320x240
    /// - PS2: 512x448
    /// - N64: 320x240
    /// - SNES: 256x224
    pub render_resolution: UVec2,

    /// The base resolution used for aspect ratio calculations.
    ///
    /// This represents the "reference" resolution (typically 4:3 PSX resolution like 320x240).
    /// When aspect ratio matching is enabled, one dimension of this resolution is kept constant
    /// while the other is adjusted to match the window's aspect ratio.
    pub base_resolution: UVec2,

    /// Whether to automatically adjust render resolution to match window aspect ratio.
    ///
    /// When enabled:
    /// - If window is wider than base aspect ratio: keeps height constant, adjusts width
    /// - If window is taller than base aspect ratio: keeps width constant, adjusts height
    /// - Maintains low resolution aesthetic while preventing stretching
    pub aspect_ratio_matching: bool,

    /// Whether to use nearest neighbor filtering for pixelated look.
    ///
    /// - `true`: Uses nearest neighbor filtering for sharp, pixelated edges (authentic retro look)
    /// - `false`: Uses linear filtering for smoother upscaling
    pub pixelated: bool,
}

impl Default for PsxRenderSettings {
    /// Creates default PSX render settings with aspect ratio matching enabled.
    /// Base resolution is classic PSX 4:3 (320x240), but render resolution starts
    /// at 16:9 equivalent (427x240) and will adjust based on window aspect ratio.
    fn default() -> Self {
        Self {
            render_resolution: UVec2::new(427, 240), // Will be adjusted by aspect ratio matching
            base_resolution: UVec2::new(320, 240),   // Classic PSX 4:3 resolution
            aspect_ratio_matching: true,             // Enabled by default
            pixelated: true,
        }
    }
}

/// Internal resource to hold the render target image handle
#[derive(Resource)]
pub(crate) struct RenderTargetHandle(pub(crate) Handle<Image>);
