#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

// Uniforms
struct WindSimMaterial {
  color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: WindSimMaterial;
@group(1) @binding(1) var vector_map_texture: texture_2d<f32>;
@group(1) @binding(2) var vector_map_sampler: sampler;
@group(1) @binding(3) var particle_texture: texture_2d<f32>;
@group(1) @binding(4) var particle_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
  // Each pixel of the output render texture will correlate to the next position of the
  // particles.

  let pos = textureSample(
    particle_texture,
    particle_sampler,
    mesh.uv
  );

  let posv2 = vec2(pos.x, pos.y);
  let wind = textureSample(
    vector_map_texture,
    vector_map_sampler,
    posv2 
  );
  let added_wind = normalize(wind) + 0.5;

  let new_pos = pos + sin(globals.time) * added_wind * 0.05;
  //let new_pos = cross(pos.xyz, wind.xyz);
  return new_pos;

  //return vec4(vec3(smoothstep(0.1, 0.8, new_pos)), 1.0);
}
