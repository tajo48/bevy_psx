use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, rotate_objects)
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
    println!();
    println!("Aspect ratio matching is ON by default:");
    println!("- Wide windows (16:9, 21:9): Keeps height at 240px, adjusts width");
    println!("- Tall windows (portrait): Keeps width at 320px, adjusts height");
    println!("- Square windows (1:1): Uses base PSX resolution (320x240)");
}

fn rotate_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * rotating.speed);
    }
}
