#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#import bevy_render::instance_index::get_instance_index

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct PsxMaterialExtension {
    // Vertex snapping uniforms
    snap_amount: f32,
    snap_enabled: u32,

    // Fragment shader uniforms
    quantize_steps: u32,
    quantize_enabled: u32,
    use_palette: u32,
    palette_size: u32,
    palette_colors: array<vec3<f32>, 256>,
    dither_enabled: u32,
    dither_strength: f32,
    dither_pattern: u32,
    color_space: u32,
    error_diffusion: u32,
    blend_mode: u32,
    blend_factor: f32,
}

@group(3) @binding(100)
var<uniform> psx_material: PsxMaterialExtension;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    #ifdef VERTEX_UVS
        @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_UVS_B
        @location(3) uv_b: vec2<f32>,
    #endif
    #ifdef VERTEX_TANGENTS
        @location(4) tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
        @location(5) color: vec4<f32>,
    #endif
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get world position using correct Bevy function
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));

    // Transform to clip space using correct view matrix access
    let clip_position = view.clip_from_world * world_position;

    // Apply PSX-style vertex snapping if enabled
    if (psx_material.snap_enabled != 0u) {
        let snap_scale = psx_material.snap_amount;

        // Perform perspective division for snapping in screen space
        let w = clip_position.w;
        let ndc_position = clip_position.xyz / w;

        // Apply snapping to x and y coordinates
        let snapped_ndc = vec3<f32>(
            floor(ndc_position.x * snap_scale) / snap_scale,
            floor(ndc_position.y * snap_scale) / snap_scale,
            ndc_position.z
        );

        // Convert back to clip space
        out.position = vec4<f32>(snapped_ndc * w, w);
    } else {
        out.position = clip_position;
    }

    // Set up other vertex outputs
    out.world_position = world_position;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.instance_index = vertex.instance_index;

    #ifdef VERTEX_UVS
        out.uv = vertex.uv;
    #endif

    #ifdef VERTEX_UVS_B
        out.uv_b = vertex.uv_b;
    #endif

    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_functions::mesh_tangent_local_to_world(world_from_local, vertex.tangent);
    #endif

    #ifdef VERTEX_COLORS
        out.color = vertex.color;
    #endif

    return out;
}

// 4x4 Bayer dither matrix
fn get_bayer_4x4(screen_pos: vec2<f32>) -> f32 {
    let x = u32(screen_pos.x) % 4u;
    let y = u32(screen_pos.y) % 4u;
    let index = y * 4u + x;

    let dither_matrix = array<f32, 16>(
        0.0/16.0,  8.0/16.0,  2.0/16.0, 10.0/16.0,
       12.0/16.0,  4.0/16.0, 14.0/16.0,  6.0/16.0,
        3.0/16.0, 11.0/16.0,  1.0/16.0,  9.0/16.0,
       15.0/16.0,  7.0/16.0, 13.0/16.0,  5.0/16.0
    );

    return dither_matrix[index] - 0.5;
}

// 8x8 Bayer dither matrix
fn get_bayer_8x8(screen_pos: vec2<f32>) -> f32 {
    let x = u32(screen_pos.x) % 8u;
    let y = u32(screen_pos.y) % 8u;
    let index = y * 8u + x;

    let dither_matrix = array<f32, 64>(
         0.0/64.0, 32.0/64.0,  8.0/64.0, 40.0/64.0,  2.0/64.0, 34.0/64.0, 10.0/64.0, 42.0/64.0,
        48.0/64.0, 16.0/64.0, 56.0/64.0, 24.0/64.0, 50.0/64.0, 18.0/64.0, 58.0/64.0, 26.0/64.0,
        12.0/64.0, 44.0/64.0,  4.0/64.0, 36.0/64.0, 14.0/64.0, 46.0/64.0,  6.0/64.0, 38.0/64.0,
        60.0/64.0, 28.0/64.0, 52.0/64.0, 20.0/64.0, 62.0/64.0, 30.0/64.0, 54.0/64.0, 22.0/64.0,
         3.0/64.0, 35.0/64.0, 11.0/64.0, 43.0/64.0,  1.0/64.0, 33.0/64.0,  9.0/64.0, 41.0/64.0,
        51.0/64.0, 19.0/64.0, 59.0/64.0, 27.0/64.0, 49.0/64.0, 17.0/64.0, 57.0/64.0, 25.0/64.0,
        15.0/64.0, 47.0/64.0,  7.0/64.0, 39.0/64.0, 13.0/64.0, 45.0/64.0,  5.0/64.0, 37.0/64.0,
        63.0/64.0, 31.0/64.0, 55.0/64.0, 23.0/64.0, 61.0/64.0, 29.0/64.0, 53.0/64.0, 21.0/64.0
    );

    return dither_matrix[index] - 0.5;
}

