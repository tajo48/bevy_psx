//! # Bevy PSX
//!
//! A Bevy plugin that provides PSX-style low resolution rendering capabilities.
//!
//! This crate adds support for authentic retro-style rendering by rendering your 3D scene
//! to a low-resolution texture and then upscaling it to fill the window, with optional
//! nearest-neighbor filtering for a pixelated look.

mod components;
mod materials;
mod palette;
mod plugin;
mod resources;
mod systems;

pub use components::PsxCamera;
pub use materials::{
    BlendMode, ColorSpace, DitherPattern, PsxMaterial, PsxMaterialExtension, PsxSettings,
};
pub use palette::{Palette, PaletteError, PaletteManager};
pub use plugin::PsxCameraPlugin;
pub use resources::PsxRenderSettings;

/// Convenient re-exports for users of the bevy_psx library.
///
/// # Example
/// ```ignore
/// use bevy_psx::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        BlendMode, ColorSpace, DitherPattern, Palette, PaletteError, PaletteManager, PsxCamera,
        PsxCameraPlugin, PsxMaterial, PsxMaterialExtension, PsxRenderSettings, PsxSettings,
    };
}
