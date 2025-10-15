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

struct PsxLightBandingExtension {
    bands: u32,
    enabled: u32,
    dither_strength: f32,
    band_smoothness: f32,
}

@group(3) @binding(100)
var<uniform> psx_light_banding_extension: PsxLightBandingExtension;

// Simple dither pattern for band transitions
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
    if (psx_light_banding_extension.enabled == 0u || psx_light_banding_extension.bands == 0u) {
        return color;
    }

    let bands = f32(psx_light_banding_extension.bands);
    let dither = get_dither_value(screen_pos) * psx_light_banding_extension.dither_strength;

    // Calculate luminance for light-based banding
    let luminance = dot(color, vec3<f32>(0.299, 0.587, 0.114));

    // Apply banding to luminance with optional dithering
    let banded_luminance = floor((luminance + dither) * bands) / bands;

    // Preserve color ratios but apply banded luminance
    let color_ratio = select(vec3<f32>(1.0), color / luminance, luminance > 0.001);
    let banded_color = color_ratio * banded_luminance;

    // Optional smoothness to reduce harsh band transitions
    let smoothness = psx_light_banding_extension.band_smoothness;
    if (smoothness > 0.0) {
        return mix(banded_color, color, smoothness);
    }

    return banded_color;
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

    // Apply light banding effect
    out.color = vec4<f32>(
        apply_light_banding(out.color.rgb, in.position.xy),
        out.color.a
    );

    // Apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // Note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
