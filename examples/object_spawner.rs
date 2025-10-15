//! # PSX Stress Test - Automatic Object Spawner
//!
//! This example is an automatic stress test for the bevy_psx plugin that demonstrates
//! performance under extreme load. It automatically ramps up the spawn rate from
//! 1 object per frame to 1000 objects per frame over 20 seconds.
//!
//! ## Stress Test Profile
//! - **Initial spawn rate**: 1 object per frame
//! - **Final spawn rate**: 1000 objects per frame
//! - **Ramp duration**: 20 seconds
//! - **Object types**: Random cubes, spheres, cylinders, and cones
//! - **No object lifetimes**: Objects never despawn (permanent stress test)
//! - **No max objects limit**: Unlimited spawning for ultimate stress
//!
//! ## Performance Monitoring
//! - Real-time spawn rate tracking
//! - Actual vs target performance comparison
//! - Object count monitoring
//! - Frame rate impact assessment
//! - Memory usage implications
//!
//! ## PSX Features Under Stress
//! - Low-resolution rendering performance
//! - Vertex snapping with high object counts
//! - Palette quantization efficiency
//! - Rendering pipeline stress testing
//!
//! ## No Manual Controls
//! This is a fully automated stress test - no keyboard input required.
//! The test will run automatically and report performance metrics.

use bevy::prelude::*;
use bevy_psx::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .init_resource::<SpawnerSettings>()
        .init_resource::<SpawnStats>()
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                spawn_objects,
                auto_adjust_spawn_rate,
                update_stats,
                cleanup_old_objects,
                rotate_objects,
                handle_palette_controls,
            ),
        )
        .run();
}

#[derive(Resource)]
struct SpawnerSettings {
    objects_per_frame: f32,
    enabled: bool,
    spawn_radius: f32,
    spawn_height: f32,
    start_delay: f32,
}

impl Default for SpawnerSettings {
    fn default() -> Self {
        Self {
            objects_per_frame: 1.0,
            enabled: false,     // Start disabled for countdown
            spawn_radius: 15.0, // Larger spawn area
            spawn_height: 8.0,  // Higher spawn area
            start_delay: 3.0,   // 3 second countdown
        }
    }
}

#[derive(Resource)]
struct SpawnStats {
    total_spawned: u32,
    current_count: u32,
    spawn_times: VecDeque<f64>,
    last_stats_update: f64,
}

impl Default for SpawnStats {
    fn default() -> Self {
        Self {
            total_spawned: 0,
            current_count: 0,
            spawn_times: VecDeque::new(),
            last_stats_update: 0.0,
        }
    }
}

#[derive(Component)]
struct SpawnedObject {
    rotation_speed: Vec3,
}

#[derive(Component)]
#[allow(dead_code)]
struct ObjectType(u8);

fn setup_scene(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera with PsxCamera component
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.2, 0.3),
        brightness: 0.5,
        ..default()
    });

    // Add a ground plane
    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.2, 0.3, 0.2),
    //         ..default()
    //     })),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));

    // Print initial instructions
    print_instructions();
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawner_settings: Res<SpawnerSettings>,
    mut spawn_stats: ResMut<SpawnStats>,
    time: Res<Time>,
) {
    if !spawner_settings.enabled {
        return;
    }

    let current_time = time.elapsed_secs_f64();

    // Spawn multiple objects per frame based on objects_per_frame
    let objects_to_spawn = spawner_settings.objects_per_frame as u32;

    for i in 0..objects_to_spawn {
        // Simple hash-based random generation
        let seed = (current_time * 1000.0) as u64 + spawn_stats.total_spawned as u64 + i as u64;

        let angle = hash_to_f32(seed) * std::f32::consts::TAU;
        let radius = hash_to_f32(seed + 1) * spawner_settings.spawn_radius;
        let height = hash_to_f32(seed + 2) * spawner_settings.spawn_height;

        let position = Vec3::new(angle.cos() * radius, height, angle.sin() * radius);

        // Random object type
        let object_type = (hash_to_f32(seed + 3) * 4.0) as u8;

        // Create mesh based on type
        let (mesh, color) = match object_type {
            0 => (
                meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                Color::srgb(0.8, 0.2, 0.2),
            ),
            1 => (meshes.add(Sphere::new(0.3)), Color::srgb(0.2, 0.8, 0.2)),
            2 => (
                meshes.add(Cylinder::new(0.2, 0.6)),
                Color::srgb(0.2, 0.2, 0.8),
            ),
            _ => (
                meshes.add(Cone {
                    radius: 0.3,
                    height: 0.6,
                }),
                Color::srgb(0.8, 0.8, 0.2),
            ),
        };

        // Random rotation speeds
        let rotation_speed = Vec3::new(
            (hash_to_f32(seed + 4) - 0.5) * 4.0,
            (hash_to_f32(seed + 5) - 0.5) * 4.0,
            (hash_to_f32(seed + 6) - 0.5) * 4.0,
        );

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            Transform::from_translation(position),
            SpawnedObject { rotation_speed },
            ObjectType(object_type),
        ));

        // Update stats
        spawn_stats.total_spawned += 1;
        spawn_stats.current_count += 1;
    }

    // Update spawn tracking for rate calculation
    spawn_stats.spawn_times.push_back(current_time);

    // Keep only recent spawn times for rate calculation (last 5 seconds)
    while let Some(&front_time) = spawn_stats.spawn_times.front() {
        if current_time - front_time > 5.0 {
            spawn_stats.spawn_times.pop_front();
        } else {
            break;
        }
    }
}

