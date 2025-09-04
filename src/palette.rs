use bevy::prelude::*;
use std::fs;
use std::path::Path;

/// Represents a color palette that can be loaded from various formats
#[derive(Debug, Clone, Resource)]
pub struct Palette {
    /// The colors in the palette as Vec3 (RGB values from 0.0 to 1.0)
    pub colors: Vec<Vec3>,
    /// Optional name of the palette
    pub name: Option<String>,
}

impl Palette {
    /// Create a new empty palette
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            name: None,
        }
    }

    /// Create a palette with a name
    pub fn with_name(name: String) -> Self {
        Self {
            colors: Vec::new(),
            name: Some(name),
        }
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, color: Vec3) {
        self.colors.push(color);
    }

    /// Get the number of colors in the palette
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Check if the palette is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Load a palette from a .hex file
    pub fn load_from_hex_file<P: AsRef<Path>>(path: P) -> Result<Self, PaletteError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| PaletteError::IoError(format!("Failed to read file: {}", e)))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        Self::parse_hex_content(&content, name)
    }

    /// Parse hex content from a string
    pub fn parse_hex_content(content: &str, name: Option<String>) -> Result<Self, PaletteError> {
        let mut palette = Self {
            colors: Vec::new(),
            name,
        };

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            // Remove any leading # if present
            let hex_str = if line.starts_with('#') {
                &line[1..]
            } else {
                line
            };

            // Parse hex color
            match parse_hex_color(hex_str) {
                Ok(color) => palette.add_color(color),
                Err(e) => {
                    return Err(PaletteError::ParseError(format!(
                        "Line {}: {}",
                        line_num + 1,
                        e
                    )));
                }
            }
        }

        if palette.is_empty() {
            return Err(PaletteError::EmptyPalette);
        }

        Ok(palette)
    }

    /// Find the closest color in the palette to the given color
    pub fn find_closest_color(&self, target: Vec3) -> Vec3 {
        if self.colors.is_empty() {
            return Vec3::ZERO;
        }

        let mut closest = self.colors[0];
        let mut min_distance = distance_squared(target, closest);

        for &color in &self.colors[1..] {
            let dist = distance_squared(target, color);
            if dist < min_distance {
                min_distance = dist;
                closest = color;
            }
        }

        closest
    }

    /// Convert the palette to an array suitable for shader use
    pub fn to_shader_array(&self, max_size: usize) -> Vec<Vec3> {
        let mut result = Vec::with_capacity(max_size);

        // Add actual colors
        for &color in &self.colors {
            if result.len() >= max_size {
                break;
            }
            result.push(color);
        }

        // Pad with the last color or black if empty
        let fill_color = self.colors.last().copied().unwrap_or(Vec3::ZERO);
        while result.len() < max_size {
            result.push(fill_color);
        }

        result
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur when loading palettes
#[derive(Debug, Clone)]
pub enum PaletteError {
    IoError(String),
    ParseError(String),
    EmptyPalette,
    InvalidHexColor(String),
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaletteError::IoError(msg) => write!(f, "IO Error: {}", msg),
            PaletteError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            PaletteError::EmptyPalette => write!(f, "Empty palette"),
            PaletteError::InvalidHexColor(color) => write!(f, "Invalid hex color: {}", color),
        }
    }
}

impl std::error::Error for PaletteError {}

