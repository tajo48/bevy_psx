use std::f32::consts::PI;

use bevy::{
    camera::{Exposure, PhysicalCameraParameters},
    color::palettes::css::*,
    light::CascadeShadowConfigBuilder,
    prelude::*,
};
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s: 1.0 / 125.0,
            sensitivity_iso: 100.0,
            sensor_height: 0.01866,
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_exposure,
                toggle_ambient_light,
                movement,
                animate_light_direction,
                handle_psx_controls,
                handle_palette_controls,
                handle_light_toggles,
            ),
        )
        .run();
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct RedPointLight;

#[derive(Component)]
struct GreenSpotLight;

#[derive(Component)]
struct BluePointLight;

#[derive(Component)]
struct DirectionalSunLight;

/// set up a simple 3D scene with PSX camera
fn setup(
    parameters: Res<Parameters>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
) {
    // Enable palettes for this demo to show how they work with complex lighting
    palette_settings.use_palette = true;

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.15, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: INDIGO.into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform,
    ));
    // back (right) wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.15, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: INDIGO.into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform,
    ));

    // Bevy logo to demonstrate alpha mask shadows
    let mut transform = Transform::from_xyz(-2.2, 0.5, 1.0);
    transform.rotate_y(PI / 8.);
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(2.0, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("branding/bevy_logo_light.png")),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Mask(0.5),
            cull_mode: None,
            ..default()
        })),
        transform,
        Movable,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DEEP_PINK.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Movable,
    ));
    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_xyz(1.5, 1.0, 1.5),
        Movable,
    ));

    // ambient light
    // ambient lights' brightnesses are measured in candela per meter square, calculable as (color * brightness)
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 200.0,
        ..default()
    });

    // red point light
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: RED.into(),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 0.0),
        RedPointLight,
        children![(
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                emissive: LinearRgba::new(4.0, 0.0, 0.0, 0.0),
                ..default()
            })),
        )],
    ));

    // green spot light
    commands.spawn((
        SpotLight {
            intensity: 100_000.0,
            color: LIME.into(),
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_xyz(-1.0, 2.0, 0.0).looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        GreenSpotLight,
        children![(
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.125))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(0.0, 4.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        )],
    ));

    // blue point light
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: BLUE.into(),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 0.0),
        BluePointLight,
        children![(
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: BLUE.into(),
                emissive: LinearRgba::new(0.0, 0.0, 713.0, 0.0),
                ..default()
            })),
        )],
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        DirectionalSunLight,
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

    // example instructions
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        children![
            TextSpan::new("PSX Lights Demo - Complex lighting with PSX rendering\n"),
            TextSpan::new("Ambient light is on\n"),
            TextSpan(format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops,)),
            TextSpan(format!(
                "Shutter speed: 1/{:.0}s\n",
                1.0 / parameters.shutter_speed_s
            )),
            TextSpan(format!(
                "Sensitivity: ISO {:.0}\n",
                parameters.sensitivity_iso
            )),
            TextSpan::new("\n=== Camera Controls ===\n"),
            TextSpan::new("1/2 - Decrease/Increase aperture\n"),
            TextSpan::new("3/4 - Decrease/Increase shutter speed\n"),
            TextSpan::new("5/6 - Decrease/Increase sensitivity\n"),
            TextSpan::new("R - Reset exposure\n"),
            TextSpan::new("Space - Toggle ambient light\n"),
            TextSpan::new("Arrow keys - Move objects\n"),
            TextSpan::new("\n=== Light Controls ===\n"),
            TextSpan::new("Z - Toggle red point light\n"),
            TextSpan::new("X - Toggle green spot light\n"),
            TextSpan::new("C - Toggle blue point light\n"),
            TextSpan::new("G - Toggle directional light\n"),
            TextSpan::new("\n=== PSX Controls ===\n"),
            TextSpan::new("V/B - Vertex snap amount\n"),
            TextSpan::new("T - Toggle vertex snapping\n"),
            TextSpan::new("P - Toggle palettes\n"),
            TextSpan::new("N/M - Switch palettes\n"),
            TextSpan::new("Q/E - Quantization steps\n"),
        ],
    ));

    // PSX camera with physical parameters and exposure
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure::from_physical_camera(**parameters),
        PsxCamera, // This enables PSX rendering!
    ));
}

