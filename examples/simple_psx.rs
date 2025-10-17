use bevy::prelude::*;
use bevy_psx::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PsxCameraPlugin)
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(
            Update,
            (
                rotate_cube,
                move_sphere,
                handle_keyboard_input,
                update_ui_text,
                handle_palette_switching,
            ),
        )
        .run();
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
struct MovingSphere;

#[derive(Component)]
struct SettingsText;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut psx_settings: ResMut<PsxSettings>,
) {
    // Enable PSX effects by default for the demo
    psx_settings.use_palette = true;
    psx_settings.dither_enabled = true;
    psx_settings.quantize_enabled = true;
    psx_settings.snap_enabled = true;

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
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));

    // Add a moving sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
        Transform::from_xyz(2.0, 1.0, 0.0),
        MovingSphere,
    ));

    // Add a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8.0, 8.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add some additional objects for visual interest
    for i in 0..3 {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / 3.0;
        let x = angle.cos() * 3.0;
        let z = angle.sin() * 3.0;

        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 1.5))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.8))),
            Transform::from_xyz(x, 0.75, z),
        ));
    }
}

fn setup_ui(mut commands: Commands) {
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
                Text::new("PSX Simple Demo"),
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
                    font_size: 20.0,
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
                     1/2/3/4: Change resolution\n\
                     V: Toggle vertex snapping\n\
                     F: Toggle filtering\n\
                     P: Toggle palette quantization\n\
                     D: Toggle dithering\n\
                     Q: Toggle basic quantization\n\
                     E/R: Adjust quantization steps\n\
                     T/Y: Adjust dither strength\n\
                     G/H: Switch dither patterns\n\
                     B/N: Adjust vertex snap amount\n\
                     Shift+N/M: Switch palettes",
                ),
                TextFont {
                    font_size: 18.0,
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

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.5);
        transform.rotate_x(time.delta_secs() * 0.3);
    }
}

fn move_sphere(time: Res<Time>, mut query: Query<&mut Transform, With<MovingSphere>>) {
    for mut transform in query.iter_mut() {
        let time_secs = time.elapsed_secs();
        transform.translation.x = (time_secs * 2.0).sin() * 2.0;
        transform.translation.z = (time_secs * 1.5).cos() * 1.5;
    }
}

fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut psx_settings: ResMut<PsxRenderSettings>,
    mut psx_material_settings: ResMut<PsxSettings>,
    windows: Query<&Window>,
) {
    // Change resolution with number keys
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        psx_settings.base_resolution = UVec2::new(320, 240);
        psx_settings.render_resolution = UVec2::new(320, 240);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        psx_settings.base_resolution = UVec2::new(512, 448);
        psx_settings.render_resolution = UVec2::new(512, 448);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        psx_settings.base_resolution = UVec2::new(800, 600);
        psx_settings.render_resolution = UVec2::new(800, 600);
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        // Adaptive resolution based on window size
        if let Ok(window) = windows.single() {
            let size = UVec2::new(window.width() as u32 / 2, window.height() as u32 / 2);
            psx_settings.base_resolution = size;
            psx_settings.render_resolution = size;
        }
    }

    // Toggle aspect ratio matching
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        psx_settings.aspect_ratio_matching = !psx_settings.aspect_ratio_matching;
    }

    // Toggle filtering
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        psx_settings.pixelated = !psx_settings.pixelated;
    }

    // Toggle vertex snapping
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        psx_material_settings.snap_enabled = !psx_material_settings.snap_enabled;
    }

    // Adjust vertex snap amount with B/N keys
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        if psx_material_settings.snap_amount > 8.0 {
            psx_material_settings.snap_amount -= 8.0;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyN) && !keyboard_input.pressed(KeyCode::ShiftLeft) {
        if psx_material_settings.snap_amount < 256.0 {
            psx_material_settings.snap_amount += 8.0;
        }
    }

    // Toggle palette quantization with P key
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        psx_material_settings.use_palette = !psx_material_settings.use_palette;
    }

    // Toggle dithering with D key
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        psx_material_settings.dither_enabled = !psx_material_settings.dither_enabled;
    }

    // Toggle basic quantization with Q key
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        psx_material_settings.quantize_enabled = !psx_material_settings.quantize_enabled;
    }

    // Adjust quantization steps with E/R keys
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if psx_material_settings.quantize_steps > 8 {
            psx_material_settings.quantize_steps -= 8;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if psx_material_settings.quantize_steps < 128 {
            psx_material_settings.quantize_steps += 8;
        }
    }

    // Adjust dither strength with T/Y keys
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        if psx_material_settings.dither_strength > 0.1 {
            psx_material_settings.dither_strength -= 0.1;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyY) {
        if psx_material_settings.dither_strength < 1.0 {
            psx_material_settings.dither_strength += 0.1;
        }
    }

    // Switch dither patterns with G/H keys
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        psx_material_settings.dither_pattern = match psx_material_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Random,
            DitherPattern::Bayer8x8 => DitherPattern::Bayer4x4,
            DitherPattern::BlueNoise => DitherPattern::Bayer8x8,
            DitherPattern::Random => DitherPattern::BlueNoise,
        };
    }

    if keyboard_input.just_pressed(KeyCode::KeyH) {
        psx_material_settings.dither_pattern = match psx_material_settings.dither_pattern {
            DitherPattern::Bayer4x4 => DitherPattern::Bayer8x8,
            DitherPattern::Bayer8x8 => DitherPattern::BlueNoise,
            DitherPattern::BlueNoise => DitherPattern::Random,
            DitherPattern::Random => DitherPattern::Bayer4x4,
        };
    }
}

fn update_ui_text(
    mut text_query: Query<&mut Text, With<SettingsText>>,
    psx_settings: Res<PsxRenderSettings>,
    psx_material_settings: Res<PsxSettings>,
    palette_manager: Res<PaletteManager>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        let current_palette = palette_manager
            .current_palette()
            .map(|p| p.name.as_deref().unwrap_or("Unknown"))
            .unwrap_or("None");

        **text = format!(
            "RENDER SETTINGS:\n\
             Resolution: {}x{}\n\
             Aspect Ratio Matching: {}\n\
             Filtering: {}\n\n\
             VERTEX SETTINGS:\n\
             Vertex Snapping: {} (Amount: {:.0})\n\n\
             PSX MATERIAL SETTINGS:\n\
             Basic Quantization: {} (Steps: {})\n\
             Palette Quantization: {}\n\
             Current Palette: {} ({} colors)\n\
             Dithering: {} (Strength: {:.1})\n\
             Dither Pattern: {:?}",
            psx_settings.render_resolution.x,
            psx_settings.render_resolution.y,
            if psx_settings.aspect_ratio_matching {
                "ON"
            } else {
                "OFF"
            },
            if psx_settings.pixelated {
                "Pixelated"
            } else {
                "Smooth"
            },
            if psx_material_settings.snap_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_material_settings.snap_amount,
            if psx_material_settings.quantize_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_material_settings.quantize_steps,
            if psx_material_settings.use_palette {
                "ON"
            } else {
                "OFF"
            },
            current_palette,
            palette_manager
                .current_palette()
                .map(|p| p.len())
                .unwrap_or(0),
            if psx_material_settings.dither_enabled {
                "ON"
            } else {
                "OFF"
            },
            psx_material_settings.dither_strength,
            psx_material_settings.dither_pattern,
        );
    }
}

fn handle_palette_switching(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut palette_manager: ResMut<PaletteManager>,
) {
    // Switch to next palette with N key (with Shift modifier to avoid conflict with vertex snap)
    if keyboard_input.just_pressed(KeyCode::KeyN) && keyboard_input.pressed(KeyCode::ShiftLeft) {
        palette_manager.next_palette();
    }

    // Switch to previous palette with M key
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        palette_manager.prev_palette();
    }
}
