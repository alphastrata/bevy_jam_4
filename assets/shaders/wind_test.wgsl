#import bevy_pbr::forward_io::VertexOutput

// Uniforms
struct WindSimMaterial {
  color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: WindSimMaterial;
@group(1) @binding(1) var vector_map_texture: texture_2d<f32>;
@group(1) @binding(2) var vector_map_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  return material.color *
}
