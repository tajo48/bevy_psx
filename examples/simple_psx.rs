use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, move_sphere, update_settings))
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
) {
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
    println!("The scene is rendered at PSX resolution (320x240) by default");
    println!("All 3D models automatically have PSX vertex snapping applied!");
    println!("\nControls:");
    println!("  1 - PSX resolution (320x240)");
    println!("  2 - PS2 resolution (512x448)");
    println!("  3 - High resolution (800x600)");
    println!("  P - Toggle pixelated/smooth filtering");
    println!("  V - Increase vertex snap amount (smoother)");
    println!("  B - Decrease vertex snap amount (more jittery)");
    println!("  T - Toggle vertex snapping on/off");
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

    // Toggle pixelated mode with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
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
}
