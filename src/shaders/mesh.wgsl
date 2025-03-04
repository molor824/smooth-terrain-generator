struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
};

struct VertexIn {
    @location(0) position: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = vec4(input.position, 1.0);

    return out;
}

@group(0) @binding(0)
var<uniform> color: vec3<f32>;

@fragment
fn fragment_main(input: VertexOut) -> @location(0) vec4<f32> {
    return vec4(color, 1.0);
}
