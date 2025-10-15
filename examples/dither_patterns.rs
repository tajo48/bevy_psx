//! # PSX Dither Patterns Demo
//!
//! This example demonstrates all available dither patterns in the bevy_psx plugin.
//! It shows how different dither patterns affect the visual quality and appearance
//! of rendered objects with various settings.
//!
//! ## Dither Patterns Available
//! - **Bayer 4x4**: Classic ordered dithering pattern (default)
//! - **Bayer 8x8**: Larger ordered dithering pattern for smoother gradients
//! - **Blue Noise**: High-quality dithering with reduced visual artifacts
//! - **Random**: Pseudo-random dithering for organic noise
//!
//! ## Controls
//! - **G/H**: Switch between dither patterns
//! - **D**: Toggle dithering on/off
//! - **T/Y**: Adjust dither strength
//! - **P**: Toggle palette quantization
//! - **Q**: Toggle basic quantization
//! - **E/R**: Adjust quantization steps
//! - **N/M**: Switch color palettes
//! - **1/2/3**: Change scenes to test different scenarios
//!
//! ## Test Scenes
//! 1. **Gradient Spheres**: Perfect for seeing dither pattern differences
//! 2. **Textured Objects**: Shows dithering with texture mapping
//! 3. **Mixed Materials**: Complex scene with various surface types

use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .init_resource::<DemoScene>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (handle_controls, animate_objects, update_ui, switch_scenes),
        )
        .run();
}

#[derive(Resource)]
struct DemoScene {
    current_scene: u8,
    scene_entities: Vec<Entity>,
}

impl Default for DemoScene {
    fn default() -> Self {
        Self {
            current_scene: 1,
            scene_entities: Vec::new(),
        }
    }
}

#[derive(Component)]
struct DemoObject {
    rotation_speed: Vec3,
    oscillation: Vec3,
    base_position: Vec3,
}

#[derive(Component)]
struct DitherInfoText;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut psx_settings: ResMut<PsxSettings>,
    mut demo_scene: ResMut<DemoScene>,
    _asset_server: Res<AssetServer>,
) {
    // Configure PSX settings for optimal dither demonstration
    psx_settings.use_palette = true;
    psx_settings.dither_enabled = true;
    psx_settings.quantize_enabled = false;
    psx_settings.dither_strength = 0.1;
    psx_settings.dither_pattern = DitherPattern::Bayer8x8;
    psx_settings.snap_enabled = true;

    // Print instructions
    print_instructions();

    // Setup camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,
    ));

    // Setup lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.15, 0.15, 0.2),
        brightness: 0.3,
        ..default()
    });

    // Setup UI
    setup_ui(&mut commands);

    // Setup initial scene
    spawn_gradient_spheres(&mut commands, &mut meshes, &mut materials, &mut demo_scene);
}

fn setup_ui(commands: &mut Commands) {
    // UI Root
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PSX Dither Patterns Demo"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Dither info display
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                DitherInfoText,
            ));

            // Controls help
            parent.spawn((
                Text::new(
                    "DITHER CONTROLS:\n\
                     G/H: Switch dither patterns • D: Toggle dithering • T/Y: Dither strength\n\
                     P: Toggle palette • Q: Toggle quantization • E/R: Quantization steps\n\
                     N/M: Switch palettes • 1/2/3: Change test scenes",
                ),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_gradient_spheres(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    demo_scene: &mut ResMut<DemoScene>,
) {
    // Clear existing scene
    demo_scene.scene_entities.clear();

    // Create spheres with gradient colors to showcase dithering
    let colors = [
        Color::srgb(1.0, 0.0, 0.0), // Red
        Color::srgb(1.0, 0.5, 0.0), // Orange
        Color::srgb(1.0, 1.0, 0.0), // Yellow
        Color::srgb(0.0, 1.0, 0.0), // Green
        Color::srgb(0.0, 1.0, 1.0), // Cyan
        Color::srgb(0.0, 0.0, 1.0), // Blue
        Color::srgb(0.5, 0.0, 1.0), // Purple
        Color::srgb(1.0, 0.0, 1.0), // Magenta
    ];

    for (i, color) in colors.iter().enumerate() {
        let angle = i as f32 * std::f32::consts::TAU / colors.len() as f32;
        let radius = 4.0;
        let position = Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);

        let entity = commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(0.8))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: *color,
                    metallic: 0.0,
                    perceptual_roughness: 0.5,
                    ..default()
                })),
                Transform::from_translation(position),
                DemoObject {
                    rotation_speed: Vec3::new(0.5, 1.0, 0.3) * (i as f32 + 1.0) * 0.2,
                    oscillation: Vec3::new(0.0, (i as f32 + 1.0) * 0.3, 0.0),
                    base_position: position,
                },
            ))
            .id();

        demo_scene.scene_entities.push(entity);
    }

    // Add central gradient cube
    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                metallic: 0.3,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            DemoObject {
                rotation_speed: Vec3::new(0.2, 0.3, 0.1),
                oscillation: Vec3::ZERO,
                base_position: Vec3::ZERO,
            },
        ))
        .id();

    demo_scene.scene_entities.push(entity);
}

