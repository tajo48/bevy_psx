#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings::view,
}

#import bevy_render::instance_index::get_instance_index

struct PsxVertexSnapExtension {
    snap_amount: f32,
}

@group(2) @binding(100)
var<uniform> extension: PsxVertexSnapExtension;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    #ifdef VERTEX_UVS
        @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_TANGENTS
        @location(3) tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
        @location(4) color: vec4<f32>,
    #endif
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    #ifdef VERTEX_UVS
        @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_UVS_B
        @location(3) uv_b: vec2<f32>,
    #endif
    #ifdef VERTEX_TANGENTS
        @location(4) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
        @location(5) color: vec4<f32>,
    #endif
    @location(6) @interpolate(flat) instance_index: u32,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Get world position using correct Bevy function
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));

    // Transform to clip space using correct view matrix access
    let clip_position = view.clip_from_world * world_position;

    // Apply PSX-style vertex snapping
    let snap_scale = extension.snap_amount;

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
    out.clip_position = vec4<f32>(snapped_ndc * w, w);

    // Set up other vertex outputs
    out.world_position = world_position;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.instance_index = vertex.instance_index;

    #ifdef VERTEX_UVS
        out.uv = vertex.uv;
    #endif

    #ifdef VERTEX_UVS_B
        out.uv_b = vertex.uv;
    #endif

    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_functions::mesh_tangent_local_to_world(world_from_local, vertex.tangent);
    #endif

    #ifdef VERTEX_COLORS
        out.color = vertex.color;
    #endif

    return out;
}