fn update_exposure(
    key_input: Res<ButtonInput<KeyCode>>,
    mut parameters: ResMut<Parameters>,
    mut exposure: Single<&mut Exposure>,
    text: Single<Entity, With<Text>>,
    mut writer: TextUiWriter,
) {
    // TODO: Clamp values to a reasonable range
    let entity = *text;
    if key_input.just_pressed(KeyCode::Digit2) {
        parameters.aperture_f_stops *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit1) {
        parameters.aperture_f_stops *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit4) {
        parameters.shutter_speed_s *= 2.0;
    } else if key_input.just_pressed(KeyCode::Digit3) {
        parameters.shutter_speed_s *= 0.5;
    }
    if key_input.just_pressed(KeyCode::Digit6) {
        parameters.sensitivity_iso += 100.0;
    } else if key_input.just_pressed(KeyCode::Digit5) {
        parameters.sensitivity_iso -= 100.0;
    }
    if key_input.just_pressed(KeyCode::KeyR) {
        *parameters = Parameters::default();
    }

    *writer.text(entity, 2) = format!("Aperture: f/{:.0}\n", parameters.aperture_f_stops);
    *writer.text(entity, 3) = format!(
        "Shutter speed: 1/{:.0}s\n",
        1.0 / parameters.shutter_speed_s
    );
    *writer.text(entity, 4) = format!("Sensitivity: ISO {:.0}\n", parameters.sensitivity_iso);

    **exposure = Exposure::from_physical_camera(**parameters);
}

fn toggle_ambient_light(
    key_input: Res<ButtonInput<KeyCode>>,
    mut ambient_light: ResMut<AmbientLight>,
    text: Single<Entity, With<Text>>,
    mut writer: TextUiWriter,
) {
    if key_input.just_pressed(KeyCode::Space) {
        if ambient_light.brightness > 1. {
            ambient_light.brightness = 0.;
        } else {
            ambient_light.brightness = 200.;
        }

        let entity = *text;
        let ambient_light_state_text: &str = match ambient_light.brightness {
            0. => "off",
            _ => "on",
        };
        *writer.text(entity, 1) = format!("Ambient light is {ambient_light_state_text}\n");
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_secs() * 2.0 * direction;
    }
}

fn handle_psx_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut vertex_snap_settings: ResMut<PsxVertexSnapSettings>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
) {
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

fn handle_palette_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut palette_settings: ResMut<PsxPaletteSettings>,
    mut palette_manager: ResMut<PaletteManager>,
) {
    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        palette_settings.use_palette = !palette_settings.use_palette;

        if palette_settings.use_palette {
            println!("🎨 PALETTE QUANTIZATION ON - Colors will now be quantized!");
            if let Some(palette) = palette_manager.current_palette() {
                let name = palette.name.as_deref().unwrap_or("Unknown");
                println!("   Active palette: {} ({} colors)", name, palette.len());
            }
            println!("   Use N/M to switch palettes");
        } else {
            println!("🔴 PALETTE QUANTIZATION OFF - Full color range restored");
            println!("   Lighting effects will show with full color depth");
        }
    }

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

fn handle_light_toggles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut red_point_lights: Query<&mut PointLight, With<RedPointLight>>,
    mut green_spot_lights: Query<&mut SpotLight, With<GreenSpotLight>>,
    mut blue_point_lights: Query<&mut PointLight, (With<BluePointLight>, Without<RedPointLight>)>,
    mut directional_lights: Query<&mut DirectionalLight, With<DirectionalSunLight>>,
) {
    // Toggle red point light with Z key
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        for mut light in red_point_lights.iter_mut() {
            if light.intensity > 0.0 {
                light.intensity = 0.0;
                println!("🔴 Red point light: OFF");
            } else {
                light.intensity = 100_000.0;
                println!("🔴 Red point light: ON");
            }
        }
    }

    // Toggle green spot light with X key
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        for mut light in green_spot_lights.iter_mut() {
            if light.intensity > 0.0 {
                light.intensity = 0.0;
                println!("🟢 Green spot light: OFF");
            } else {
                light.intensity = 100_000.0;
                println!("🟢 Green spot light: ON");
            }
        }
    }

    // Toggle blue point light with C key
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        for mut light in blue_point_lights.iter_mut() {
            if light.intensity > 0.0 {
                light.intensity = 0.0;
                println!("🔵 Blue point light: OFF");
            } else {
                light.intensity = 100_000.0;
                println!("🔵 Blue point light: ON");
            }
        }
    }

    // Toggle directional light with G key
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        for mut light in directional_lights.iter_mut() {
            if light.illuminance > 0.0 {
                light.illuminance = 0.0;
                println!("☀️ Directional light: OFF");
            } else {
                light.illuminance = light_consts::lux::OVERCAST_DAY;
                println!("☀️ Directional light: ON");
            }
        }
    }
}