fn spawn_textured_objects(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    demo_scene: &mut ResMut<DemoScene>,
    _asset_server: &Res<AssetServer>,
) {
    // Clear existing scene
    demo_scene.scene_entities.clear();

    // Create objects with different surface properties
    let objects = [
        (
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            Vec3::new(-2.0, 0.0, -2.0),
            Color::srgb(0.8, 0.2, 0.2),
            0.0,
            0.8,
        ),
        (
            Mesh3d(meshes.add(Sphere::new(0.6))),
            Vec3::new(2.0, 0.0, -2.0),
            Color::srgb(0.2, 0.8, 0.2),
            0.8,
            0.2,
        ),
        (
            Mesh3d(meshes.add(Cylinder::new(0.5, 1.2))),
            Vec3::new(-2.0, 0.0, 2.0),
            Color::srgb(0.2, 0.2, 0.8),
            0.5,
            0.5,
        ),
        (
            Mesh3d(meshes.add(Torus::new(0.3, 0.8))),
            Vec3::new(2.0, 0.0, 2.0),
            Color::srgb(0.8, 0.8, 0.2),
            0.2,
            0.9,
        ),
    ];

    for (i, (mesh, position, color, metallic, perceptual_roughness)) in objects.iter().enumerate() {
        let entity = commands
            .spawn((
                mesh.clone(),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: *color,
                    metallic: *metallic,
                    perceptual_roughness: *perceptual_roughness,
                    ..default()
                })),
                Transform::from_translation(*position),
                DemoObject {
                    rotation_speed: Vec3::new(0.3, 0.5, 0.2) * (i as f32 + 1.0) * 0.5,
                    oscillation: Vec3::new(0.0, 0.5 + i as f32 * 0.2, 0.0),
                    base_position: *position,
                },
            ))
            .id();

        demo_scene.scene_entities.push(entity);
    }

    // Add ground plane
    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.6, 0.6, 0.7),
                metallic: 0.1,
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(0.0, -1.0, 0.0),
        ))
        .id();

    demo_scene.scene_entities.push(entity);
}

fn spawn_mixed_scene(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    demo_scene: &mut ResMut<DemoScene>,
) {
    // Clear existing scene
    demo_scene.scene_entities.clear();

    // Create a complex scene with various materials and colors
    for i in 0..12 {
        let angle = i as f32 * std::f32::consts::TAU / 12.0;
        let radius = 3.0 + (i % 3) as f32 * 1.0;
        let height = ((i % 4) as f32 - 1.5) * 0.8;
        let position = Vec3::new(angle.cos() * radius, height, angle.sin() * radius);

        let (mesh, color, metallic, perceptual_roughness) = match i % 4 {
            0 => (
                meshes.add(Cuboid::new(0.6, 0.6, 0.6)),
                Color::srgb(0.9, 0.3, 0.1),
                0.0,
                0.8,
            ),
            1 => (
                meshes.add(Sphere::new(0.4)),
                Color::srgb(0.1, 0.9, 0.3),
                0.6,
                0.4,
            ),
            2 => (
                meshes.add(Cylinder::new(0.3, 0.8)),
                Color::srgb(0.3, 0.1, 0.9),
                0.8,
                0.2,
            ),
            _ => (
                meshes.add(Cone {
                    radius: 0.3,
                    height: 0.8,
                }),
                Color::srgb(0.9, 0.9, 0.1),
                0.2,
                0.9,
            ),
        };

        let entity = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    metallic,
                    perceptual_roughness,
                    ..default()
                })),
                Transform::from_translation(position),
                DemoObject {
                    rotation_speed: Vec3::new(
                        (i as f32 * 0.1 + 0.2).sin(),
                        (i as f32 * 0.15 + 0.3).cos(),
                        (i as f32 * 0.05 + 0.1).sin(),
                    ),
                    oscillation: Vec3::new(0.0, (i as f32 + 1.0) * 0.15, 0.0),
                    base_position: position,
                },
            ))
            .id();

        demo_scene.scene_entities.push(entity);
    }
}

