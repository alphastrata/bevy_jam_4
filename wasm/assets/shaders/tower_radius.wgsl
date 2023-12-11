#import bevy_pbr::forward_io::VertexOutput;

@group(1) @binding(0) var<uniform> colour: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = (in.uv * 2.0) - 1.0;
    var col = colour;
    let mask = sdCircle(uv, 0.91);

    // Calculate the flashing effect using a sine wave and time
    col *= smoothstep(0.0, 0.09, mask);

    return col;
}    

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}
