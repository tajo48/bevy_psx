use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (rotate_objects, update_settings))
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
) {
    // Spawn camera with PsxCamera component
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

    // Create materials with a variety of colors to demonstrate palette quantization
    let materials_data = [
        // Bright colors
        (Color::srgb(1.0, 0.0, 0.0), "Bright Red"),
        (Color::srgb(0.0, 1.0, 0.0), "Bright Green"),
        (Color::srgb(0.0, 0.0, 1.0), "Bright Blue"),
        (Color::srgb(1.0, 1.0, 0.0), "Yellow"),
        (Color::srgb(1.0, 0.0, 1.0), "Magenta"),
        (Color::srgb(0.0, 1.0, 1.0), "Cyan"),
        // Mid-tone colors
        (Color::srgb(0.8, 0.4, 0.2), "Orange"),
        (Color::srgb(0.4, 0.8, 0.6), "Mint"),
        (Color::srgb(0.6, 0.4, 0.8), "Purple"),
        (Color::srgb(0.9, 0.7, 0.3), "Gold"),
        // Subtle variations (these will show palette quantization most clearly)
        (Color::srgb(0.51, 0.49, 0.48), "Gray 1"),
        (Color::srgb(0.53, 0.51, 0.50), "Gray 2"),
        (Color::srgb(0.55, 0.53, 0.52), "Gray 3"),
        // Earth tones
        (Color::srgb(0.4, 0.3, 0.2), "Brown"),
        (Color::srgb(0.3, 0.4, 0.2), "Olive"),
        (Color::srgb(0.2, 0.3, 0.4), "Steel Blue"),
    ];

    let mut created_materials = Vec::new();
    for (color, _name) in materials_data.iter() {
        created_materials.push(materials.add(StandardMaterial {
            base_color: *color,
            ..default()
        }));
    }

    // Create a grid of objects with different materials
    let grid_size = 4;
    let spacing = 2.0;
    let start_offset = -(grid_size as f32 * spacing * 0.5) + spacing * 0.5;

    for (i, material_handle) in created_materials.iter().enumerate().take(16) {
        let x = (i % grid_size) as f32;
        let z = (i / grid_size) as f32;

        let position = Vec3::new(start_offset + x * spacing, 0.5, start_offset + z * spacing);

        // Alternate between different shapes
        let mesh = match i % 4 {
            0 => meshes.add(Cuboid::new(0.8, 0.8, 0.8)),
            1 => meshes.add(Sphere::new(0.4)),
            2 => meshes.add(Cylinder::new(0.4, 0.8)),
            _ => meshes.add(Cone {
                radius: 0.4,
                height: 0.8,
            }),
        };

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_translation(position),
            Rotating {
                speed: 0.5 + (i as f32 * 0.1),
            },
        ));
    }

    // Add a large floor plane with a subtle color
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(15.0, 15.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.28, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add some background objects with gradual color variations
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 0.25;
        let distance = 6.0;
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;

        // Create a gradient from dark to light
        let intensity = (i as f32) / 7.0;
        let color = Color::srgb(
            0.2 + intensity * 0.6,
            0.1 + intensity * 0.4,
            0.3 + intensity * 0.5,
        );

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_xyz(x, 1.0, z),
        ));
    }

    // Print instructions
    println!("\n=== PSX Palette Quantization Demo ===");
    println!("This demo shows how colors are quantized to a limited PSX-style palette");
    println!("Notice how similar colors get mapped to the same palette colors!");
    println!("\nControls:");
    println!("  P - Toggle palette quantization on/off");
    println!("  Q - Decrease quantization steps (more posterized)");
    println!("  E - Increase quantization steps (smoother gradients)");
    println!("  1 - PSX resolution (320x240)");
    println!("  2 - PS2 resolution (512x448)");
    println!("  3 - High resolution (800x600)");
    println!("  R - Toggle pixelated/smooth filtering");
    println!("=====================================\n");

    println!("Current palette has {} colors", 64);
    println!("Watch how the subtle gray variations get quantized to the same colors!");
}

fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * rotating.speed);
    }
}

fn update_settings(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut psx_settings: ResMut<PsxRenderSettings>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
) {
    // Change resolution with number keys
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        psx_settings.render_resolution = UVec2::new(320, 240);
        println!("Switched to PSX resolution (320x240)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        psx_settings.render_resolution = UVec2::new(512, 448);
        println!("Switched to PS2 resolution (512x448)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        psx_settings.render_resolution = UVec2::new(800, 600);
        println!("Switched to high resolution (800x600)");
    }

    // Toggle pixelated mode with R key
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        psx_settings.pixelated = !psx_settings.pixelated;
        println!(
            "Pixelated mode: {}",
            if psx_settings.pixelated { "ON" } else { "OFF" }
        );
    }

    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        palette_settings.use_palette = !palette_settings.use_palette;
        println!(
            "Palette quantization: {}",
            if palette_settings.use_palette {
                "ON"
            } else {
                "OFF"
            }
        );

        if palette_settings.use_palette {
            println!("Colors will be mapped to the nearest PSX palette color");
        } else {
            println!("Using original colors (basic quantization only)");
        }
    }

    // Adjust quantization steps
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        if palette_settings.quantize_steps > 8 {
            palette_settings.quantize_steps -= 8;
            println!(
                "Quantization steps: {} (more posterized)",
                palette_settings.quantize_steps
            );
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if palette_settings.quantize_steps < 128 {
            palette_settings.quantize_steps += 8;
            println!(
                "Quantization steps: {} (smoother gradients)",
                palette_settings.quantize_steps
            );
        }
    }

    // Show current settings
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("\n--- Current Settings ---");
        println!(
            "Resolution: {}x{}",
            psx_settings.render_resolution.x, psx_settings.render_resolution.y
        );
        println!("Pixelated: {}", psx_settings.pixelated);
        println!("Palette Quantization: {}", palette_settings.use_palette);
        println!("Quantization Steps: {}", palette_settings.quantize_steps);
        println!("Palette Enabled: {}", palette_settings.enabled);
        println!("------------------------\n");
    }
}