fn handle_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut psx_settings: ResMut<PsxSettings>,
    mut palette_manager: ResMut<PaletteManager>,
) {
    // Switch dither patterns with G/H keys
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        psx_settings.dither_pattern = match psx_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Bayer8x8,
            DitherPattern::Bayer8x8 => DitherPattern::BlueNoise,
            DitherPattern::BlueNoise => DitherPattern::Random,
            DitherPattern::Random => DitherPattern::Bayer4x4,
        };
        println!("Dither pattern: {:?}", psx_settings.dither_pattern);
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        psx_settings.dither_pattern = match psx_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Random,
            DitherPattern::Bayer8x8 => DitherPattern::Bayer4x4,
            DitherPattern::BlueNoise => DitherPattern::Bayer8x8,
            DitherPattern::Random => DitherPattern::BlueNoise,
        };
        println!("Dither pattern: {:?}", psx_settings.dither_pattern);
    }

    // Toggle dithering with D key
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        psx_settings.dither_enabled = !psx_settings.dither_enabled;
        println!(
            "Dithering: {}",
            if psx_settings.dither_enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Adjust dither strength with T/Y keys
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if psx_settings.dither_strength > 0.05 {
            psx_settings.dither_strength -= 0.05;
            println!("Dither strength: {:.2}", psx_settings.dither_strength);
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyY) {
        if psx_settings.dither_strength < 1.0 {
            psx_settings.dither_strength += 0.05;
            println!("Dither strength: {:.2}", psx_settings.dither_strength);
        }
    }

    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        psx_settings.use_palette = !psx_settings.use_palette;
        println!(
            "Palette quantization: {}",
            if psx_settings.use_palette {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle basic quantization with Q key
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        psx_settings.quantize_enabled = !psx_settings.quantize_enabled;
        println!(
            "Basic quantization: {}",
            if psx_settings.quantize_enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Adjust quantization steps with E/R keys
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if psx_settings.quantize_steps > 8 {
            psx_settings.quantize_steps -= 8;
            println!("Quantization steps: {}", psx_settings.quantize_steps);
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if psx_settings.quantize_steps < 128 {
            psx_settings.quantize_steps += 8;
            println!("Quantization steps: {}", psx_settings.quantize_steps);
        }
    }

    // Switch palettes with N/M keys
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        palette_manager.next_palette();
        if let Some(palette) = palette_manager.current_palette() {
            let name = palette.name.as_deref().unwrap_or("Unknown");
            println!("Switched to palette: {} ({} colors)", name, palette.len());
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        palette_manager.prev_palette();
        if let Some(palette) = palette_manager.current_palette() {
            let name = palette.name.as_deref().unwrap_or("Unknown");
            println!("Switched to palette: {} ({} colors)", name, palette.len());
        }
    }
}

fn switch_scenes(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut demo_scene: ResMut<DemoScene>,
    asset_server: Res<AssetServer>,
    entities: Query<Entity, With<DemoObject>>,
) {
    let mut scene_changed = false;
    let mut new_scene = demo_scene.current_scene;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        new_scene = 1;
        scene_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        new_scene = 2;
        scene_changed = true;
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        new_scene = 3;
        scene_changed = true;
    }

    if scene_changed && new_scene != demo_scene.current_scene {
        // Despawn existing objects
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }

        demo_scene.current_scene = new_scene;

        match new_scene {
            1 => {
                println!("Switched to Scene 1: Gradient Spheres");
                spawn_gradient_spheres(&mut commands, &mut meshes, &mut materials, &mut demo_scene);
            }
            2 => {
                println!("Switched to Scene 2: Textured Objects");
                spawn_textured_objects(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut demo_scene,
                    &asset_server,
                );
            }
            3 => {
                println!("Switched to Scene 3: Mixed Materials");
                spawn_mixed_scene(&mut commands, &mut meshes, &mut materials, &mut demo_scene);
            }
            _ => {}
        }
    }
}

fn animate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &DemoObject)>) {
    let time_secs = time.elapsed_secs();

    for (mut transform, demo_object) in query.iter_mut() {
        // Apply rotation
        transform.rotate_x(demo_object.rotation_speed.x * time.delta_secs());
        transform.rotate_y(demo_object.rotation_speed.y * time.delta_secs());
        transform.rotate_z(demo_object.rotation_speed.z * time.delta_secs());

        // Apply oscillation
        let oscillation_offset = Vec3::new(
            (time_secs * demo_object.oscillation.x).sin() * 0.3,
            (time_secs * demo_object.oscillation.y).sin() * 0.5,
            (time_secs * demo_object.oscillation.z).sin() * 0.3,
        );

        transform.translation = demo_object.base_position + oscillation_offset;
    }
}