// Simple blue noise approximation
fn get_blue_noise(screen_pos: vec2<f32>) -> f32 {
    let p = fract(screen_pos * 0.1031);
    let p3 = vec3<f32>(p, p.x);
    let p3_dot = dot(p3, vec3<f32>(0.1031, 0.1030, 0.0973));
    let p3_fract = fract(p3 + vec3<f32>(p3_dot));
    let dot_result = dot(p3_fract.xy, vec2<f32>(0.06711, 0.00583));
    return fract(dot_result) - 0.5;
}

// Pseudo-random dither
fn get_random_dither(screen_pos: vec2<f32>) -> f32 {
    let seed = dot(screen_pos, vec2<f32>(12.9898, 78.233));
    return fract(sin(seed) * 43758.5453) - 0.5;
}

// Get dither value based on selected pattern
fn get_dither_value(screen_pos: vec2<f32>) -> f32 {
    switch psx_material.dither_pattern {
        case 1u: { return get_bayer_8x8(screen_pos); }
        case 2u: { return get_blue_noise(screen_pos); }
        case 3u: { return get_random_dither(screen_pos); }
        default: { return get_bayer_4x4(screen_pos); }
    }
}

// RGB to HSV conversion
fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
    let max_val = max(max(rgb.r, rgb.g), rgb.b);
    let min_val = min(min(rgb.r, rgb.g), rgb.b);
    let delta = max_val - min_val;

    var h = 0.0;
    let s = select(0.0, delta / max_val, max_val > 0.0);
    let v = max_val;

    if (delta > 0.0) {
        if (max_val == rgb.r) {
            h = ((rgb.g - rgb.b) / delta) / 6.0;
        } else if (max_val == rgb.g) {
            h = (2.0 + (rgb.b - rgb.r) / delta) / 6.0;
        } else {
            h = (4.0 + (rgb.r - rgb.g) / delta) / 6.0;
        }
        h = fract(h);
    }

    return vec3<f32>(h, s, v);
}

// Simple RGB to LAB approximation (not perceptually accurate but good enough for games)
fn rgb_to_lab(rgb: vec3<f32>) -> vec3<f32> {
    // Simple approximation - in real LAB this would involve XYZ conversion
    let l = dot(rgb, vec3<f32>(0.299, 0.587, 0.114));
    let a = (rgb.r - rgb.g) * 0.5;
    let b = (rgb.r + rgb.g - 2.0 * rgb.b) * 0.25;
    return vec3<f32>(l, a + 0.5, b + 0.5);
}

// Color distance calculation in different color spaces
fn color_distance(c1: vec3<f32>, c2: vec3<f32>) -> f32 {
    switch psx_material.color_space {
        case 1u: {
            // HSV space - emphasize hue differences
            let hsv1 = rgb_to_hsv(c1);
            let hsv2 = rgb_to_hsv(c2);
            let h_diff = min(abs(hsv1.x - hsv2.x), 1.0 - abs(hsv1.x - hsv2.x));
            return sqrt(h_diff * h_diff * 4.0 + (hsv1.y - hsv2.y) * (hsv1.y - hsv2.y) + (hsv1.z - hsv2.z) * (hsv1.z - hsv2.z));
        }
        case 2u: {
            // LAB space approximation
            let lab1 = rgb_to_lab(c1);
            let lab2 = rgb_to_lab(c2);
            return distance(lab1, lab2);
        }
        default: {
            // RGB space
            return distance(c1, c2);
        }
    }
}

