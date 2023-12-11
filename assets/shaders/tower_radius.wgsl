#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_pbr::forward_io::VertexOutput;

@group(1) @binding(0) var<uniform> colour: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = (in.uv * 2.0) - 1.0;
    var col = colour;
    let time = globals.time;
    let mask = sdCircle(uv, 0.91);
    let circleRadius = 0.84;
    let distanceFromCenter = sdCircle(uv, circleRadius);

    var baseColor = colour;
    baseColor.a = 0.01;

    let additionalOpacity = clamp(1.0 - (distanceFromCenter) / circleRadius, 0.0, 0.7);

    baseColor.a += additionalOpacity * (0.72);

    return baseColor;
}    

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}