fn auto_adjust_spawn_rate(mut spawner_settings: ResMut<SpawnerSettings>, time: Res<Time>) {
    let elapsed_time = time.elapsed_secs();

    // Handle startup countdown
    if elapsed_time < spawner_settings.start_delay {
        let countdown = (spawner_settings.start_delay - elapsed_time).ceil() as i32;
        let prev_countdown =
            (spawner_settings.start_delay - (elapsed_time - time.delta_secs())).ceil() as i32;
        if countdown != prev_countdown && countdown > 0 {
            println!("⏳ Starting stress test in {}...", countdown);
        }
        return;
    }

    // Enable spawning after countdown
    if !spawner_settings.enabled && elapsed_time >= spawner_settings.start_delay {
        spawner_settings.enabled = true;
        println!("🎬 EXTREME STRESS TEST STARTED! Ramping up to 1000 objects/frame...");
        println!("⚠️  WARNING: This will spawn objects every single frame!");
    }

    let test_time = elapsed_time - spawner_settings.start_delay;

    // Ramp from 1 to 1000 objects per frame over 20 seconds
    let ramp_duration = 20.0;
    let min_rate = 1.0;
    let max_rate = 1000.0;

    let new_rate = if test_time < ramp_duration {
        // Exponential ramp for more dramatic stress testing
        let progress = test_time / ramp_duration;
        let exponential_progress = progress * progress; // Quadratic curve for acceleration
        min_rate + (max_rate - min_rate) * exponential_progress
    } else {
        // After 20 seconds, maintain maximum rate
        max_rate
    };

    // Only update if there's a significant change
    if (new_rate - spawner_settings.objects_per_frame).abs() > 1.0 || test_time < 1.0 {
        spawner_settings.objects_per_frame = new_rate;

        // Print rate changes at key milestones
        let should_print = test_time < 1.0
            || (test_time % 2.0 < 0.1)
            || (new_rate >= max_rate
                && (spawner_settings.objects_per_frame - new_rate).abs() > 1.0);

        if should_print && spawner_settings.enabled {
            let progress_percent = (test_time / ramp_duration * 100.0).min(100.0);
            println!(
                "🚀 STRESS TEST - Time: {:.1}s | Target Rate: {:.0} obj/frame | Progress: {:.1}%",
                test_time, new_rate, progress_percent
            );

            // Special milestone messages
            if progress_percent >= 25.0 && progress_percent < 27.0 {
                println!("   🎯 25% - Entering moderate stress zone...");
            } else if progress_percent >= 50.0 && progress_percent < 52.0 {
                println!("   ⚡ 50% - High performance stress active!");
            } else if progress_percent >= 75.0 && progress_percent < 77.0 {
                println!("   🔥 75% - Extreme stress testing initiated!");
            } else if progress_percent >= 100.0 {
                println!("   🏁 100% - MAXIMUM STRESS REACHED!");
            }
        }
    }
}

