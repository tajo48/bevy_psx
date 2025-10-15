#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

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

struct PsxPaletteExtension {
    quantize_steps: u32,
    use_palette: u32,
    palette_size: u32,
    palette_colors: array<vec3<f32>, 256>,
    // Light banding properties (for when both effects are enabled)
    bands: u32,
    light_banding_enabled: u32,
    dither_strength: f32,
    band_smoothness: f32,
}

@group(3) @binding(100)
var<uniform> psx_palette_extension: PsxPaletteExtension;

// Simple dither pattern for palette transitions
fn get_dither_value(screen_pos: vec2<f32>) -> f32 {
    let x = u32(screen_pos.x) % 4u;
    let y = u32(screen_pos.y) % 4u;
    let index = y * 4u + x;

    // 4x4 Bayer dither matrix
    let dither_matrix = array<f32, 16>(
        0.0/16.0,  8.0/16.0,  2.0/16.0, 10.0/16.0,
       12.0/16.0,  4.0/16.0, 14.0/16.0,  6.0/16.0,
        3.0/16.0, 11.0/16.0,  1.0/16.0,  9.0/16.0,
       15.0/16.0,  7.0/16.0, 13.0/16.0,  5.0/16.0
    );

    return dither_matrix[index] - 0.5;
}

fn apply_light_banding(color: vec3<f32>, screen_pos: vec2<f32>) -> vec3<f32> {
    if (psx_palette_extension.light_banding_enabled == 0u || psx_palette_extension.bands == 0u) {
        return color;
    }

    let bands = f32(psx_palette_extension.bands);
    let dither = get_dither_value(screen_pos) * psx_palette_extension.dither_strength;

    // Calculate luminance for light-based banding
    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));

    // Apply banding to luminance with optional dithering
    let banded_luminance = floor((luminance + dither) * bands) / bands;

    // Preserve color ratios but apply banded luminance
    let color_ratio = select(vec3<f32>(1.0), color / luminance, luminance > 0.001);
    let banded_color = color_ratio * banded_luminance;

    // Optional smoothness to reduce harsh band transitions
    let smoothness = psx_palette_extension.band_smoothness;
    if (smoothness > 0.0) {
        return mix(banded_color, color, smoothness);
    }

    return banded_color;
}

fn find_closest_palette_color(color: vec3<f32>) -> vec3<f32> {
    if (psx_palette_extension.palette_size == 0u) {
        return color; // No palette available, return original color
    }

    var closest_color = psx_palette_extension.palette_colors[0];
    var min_distance = distance(color, psx_palette_extension.palette_colors[0]);

    for (var i = 1u; i < psx_palette_extension.palette_size; i = i + 1u) {
        let dist = distance(color, psx_palette_extension.palette_colors[i]);
        if (dist < min_distance) {
            min_distance = dist;
            closest_color = psx_palette_extension.palette_colors[i];
        }
    }

    return closest_color;
}

fn find_closest_palette_color_with_dither(color: vec3<f32>, screen_pos: vec2<f32>) -> vec3<f32> {
    if (psx_palette_extension.palette_size == 0u) {
        return color; // No palette available, return original color
    }

    let dither = get_dither_value(screen_pos) * 0.1; // Subtle dithering strength
    let dithered_color = color + vec3<f32>(dither);

    // Find the closest color
    var closest_color = psx_palette_extension.palette_colors[0];
    var min_distance = distance(dithered_color, psx_palette_extension.palette_colors[0]);
    var closest_index = 0u;

    for (var i = 1u; i < psx_palette_extension.palette_size; i = i + 1u) {
        let dist = distance(dithered_color, psx_palette_extension.palette_colors[i]);
        if (dist < min_distance) {
            min_distance = dist;
            closest_color = psx_palette_extension.palette_colors[i];
            closest_index = i;
        }
    }

    // Apply dithering between closest colors for smoother transitions
    if (psx_palette_extension.palette_size > 1u) {
        // Find second closest color for dithering
        var second_closest = psx_palette_extension.palette_colors[0];
        var second_min_distance = 999999.0;

        for (var i = 0u; i < psx_palette_extension.palette_size; i = i + 1u) {
            if (i != closest_index) {
                let dist = distance(color, psx_palette_extension.palette_colors[i]);
                if (dist < second_min_distance) {
                    second_min_distance = dist;
                    second_closest = psx_palette_extension.palette_colors[i];
                }
            }
        }

        // Use dither pattern to blend between closest and second closest
        let dither_threshold = abs(dither) * 2.0;
        let distance_ratio = min_distance / (min_distance + second_min_distance + 0.001);

        if (dither_threshold > distance_ratio) {
            return second_closest;
        }
    }

    return closest_color;
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

    // Apply light banding first (affects lighting)
    out.color = vec4<f32>(
        apply_light_banding(out.color.rgb, in.position.xy),
        out.color.a
    );

    // Apply basic quantization if enabled (reduces color depth)
    if (psx_palette_extension.quantize_steps > 0u) {
        let steps = f32(psx_palette_extension.quantize_steps);
        out.color = vec4<f32>(
            floor(out.color.rgb * steps) / steps,
            out.color.a
        );
    }

    // Apply palette quantization with dithering if enabled (final step)
    if (psx_palette_extension.use_palette > 0u) {
        out.color = vec4<f32>(
            find_closest_palette_color_with_dither(out.color.rgb, in.position.xy),
            out.color.a
        );
    }

    // Apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // Note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
