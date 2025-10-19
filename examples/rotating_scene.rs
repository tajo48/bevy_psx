use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .insert_resource(UiVisibility { visible: true })
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(
            Update,
            (
                rotate_objects,
                handle_unified_controls,
                handle_palette_switching,
                toggle_ui_visibility,
                update_ui_text,
            ),
        )
        .run();
}

#[derive(Component)]
struct Rotating {
    speed: f32,
}

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct SettingsText;

#[derive(Resource)]
struct UiVisibility {
    visible: bool,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut psx_settings: ResMut<PsxSettings>,
) {
    // Print control instructions
    println!("=== PSX Rotating Scene Demo Controls ===");
    println!("U: Toggle UI");
    println!("V: Toggle vertex snapping");
    println!("P: Toggle palette quantization");
    println!("D: Toggle dithering");
    println!("Q: Toggle basic quantization");
    println!("E/R: Adjust quantization steps");
    println!("T/Y: Adjust dither strength");
    println!("G/H: Switch dither patterns");
    println!("N/M: Switch palettes");
    println!("========================================");

    // Enable PSX effects by default for the demo
    psx_settings.use_palette = true;
    psx_settings.dither_enabled = true;
    psx_settings.quantize_enabled = true;
    psx_settings.dither_strength = 0.2;
    psx_settings.snap_enabled = true;

    // Spawn camera with PsxCamera component
    // MSAA is automatically disabled for authentic PSX look
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,
    ));

    // Add a light
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 0.3,
        ..default()
    });

    // Create materials with different colors
    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        ..default()
    });

    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2),
        ..default()
    });

    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.8),
        ..default()
    });

    let yellow_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.2),
        ..default()
    });

    // Spawn rotating cubes in a pattern
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(red_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotating { speed: 1.0 },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(green_material),
        Transform::from_xyz(2.0, 0.25, 0.0),
        Rotating { speed: -1.5 },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(blue_material),
        Transform::from_xyz(-2.0, 0.25, 0.0),
        Rotating { speed: 1.5 },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(yellow_material.clone()),
        Transform::from_xyz(0.0, 0.25, 2.0),
        Rotating { speed: -1.2 },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(yellow_material),
        Transform::from_xyz(0.0, 0.25, -2.0),
        Rotating { speed: 1.2 },
    ));

    // Add a pyramid (using a cone as approximation)
    let purple_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.2, 0.8),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cone {
            radius: 0.7,
            height: 1.5,
        })),
        MeshMaterial3d(purple_material),
        Transform::from_xyz(-1.5, 0.75, -1.5),
        Rotating { speed: 0.8 },
    ));

    // Add a sphere
    let cyan_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.8),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(cyan_material),
        Transform::from_xyz(1.5, 0.4, -1.5),
        Rotating { speed: -0.6 },
    ));

    // Add a floor plane
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(floor_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add some pillars
    let pillar_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.6),
        ..default()
    });

    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI * 0.5;
        let x = angle.cos() * 3.5;
        let z = angle.sin() * 3.5;

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.2, 2.0))),
            MeshMaterial3d(pillar_material.clone()),
            Transform::from_xyz(x, 1.0, z),
        ));
    }

    println!("PSX Camera Demo - Rotating Scene with Vertex Snapping");
    println!("-----------------------------------------------------");
    println!("The scene is rendered at PSX resolution with automatic aspect ratio matching.");
    println!("Resolution adjusts to match your window's aspect ratio while staying low-res!");
    println!("MSAA is automatically disabled for authentic PSX rendering");
    println!("All 3D models automatically have PSX vertex snapping applied!");
    println!("Notice the pixelated, retro look and vertex jitter characteristic of PSX games!");
    println!("🎨 Palettes are ON for this demo - colors will be quantized!");
    println!();
    println!("Press 'U' to toggle the UI overlay on/off");
}

fn setup_ui(mut commands: Commands) {
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
            UiRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("PSX Rotating Scene Demo"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Settings display
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                SettingsText,
            ));

            // Controls help
            parent.spawn((
                Text::new(
                    "CONTROLS:\n\
                     U: Toggle UI\n\
                     V: Toggle vertex snapping\n\
                     P: Toggle palette quantization\n\
                     D: Toggle dithering\n\
                     Q: Toggle basic quantization\n\
                     E/R: Adjust quantization steps\n\
                     T/Y: Adjust dither strength\n\
                     G/H: Switch dither patterns\n\
                     N/M: Switch palettes",
                ),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * rotating.speed);
    }
}