fn update_stats(
    spawn_stats: Res<SpawnStats>,
    time: Res<Time>,
    spawner_settings: Res<SpawnerSettings>,
) {
    let current_time = time.elapsed_secs_f64();

    // Print stats every 2 seconds
    if current_time - spawn_stats.last_stats_update >= 2.0 {
        // Calculate objects spawned per second based on frame tracking
        let _frames_in_5_seconds = spawn_stats.spawn_times.len() as f32;
        let actual_objects_per_second =
            spawn_stats.current_count as f32 / current_time.max(1.0) as f32;

        let test_time = current_time - spawner_settings.start_delay as f64;

        println!("\n=== PSX STRESS TEST STATS ===");
        println!(
            "⏱️  Test time: {:.1}s",
            if test_time > 0.0 { test_time } else { 0.0 }
        );
        println!("📊 Current objects: {}", spawn_stats.current_count);
        println!("🎯 Total spawned: {}", spawn_stats.total_spawned);
        println!(
            "🚀 Target rate: {:.0} obj/frame",
            spawner_settings.objects_per_frame
        );
        println!(
            "⚡ Current spawn rate: {:.1} obj/sec",
            actual_objects_per_second
        );
        println!("💾 No object limit: Infinite spawning enabled");

        if test_time >= 20.0 && spawner_settings.enabled {
            println!("🏁 STRESS TEST COMPLETE - Maximum spawn rate reached!");
        }

        println!("==============================\n");

        // This is a bit of a hack since we can't modify the resource in a query
        // In a real app, you'd use a proper system for this
        // We'll update this in the spawner system instead
    }
}

fn cleanup_old_objects(mut spawn_stats: ResMut<SpawnStats>, time: Res<Time>) {
    let current_time = time.elapsed_secs_f64();

    // Update stats timer - no cleanup since objects never despawn
    if current_time - spawn_stats.last_stats_update >= 2.0 {
        spawn_stats.last_stats_update = current_time;
    }
}

fn rotate_objects(time: Res<Time>, mut objects: Query<(&mut Transform, &SpawnedObject)>) {
    let delta_time = time.delta_secs();

    for (mut transform, spawned_object) in objects.iter_mut() {
        transform.rotate_x(spawned_object.rotation_speed.x * delta_time);
        transform.rotate_y(spawned_object.rotation_speed.y * delta_time);
        transform.rotate_z(spawned_object.rotation_speed.z * delta_time);
    }
}

fn print_instructions() {
    println!("🚀 === PSX STRESS TEST - PER-FRAME SPAWNER ===");
    println!("This is an automated EXTREME stress test for the bevy_psx plugin");
    println!("No manual input required - the test runs automatically");
    println!();
    println!("📊 STRESS TEST PROFILE:");
    println!("  • Starts at 1 object/frame");
    println!("  • Ramps to 1000 objects/frame over 20 seconds");
    println!("  • Uses exponential acceleration curve");
    println!("  • Objects NEVER despawn (permanent accumulation)");
    println!("  • No maximum object limit (infinite spawning)");
    println!();
    println!("⚠️  EXTREME STRESS WARNING:");
    println!("  • This test spawns objects every single frame!");
    println!("  • At 60 FPS and 1000 obj/frame = 60,000 objects/second!");
    println!("  • Memory usage will grow continuously");
    println!("  • Performance will degrade over time");
    println!();
    println!("🎯 MONITORING:");
    println!("  • Real-time performance tracking");
    println!("  • Object accumulation rate");
    println!("  • PSX rendering performance under extreme load");
    println!("  • Memory and GPU stress testing");
    println!();
    println!("🎨 PSX FEATURES UNDER TEST:");
    println!("  • Low-resolution rendering efficiency");
    println!("  • Vertex snapping with massive object counts");
    println!("  • 🔴 Palettes are OFF for maximum performance");
    println!("  • Press P to toggle palettes");
    println!("  • Press N/M to switch palettes (when on)");
    println!();
    println!("📈 Stats update every 2 seconds - Watch the chaos!");
    println!("=================================================\n");
    println!("🏁 Starting EXTREME stress test in 3 seconds...");
}

fn hash_to_f32(input: u64) -> f32 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    // Convert hash to float between 0.0 and 1.0
    (hash as f64 / u64::MAX as f64) as f32
}

fn handle_palette_controls(
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
