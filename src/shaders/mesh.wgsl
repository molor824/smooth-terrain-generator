struct VertexOut {
    @builtin(location) clip_position: vec3<f32>,
    @location(0) color: vec4<f32>,
};

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = input.position;
    out.color = input.color;

    return out;
}

@fragment
fn fragment_main(input: VertexOut) -> @location(0) vec4<f32> {
    return input.color;
}