fn toggle_ui_visibility(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_visibility: ResMut<UiVisibility>,
    mut ui_query: Query<&mut Visibility, With<UiRoot>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        ui_visibility.visible = !ui_visibility.visible;
        for mut visibility in ui_query.iter_mut() {
            *visibility = if ui_visibility.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn update_ui_text(
    mut text_query: Query<&mut Text, With<SettingsText>>,
    psx_settings: Res<PsxSettings>,
    palette_manager: Res<PaletteManager>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        let current_palette = palette_manager
            .current_palette()
            .map(|p| p.name.as_deref().unwrap_or("Unknown"))
            .unwrap_or("None");

        **text = format!(
            "PSX SETTINGS:\n\
             Vertex Snapping: {}\n\
             Basic Quantization: {} (Steps: {})\n\
             Palette Quantization: {}\n\
             Current Palette: {} ({} colors)\n\
             Dithering: {} (Strength: {:.1})\n\
             Dither Pattern: {:?}",
            if psx_settings.snap_enabled {
                "ON"
            } else {
                "OFF"
            },
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
            if psx_settings.dither_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_settings.dither_strength,
            psx_settings.dither_pattern,
        );
    }
}

fn handle_unified_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut psx_settings: ResMut<PsxSettings>,
) {
    // Vertex snapping controls
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        psx_settings.snap_enabled = !psx_settings.snap_enabled;
        println!(
            "Vertex snapping: {}",
            if psx_settings.snap_enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        psx_settings.use_palette = !psx_settings.use_palette;
        println!(
            "Palette quantization: {} ",
            if psx_settings.use_palette {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle dithering with D key
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        psx_settings.dither_enabled = !psx_settings.dither_enabled;
        println!(
            "Dithering: {} ",
            if psx_settings.dither_enabled {
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
            "Basic quantization: {} ",
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
            println!(
                "Quantization steps: {} (more posterized)",
                psx_settings.quantize_steps
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if psx_settings.quantize_steps < 128 {
            psx_settings.quantize_steps += 8;
            println!(
                "Quantization steps: {} (smoother gradients)",
                psx_settings.quantize_steps
            );
        }
    }

    // Adjust dither strength with T/Y keys
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if psx_settings.dither_strength > 0.1 {
            psx_settings.dither_strength -= 0.1;
            println!(
                "Dither strength: {:.1} (less dithering)",
                psx_settings.dither_strength
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyY) {
        if psx_settings.dither_strength < 1.0 {
            psx_settings.dither_strength += 0.1;
            println!(
                "Dither strength: {:.1} (more dithering)",
                psx_settings.dither_strength
            );
        }
    }

    // Switch dither patterns with G/H keys
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        psx_settings.dither_pattern = match psx_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Random,
            DitherPattern::Bayer8x8 => DitherPattern::Bayer4x4,
            DitherPattern::BlueNoise => DitherPattern::Bayer8x8,
            DitherPattern::Random => DitherPattern::BlueNoise,
        };
        println!("Dither pattern: {:?}", psx_settings.dither_pattern);
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        psx_settings.dither_pattern = match psx_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Bayer8x8,
            DitherPattern::Bayer8x8 => DitherPattern::BlueNoise,
            DitherPattern::BlueNoise => DitherPattern::Random,
            DitherPattern::Random => DitherPattern::Bayer4x4,
        };
        println!("Dither pattern: {:?}", psx_settings.dither_pattern);
    }
}

fn handle_palette_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut palette_manager: ResMut<PaletteManager>,
) {
    // Switch to next palette with N key
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        if let Some(index) = palette_manager.next_palette() {
            if let Some(palette) = palette_manager.get_palette(index) {
                let name = palette.name.as_deref().unwrap_or("Unknown");
                println!(
                    "✓ Switched to palette: {} ({} colors) [Index: {}]",
                    name,
                    palette.len(),
                    index
                );
            }
        } else {
            println!("No palettes available to switch to");
        }
    }

    // Switch to previous palette with M key
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if let Some(index) = palette_manager.prev_palette() {
            if let Some(palette) = palette_manager.get_palette(index) {
                let name = palette.name.as_deref().unwrap_or("Unknown");
                println!(
                    "✓ Switched to palette: {} ({} colors) [Index: {}]",
                    name,
                    palette.len(),
                    index
                );
            }
        } else {
            println!("No palettes available to switch to");
        }
    }
}
