/// ** ///
/// THIS IS THE DEFAULT 2D SHADER ///
/// You can always get back to this with `python3 scripts/reset-2d.py` ///
/// ***************************** ///
#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_render::view::View
#import bevy_pbr::forward_io::VertexOutput;

@group(0) @binding(0) var<uniform> view: View;

const SPEED:f32 = 1.0;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = (in.uv * 2.0) - 1.0;
    var col = vec4f(1.0);

    let feet_mask = sdCircle(uv, 0.25); // Get a mask for the area around our feet.
    var out = vec4f(col, m);

    out *= smoothstep(0.0, 0.09, feet_mask);
    return out;
   
}    
    
fn sdCircle(p: vec2f, r: f32) -> f32 {
    return length(p) - r;
}
