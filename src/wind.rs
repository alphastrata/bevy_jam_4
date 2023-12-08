use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct WindPlugin;
impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WindSimMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WindSimMaterial {
    #[uniform(0)]
    color: Color,
    /// We want to pass in a image that represents a wind vector at every position
    #[texture(1)]
    #[sampler(2)]
    vector_map: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    particle_positions: Option<Handle<Image>>,
    // ..
}
impl Material for WindSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wind_test.wgsl".into()
    }
}

/// Create a start position for particles
fn setup() {}
