struct Transform {
    rotation_scale: mat2x2<f32>,
    translate: vec2<f32>,
};

struct Camera {
    zoom: f32,
    @align(16)
    transform: Transform
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> model: Transform;

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(2)@binding(1)
var s_diffuse: sampler;


fn apply_view_model(position: vec2<f32>) -> vec2<f32> {
    return position * model.rotation_scale * camera.transform.rotation_scale
        - camera.transform.translate + model.translate;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vertex_main(@location(0) position: vec2<f32>,  @location(1) uv: vec2<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(apply_view_model(position), 0.0, camera.zoom);
    output.uv = uv;

    return output;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
}
 