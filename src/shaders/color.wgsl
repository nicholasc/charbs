struct Transform {
    rotation_scale: mat2x2<f32>,
    translate: vec2<f32>,
};

struct Camera {
    zoom: f32,
    @align(16)
    transform: Transform
};

// Global bindings
@group(0) @binding(0)
var<uniform> camera: Camera;

// Mesh bindings
@group(1) @binding(0)
var<uniform> model: Transform;

// Material bindings
@group(2) @binding(0)
var<uniform> color: vec4<f32>;

fn apply_view_model(position: vec2<f32>) -> vec2<f32> {
    return position * model.rotation_scale * camera.transform.rotation_scale
        - camera.transform.translate + model.translate;
}

@vertex
fn vertex_main(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(apply_view_model(position), 0.0, camera.zoom);
}

@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    return vec4<f32>(color);
}
