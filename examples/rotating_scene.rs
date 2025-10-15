use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                rotate_objects,
                handle_unified_controls,
                handle_palette_switching,
            ),
        )
        .run();
}

#[derive(Component)]
struct Rotating {
    speed: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut unified_settings: ResMut<PsxUnifiedSettings>,
) {
    // Print control instructions
    println!("=== PSX Rotating Scene Demo Controls ===");
    println!("V: Toggle vertex snapping");
    println!("P: Toggle palette quantization");
    println!("D: Toggle dithering");
    println!("Q: Toggle basic quantization");
    println!("E/R: Adjust quantization steps");
    println!("T/Y: Adjust dither strength");
    println!("G/H: Switch dither patterns");
    println!("N/M: Switch palettes");
    println!("========================================");

    // Enable unified shader effects by default for the demo
    unified_settings.use_palette = true;
    unified_settings.dither_enabled = true;
    unified_settings.quantize_enabled = true;
    unified_settings.dither_strength = 0.2;

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

    // Print instructions
    println!("PSX Camera Demo - Rotating Scene with Vertex Snapping");
    println!("-----------------------------------------------------");
    println!("The scene is rendered at PSX resolution with automatic aspect ratio matching.");
    println!("Resolution adjusts to match your window's aspect ratio while staying low-res!");
    println!("MSAA is automatically disabled for authentic PSX rendering");
    println!("All 3D models automatically have PSX vertex snapping applied!");
    println!("Notice the pixelated, retro look and vertex jitter characteristic of PSX games!");
    println!("🎨 Palettes are ON for this demo - colors will be quantized!");
    println!();
    println!("Aspect ratio matching is ON by default:");
    println!("- Wide windows (16:9, 21:9): Keeps height at 240px, adjusts width");
    println!("- Tall windows (portrait): Keeps width at 320px, adjusts height");
    println!("- Square windows (1:1): Uses base PSX resolution (320x240)");
    println!();
    println!("Controls:");
    println!("  P - Toggle palette quantization on/off");
    println!("  N - Switch to next palette");
    println!("  M - Switch to previous palette");
}

fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * rotating.speed);
    }
}

fn handle_unified_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut vertex_snap_settings: ResMut<PsxVertexSnapSettings>,
    mut unified_settings: ResMut<PsxUnifiedSettings>,
) {
    // Vertex snapping controls
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        vertex_snap_settings.enabled = !vertex_snap_settings.enabled;
        println!(
            "Vertex snapping: {}",
            if vertex_snap_settings.enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        unified_settings.use_palette = !unified_settings.use_palette;
        println!(
            "Palette quantization: {} ",
            if unified_settings.use_palette {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle dithering with D key
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        unified_settings.dither_enabled = !unified_settings.dither_enabled;
        println!(
            "Dithering: {} ",
            if unified_settings.dither_enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Toggle basic quantization with Q key
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        unified_settings.quantize_enabled = !unified_settings.quantize_enabled;
        println!(
            "Basic quantization: {} ",
            if unified_settings.quantize_enabled {
                "ON"
            } else {
                "OFF"
            }
        );
    }

    // Adjust quantization steps with E/R keys
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if unified_settings.quantize_steps > 8 {
            unified_settings.quantize_steps -= 8;
            println!(
                "Quantization steps: {} (more posterized)",
                unified_settings.quantize_steps
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if unified_settings.quantize_steps < 128 {
            unified_settings.quantize_steps += 8;
            println!(
                "Quantization steps: {} (smoother gradients)",
                unified_settings.quantize_steps
            );
        }
    }

    // Adjust dither strength with T/Y keys
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if unified_settings.dither_strength > 0.1 {
            unified_settings.dither_strength -= 0.1;
            println!(
                "Dither strength: {:.1} (less dithering)",
                unified_settings.dither_strength
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyY) {
        if unified_settings.dither_strength < 1.0 {
            unified_settings.dither_strength += 0.1;
            println!(
                "Dither strength: {:.1} (more dithering)",
                unified_settings.dither_strength
            );
        }
    }

    // Switch dither patterns with G/H keys
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        unified_settings.dither_pattern = match unified_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Random,
            DitherPattern::Bayer8x8 => DitherPattern::Bayer4x4,
            DitherPattern::BlueNoise => DitherPattern::Bayer8x8,
            DitherPattern::Random => DitherPattern::BlueNoise,
        };
        println!("Dither pattern: {:?}", unified_settings.dither_pattern);
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        unified_settings.dither_pattern = match unified_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Bayer8x8,
            DitherPattern::Bayer8x8 => DitherPattern::BlueNoise,
            DitherPattern::BlueNoise => DitherPattern::Random,
            DitherPattern::Random => DitherPattern::Bayer4x4,
        };
        println!("Dither pattern: {:?}", unified_settings.dither_pattern);
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
