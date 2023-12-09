//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureFormat},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<WindSimMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
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
}
impl Material2d for WindSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wind_test.wgsl".into()
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WindSimMaterial>>,
    mut images: ResMut<Assets<Image>>,
    _asset_server: Res<AssetServer>,
) {
    let size = Extent3d {
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
    };

    // Create a texture for the Wind Vectors (replace this with something non uniform colour)
    let wind_image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[125, 0, 255, 255],
        TextureFormat::Rgba8Unorm,
    );
    let wind_handle = images.add(wind_image);

    // Create a texture to render the new positions to.

    let quad = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(512.0, 512.0))));

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(quad),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(WindSimMaterial {
            color: Color::BLUE,
            vector_map: Some(wind_handle),
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}
