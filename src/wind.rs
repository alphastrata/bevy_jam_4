use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureFormat},
    sprite::MaterialMesh2dBundle,
};

pub struct WindPlugin;
impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WindSimMaterial>::default())
            .add_systems(Startup, setup);
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
    // #[texture(3)]
    // #[sampler(4)]
    // particle_positions: Option<Handle<Image>>,
    // ..
}
impl Material for WindSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wind_test.wgsl".into()
    }
}

/// Create a start position for particles
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WindSimMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // particle starting positions

    // wind map
    let wind_image = Image::new_fill(
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[127, 127],
        TextureFormat::Rg8Snorm,
    );
    // wind_image.
    let wind_handle = images.add(wind_image);

    let quad = meshes.add(shape::Quad::new(Vec2::new(512.0, 512.0)).into());
    commands.spawn(MaterialMeshBundle {
        mesh: quad,
        material: materials.add(WindSimMaterial {
            color: Color::PURPLE,
            vector_map: Some(wind_handle),
            // particle_positions: todo!(),
        }),
        transform: Transform::from_translation(Vec3::new(5.0, 5.0, 0.3)),
        ..default()
    });
}

/// creates a basic wind map texture that just has different directions for different quadrants
fn basic_wind_map() {}

/// TODO: create a more realistic wind currents texture
fn cooler_map() {}