/// Parse a hex color string to Vec3 RGB values (0.0 to 1.0)
fn parse_hex_color(hex: &str) -> Result<Vec3, PaletteError> {
    let hex = hex.trim();

    // Handle different hex formats
    let hex = if hex.len() == 3 {
        // Convert RGB to RRGGBB
        format!(
            "{0}{0}{1}{1}{2}{2}",
            hex.chars().nth(0).unwrap(),
            hex.chars().nth(1).unwrap(),
            hex.chars().nth(2).unwrap()
        )
    } else if hex.len() == 6 {
        hex.to_string()
    } else {
        return Err(PaletteError::InvalidHexColor(format!(
            "Invalid hex color length: {} (expected 3 or 6 characters)",
            hex
        )));
    };

    // Parse the hex values
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| {
        PaletteError::InvalidHexColor(format!("Invalid red component: {}", &hex[0..2]))
    })?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| {
        PaletteError::InvalidHexColor(format!("Invalid green component: {}", &hex[2..4]))
    })?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| {
        PaletteError::InvalidHexColor(format!("Invalid blue component: {}", &hex[4..6]))
    })?;

    // Convert to 0.0-1.0 range
    Ok(Vec3::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

/// Calculate squared distance between two colors (faster than sqrt for comparisons)
fn distance_squared(a: Vec3, b: Vec3) -> f32 {
    let diff = a - b;
    diff.dot(diff)
}

/// Resource to manage available palettes
#[derive(Resource, Default)]
pub struct PaletteManager {
    palettes: Vec<Palette>,
    current_palette_index: Option<usize>,
}

impl PaletteManager {
    /// Create a new palette manager
    pub fn new() -> Self {
        Self {
            palettes: Vec::new(),
            current_palette_index: None,
        }
    }

    /// Add a palette to the manager
    pub fn add_palette(&mut self, palette: Palette) -> usize {
        self.palettes.push(palette);
        let index = self.palettes.len() - 1;

        // Set as current if it's the first palette
        if self.current_palette_index.is_none() {
            self.current_palette_index = Some(index);
        }

        index
    }

    /// Load and add a palette from a .hex file
    pub fn load_palette_from_hex<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<usize, PaletteError> {
        let palette = Palette::load_from_hex_file(path)?;
        Ok(self.add_palette(palette))
    }

    /// Load and add a palette from embedded hex data
    pub fn load_palette_from_embedded_hex(
        &mut self,
        hex_data: &str,
        name: &str,
    ) -> Result<usize, PaletteError> {
        let palette = Palette::parse_hex_content(hex_data, Some(name.to_string()))?;
        Ok(self.add_palette(palette))
    }

    /// Get the current active palette
    pub fn current_palette(&self) -> Option<&Palette> {
        self.current_palette_index
            .and_then(|index| self.palettes.get(index))
    }

    /// Set the current palette by index
    pub fn set_current_palette(&mut self, index: usize) -> Result<(), PaletteError> {
        if index < self.palettes.len() {
            self.current_palette_index = Some(index);
            Ok(())
        } else {
            Err(PaletteError::ParseError(format!(
                "Palette index {} out of range (have {} palettes)",
                index,
                self.palettes.len()
            )))
        }
    }

    /// Get all palette names
    pub fn palette_names(&self) -> Vec<String> {
        self.palettes
            .iter()
            .enumerate()
            .map(|(i, palette)| {
                palette
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("Palette {}", i))
            })
            .collect()
    }

    /// Get the number of palettes
    pub fn len(&self) -> usize {
        self.palettes.len()
    }

    /// Check if the manager is empty
    pub fn is_empty(&self) -> bool {
        self.palettes.is_empty()
    }

    /// Get palette by index
    pub fn get_palette(&self, index: usize) -> Option<&Palette> {
        self.palettes.get(index)
    }

    /// Load all .hex files from a directory
    pub fn load_palettes_from_directory<P: AsRef<Path>>(
        &mut self,
        dir: P,
    ) -> Vec<Result<usize, PaletteError>> {
        let mut results = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("hex") {
                    results.push(self.load_palette_from_hex(&path));
                }
            }
        }

        results
    }

    /// Switch to the next palette (wraps around)
    pub fn next_palette(&mut self) -> Option<usize> {
        if self.palettes.is_empty() {
            return None;
        }

        let current = self.current_palette_index.unwrap_or(0);
        let next = (current + 1) % self.palettes.len();
        self.current_palette_index = Some(next);
        Some(next)
    }

    /// Switch to the previous palette (wraps around)
    pub fn prev_palette(&mut self) -> Option<usize> {
        if self.palettes.is_empty() {
            return None;
        }

        let current = self.current_palette_index.unwrap_or(0);
        let prev = if current == 0 {
            self.palettes.len() - 1
        } else {
            current - 1
        };
        self.current_palette_index = Some(prev);
        Some(prev)
    }

    /// Get the current palette index
    pub fn current_palette_index(&self) -> Option<usize> {
        self.current_palette_index
    }

    /// Clear all palettes
    pub fn clear(&mut self) {
        self.palettes.clear();
        self.current_palette_index = None;
    }
}
