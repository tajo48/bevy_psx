use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rotate_cube,
                move_sphere,
                update_settings,
                handle_palette_switching,
            ),
        )
        .run();
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
struct MovingSphere;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
) {
    // Enable palettes for this demo!
    palette_settings.use_palette = true;
    // Spawn camera with PsxCamera component - this is all you need!
    // MSAA is automatically disabled for authentic PSX look
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 2.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        PsxCamera,
    ));

    // Add lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add a rotating cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));

    // Add a moving sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2),
            ..default()
        })),
        Transform::from_xyz(2.0, 0.5, 0.0),
        MovingSphere,
    ));

    // Add a floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Print instructions
    println!("\n=== PSX Camera Demo with Vertex Snapping ===");
    println!("The scene is rendered at PSX resolution with automatic aspect ratio matching");
    println!("All 3D models automatically have PSX vertex snapping applied!");
    println!("🎨 PALETTES are ON - full PSX effects!");
    println!("\nControls:");
    println!("  1 - PSX resolution (320x240)");
    println!("  2 - PS2 resolution (512x448)");
    println!("  3 - High resolution (800x600)");
    println!("  A - Toggle aspect ratio matching on/off");
    println!("  R - Toggle pixelated/smooth filtering");
    println!("  V - Increase vertex snap amount (smoother)");
    println!("  B - Decrease vertex snap amount (more jittery)");
    println!("  T - Toggle vertex snapping on/off");
    println!("  P - Toggle palette quantization on/off ");
    println!("  Q - Decrease quantization steps (more posterized)");
    println!("  E - Increase quantization steps (smoother gradients)");
    println!("  N - Switch to next palette");
    println!("  M - Switch to previous palette");
    println!("======================\n");
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * 1.0);
        transform.rotate_x(time.delta_secs() * 0.5);
    }
}

fn move_sphere(time: Res<Time>, mut query: Query<&mut Transform, With<MovingSphere>>) {
    for mut transform in query.iter_mut() {
        let t = time.elapsed_secs();
        transform.translation.x = t.sin() * 2.0;
        transform.translation.z = t.cos() * 2.0;
        transform.translation.y = 0.5 + (t * 2.0).sin() * 0.3;
    }
}

fn update_settings(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut psx_settings: ResMut<PsxRenderSettings>,
    mut vertex_snap_settings: ResMut<PsxVertexSnapSettings>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
    windows: Query<&Window>,
) {
    // Change resolution with number keys
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        psx_settings.base_resolution = UVec2::new(320, 240);
        psx_settings.render_resolution = UVec2::new(320, 240);
        println!("Switched to PSX resolution (320x240)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        psx_settings.base_resolution = UVec2::new(512, 448);
        psx_settings.render_resolution = UVec2::new(512, 448);
        println!("Switched to PS2 resolution (512x448)");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        psx_settings.base_resolution = UVec2::new(800, 600);
        psx_settings.render_resolution = UVec2::new(800, 600);
        println!("Switched to high resolution (800x600)");
    }

    // Toggle aspect ratio matching with A key
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        psx_settings.aspect_ratio_matching = !psx_settings.aspect_ratio_matching;

        // Show current window info for demonstration
        if let Ok(window) = windows.single() {
            let window_aspect = window.width() / window.height();
            let base_aspect =
                psx_settings.base_resolution.x as f32 / psx_settings.base_resolution.y as f32;

            println!(
                "Aspect ratio matching: {} (resolution will {})",
                if psx_settings.aspect_ratio_matching {
                    "ON"
                } else {
                    "OFF"
                },
                if psx_settings.aspect_ratio_matching {
                    "adjust to window aspect ratio"
                } else {
                    "use fixed resolution"
                }
            );
            println!(
                "  Window: {:.2}x{:.0} (aspect {:.2})",
                window.width(),
                window.height(),
                window_aspect
            );
            println!(
                "  Base resolution: {}x{} (aspect {:.2})",
                psx_settings.base_resolution.x, psx_settings.base_resolution.y, base_aspect
            );
            println!(
                "  Current render resolution: {}x{}",
                psx_settings.render_resolution.x, psx_settings.render_resolution.y
            );
        }
    }

    // Toggle pixelated mode with R key
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        psx_settings.pixelated = !psx_settings.pixelated;
        println!(
            "Pixelated mode: {}",
            if psx_settings.pixelated { "ON" } else { "OFF" }
        );
    }

    // Vertex snapping controls
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        vertex_snap_settings.snap_amount += 16.0;
        vertex_snap_settings.snap_amount = vertex_snap_settings.snap_amount.min(512.0);
        println!(
            "Vertex snap amount: {:.1} (smoother)",
            vertex_snap_settings.snap_amount
        );
    }

    if keyboard_input.just_pressed(KeyCode::KeyB) {
        vertex_snap_settings.snap_amount -= 16.0;
        vertex_snap_settings.snap_amount = vertex_snap_settings.snap_amount.max(16.0);
        println!(
            "Vertex snap amount: {:.1} (more jittery)",
            vertex_snap_settings.snap_amount
        );
    }

    if keyboard_input.just_pressed(KeyCode::KeyT) {
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
        palette_settings.use_palette = !palette_settings.use_palette;
        println!(
            "Palette quantization: {} ",
            if palette_settings.use_palette {
                "ON"
            } else {
                "OFF"
            }
        );
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
}

fn handle_palette_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut palette_manager: ResMut<PaletteManager>,
    palette_settings: Res<PsxPaletteSettings>,
) {
    // Only handle palette switching if palettes are on
    if !palette_settings.use_palette {
        return;
    }

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
