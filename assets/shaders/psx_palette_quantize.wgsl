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
}

@group(2) @binding(100)
var<uniform> psx_palette_extension: PsxPaletteExtension;

// PSX palette colors (64 colors total)
const PALETTE_SIZE: u32 = 64u;
const PALETTE: array<vec3<f32>, 64> = array<vec3<f32>, 64>(
    vec3<f32>(0.180, 0.133, 0.184), // 2e222f
    vec3<f32>(0.243, 0.208, 0.275), // 3e3546
    vec3<f32>(0.384, 0.333, 0.396), // 625565
    vec3<f32>(0.588, 0.424, 0.424), // 966c6c
    vec3<f32>(0.671, 0.580, 0.478), // ab947a
    vec3<f32>(0.412, 0.310, 0.384), // 694f62
    vec3<f32>(0.498, 0.439, 0.541), // 7f708a
    vec3<f32>(0.608, 0.671, 0.698), // 9babb2
    vec3<f32>(0.780, 0.863, 0.816), // c7dcd0
    vec3<f32>(1.000, 1.000, 1.000), // ffffff
    vec3<f32>(0.431, 0.153, 0.153), // 6e2727
    vec3<f32>(0.702, 0.220, 0.192), // b33831
    vec3<f32>(0.918, 0.310, 0.212), // ea4f36
    vec3<f32>(0.961, 0.490, 0.290), // f57d4a
    vec3<f32>(0.682, 0.137, 0.204), // ae2334
    vec3<f32>(0.910, 0.231, 0.231), // e83b3b
    vec3<f32>(0.984, 0.420, 0.114), // fb6b1d
    vec3<f32>(0.969, 0.588, 0.090), // f79617
    vec3<f32>(0.976, 0.761, 0.169), // f9c22b
    vec3<f32>(0.478, 0.188, 0.271), // 7a3045
    vec3<f32>(0.620, 0.271, 0.224), // 9e4539
    vec3<f32>(0.804, 0.408, 0.239), // cd683d
    vec3<f32>(0.902, 0.565, 0.306), // e6904e
    vec3<f32>(0.984, 0.725, 0.329), // fbb954
    vec3<f32>(0.298, 0.239, 0.141), // 4c3e24
    vec3<f32>(0.404, 0.400, 0.200), // 676633
    vec3<f32>(0.635, 0.663, 0.278), // a2a947
    vec3<f32>(0.835, 0.878, 0.294), // d5e04b
    vec3<f32>(0.984, 1.000, 0.525), // fbff86
    vec3<f32>(0.086, 0.353, 0.298), // 165a4c
    vec3<f32>(0.137, 0.565, 0.388), // 239063
    vec3<f32>(0.118, 0.737, 0.451), // 1ebc73
    vec3<f32>(0.569, 0.859, 0.412), // 91db69
    vec3<f32>(0.804, 0.875, 0.424), // cddf6c
    vec3<f32>(0.192, 0.212, 0.220), // 313638
    vec3<f32>(0.216, 0.306, 0.290), // 374e4a
    vec3<f32>(0.329, 0.494, 0.392), // 547e64
    vec3<f32>(0.573, 0.663, 0.518), // 92a984
    vec3<f32>(0.698, 0.729, 0.565), // b2ba90
    vec3<f32>(0.043, 0.369, 0.396), // 0b5e65
    vec3<f32>(0.043, 0.541, 0.561), // 0b8a8f
    vec3<f32>(0.055, 0.686, 0.608), // 0eaf9b
    vec3<f32>(0.188, 0.882, 0.725), // 30e1b9
    vec3<f32>(0.561, 0.973, 0.886), // 8ff8e2
    vec3<f32>(0.196, 0.196, 0.325), // 323353
    vec3<f32>(0.282, 0.290, 0.467), // 484a77
    vec3<f32>(0.302, 0.396, 0.706), // 4d65b4
    vec3<f32>(0.302, 0.608, 0.902), // 4d9be6
    vec3<f32>(0.561, 0.827, 1.000), // 8fd3ff
    vec3<f32>(0.271, 0.161, 0.247), // 45293f
    vec3<f32>(0.420, 0.243, 0.459), // 6b3e75
    vec3<f32>(0.565, 0.369, 0.663), // 905ea9
    vec3<f32>(0.659, 0.518, 0.953), // a884f3
    vec3<f32>(0.918, 0.678, 0.929), // eaaded
    vec3<f32>(0.459, 0.235, 0.329), // 753c54
    vec3<f32>(0.635, 0.294, 0.435), // a24b6f
    vec3<f32>(0.812, 0.396, 0.498), // cf657f
    vec3<f32>(0.929, 0.502, 0.600), // ed8099
    vec3<f32>(0.514, 0.110, 0.365), // 831c5d
    vec3<f32>(0.765, 0.141, 0.329), // c32454
    vec3<f32>(0.941, 0.310, 0.471), // f04f78
    vec3<f32>(0.969, 0.506, 0.506), // f68181
    vec3<f32>(0.988, 0.655, 0.565), // fca790
    vec3<f32>(0.992, 0.796, 0.690), // fdcbb0
);

fn find_closest_palette_color(color: vec3<f32>) -> vec3<f32> {
    var closest_color = PALETTE[0];
    var min_distance = distance(color, PALETTE[0]);

    for (var i = 1u; i < PALETTE_SIZE; i = i + 1u) {
        let dist = distance(color, PALETTE[i]);
        if (dist < min_distance) {
            min_distance = dist;
            closest_color = PALETTE[i];
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
