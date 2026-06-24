@group(0) @binding(0)
var<uniform> u_view_proj: mat4x4<f32>;
@group(0) @binding(1)
var<uniform> u_model: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let world_pos = u_model * vec4<f32>(input.position, 1.0);
    output.position = u_view_proj * world_pos;
    output.normal = (u_model * vec4<f32>(input.normal, 0.0)).xyz;
    output.uv = input.uv;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 1.0));
    let diffuse = max(dot(input.normal, light_dir), 0.0);
    return vec4<f32>(0.5 + diffuse * 0.5, 0.5, 0.8, 1.0);
}
