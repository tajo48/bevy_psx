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
}

@group(3) @binding(100)
var<uniform> psx_palette_extension: PsxPaletteExtension;

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

    // Apply quantization if enabled
    if (psx_palette_extension.quantize_steps > 0u) {
        // Basic color quantization (reduces color depth)
        let steps = f32(psx_palette_extension.quantize_steps);
        out.color = vec4<f32>(
            floor(out.color.rgb * steps) / steps,
            out.color.a
        );
    }

    // Apply palette quantization if enabled
    if (psx_palette_extension.use_palette > 0u) {
        out.color = vec4<f32>(
            find_closest_palette_color(out.color.rgb),
            out.color.a
        );
    }

    // Apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // Note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