// Apply dithering to any color
fn apply_dithering(color: vec3<f32>, screen_pos: vec2<f32>) -> vec3<f32> {
    if (psx_material.dither_enabled == 0u) {
        return color;
    }

    let dither = get_dither_value(screen_pos) * psx_material.dither_strength;
    return clamp(color + vec3<f32>(dither), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Find closest palette color
fn find_closest_palette_color(color: vec3<f32>, screen_pos: vec2<f32>) -> vec3<f32> {
    if (psx_material.palette_size == 0u) {
        return color;
    }

    var target_color = color;

    // Apply dithering to input color first for more visible effect
    if (psx_material.dither_enabled > 0u) {
        let dither = get_dither_value(screen_pos) * psx_material.dither_strength;
        target_color = clamp(color + vec3<f32>(dither), vec3<f32>(0.0), vec3<f32>(1.0));
    }

    // Find closest palette color using dithered input
    var closest_color = psx_material.palette_colors[0];
    var min_distance = color_distance(target_color, psx_material.palette_colors[0]);
    var closest_index = 0u;

    for (var i = 1u; i < psx_material.palette_size; i = i + 1u) {
        let dist = color_distance(target_color, psx_material.palette_colors[i]);
        if (dist < min_distance) {
            min_distance = dist;
            closest_color = psx_material.palette_colors[i];
            closest_index = i;
        }
    }

    // Additional dithering between palette colors for smoother transitions
    if (psx_material.dither_enabled > 0u && psx_material.palette_size > 1u) {
        // Find second closest color
        var second_closest = psx_material.palette_colors[0];
        var second_min_distance = 999999.0;

        for (var i = 0u; i < psx_material.palette_size; i = i + 1u) {
            if (i != closest_index) {
                let dist = color_distance(color, psx_material.palette_colors[i]);
                if (dist < second_min_distance) {
                    second_min_distance = dist;
                    second_closest = psx_material.palette_colors[i];
                }
            }
        }

        // Use dither pattern to choose between closest colors more aggressively
        let raw_dither = get_dither_value(screen_pos);
        let dither_threshold = raw_dither * psx_material.dither_strength * 2.0; // Double strength for palette dithering
        let distance_ratio = min_distance / (min_distance + second_min_distance + 0.001);

        // More aggressive dithering threshold
        if (abs(dither_threshold) > distance_ratio * 0.3) {
            closest_color = second_closest;
        }
    }

    return closest_color;
}

// Apply basic quantization
fn apply_basic_quantization(color: vec3<f32>) -> vec3<f32> {
    if (psx_material.quantize_steps == 0u) {
        return color;
    }

    let steps = f32(psx_material.quantize_steps);
    return floor(color * steps) / steps;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // In deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    let original_color = out.color.rgb;
    var final_color = original_color;

    // Step 1: Apply dithering if enabled (works independently)
    if (psx_material.dither_enabled > 0u && psx_material.use_palette == 0u) {
        final_color = apply_dithering(final_color, in.position.xy);
    }

    // Step 2: Apply basic quantization if enabled
    if (psx_material.quantize_enabled > 0u) {
        final_color = apply_basic_quantization(final_color);
    }

    // Step 3: Apply palette quantization if enabled (includes dithering)
    if (psx_material.use_palette > 0u) {
        final_color = find_closest_palette_color(final_color, in.position.xy);
    }

    // Step 4: Apply blending if enabled
    if (psx_material.blend_mode > 0u) {
        final_color = mix(original_color, final_color, psx_material.blend_factor);
    }

    out.color = vec4<f32>(final_color, out.color.a);

    // Apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // Note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