fn update_ui(
    mut text_query: Query<&mut Text, With<DitherInfoText>>,
    psx_settings: Res<PsxSettings>,
    palette_manager: Res<PaletteManager>,
    demo_scene: Res<DemoScene>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        let current_palette = palette_manager
            .current_palette()
            .map(|p| p.name.as_deref().unwrap_or("Unknown"))
            .unwrap_or("None");

        let scene_name = match demo_scene.current_scene {
            1 => "Gradient Spheres",
            2 => "Textured Objects",
            3 => "Mixed Materials",
            _ => "Unknown",
        };

        **text = format!(
            "CURRENT SCENE: {} (Scene {})\n\n\
             DITHER SETTINGS:\n\
             Pattern: {:?} | Enabled: {} | Strength: {:.2}\n\n\
             QUANTIZATION:\n\
             Basic: {} (Steps: {}) | Palette: {}\n\
             Current Palette: {} ({} colors)",
            scene_name,
            demo_scene.current_scene,
            psx_settings.dither_pattern,
            if psx_settings.dither_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_settings.dither_strength,
            if psx_settings.quantize_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_settings.quantize_steps,
            if psx_settings.use_palette {
                "ON"
            } else {
                "OFF"
            },
            current_palette,
            palette_manager
                .current_palette()
                .map(|p| p.len())
                .unwrap_or(0),
        );
    }
}

fn print_instructions() {
    println!("🎨 === PSX DITHER PATTERNS DEMO ===");
    println!();
    println!("This demo showcases all available dither patterns:");
    println!("• Bayer 4x4: Classic ordered dithering (default)");
    println!("• Bayer 8x8: Larger pattern for smoother gradients");
    println!("• Blue Noise: High-quality dithering with reduced artifacts");
    println!("• Random: Pseudo-random dithering for organic noise");
    println!();
    println!("🎮 CONTROLS:");
    println!("G/H         - Switch dither patterns (forward/backward)");
    println!("D           - Toggle dithering on/off");
    println!("T/Y         - Decrease/increase dither strength");
    println!("P           - Toggle palette quantization");
    println!("Q           - Toggle basic quantization");
    println!("E/R         - Decrease/increase quantization steps");
    println!("N/M         - Switch color palettes");
    println!("1/2/3       - Change test scenes");
    println!();
    println!("🎭 TEST SCENES:");
    println!("1 - Gradient Spheres: Best for seeing dither pattern differences");
    println!("2 - Textured Objects: Shows dithering with different materials");
    println!("3 - Mixed Materials: Complex scene with various surface types");
    println!();
    println!("💡 TIP: Try different combinations of settings to see how");
    println!("   dither patterns interact with palettes and quantization!");
    println!("=====================================");
}
