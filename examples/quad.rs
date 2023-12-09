//! A shader and a material that uses it.

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use image::{DynamicImage, GenericImage, Rgba};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<WindSimMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, dummy_update)
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
    #[texture(3)]
    #[sampler(4)]
    particle_positions: Option<Handle<Image>>,
}
impl Material2d for WindSimMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/wind_test.wgsl".into()
    }
}

fn dummy_update(query: Query<&MyLittleImage>, images: Res<Assets<Image>>) {
    let i: &MyLittleImage = query.single();

    let image: &Image = images.get(&i.0).unwrap();
    println!("{:?}", &image.data);
}

#[derive(Component)]
struct MyLittleImage(Handle<Image>);

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut wind_mat: ResMut<Assets<WindSimMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
    };

    // Create a texture for the Wind Vectors (replace this with something non uniform colour)
    let _wind_image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[125, 0, 255, 1],
        TextureFormat::Rgba8Unorm,
    );

    let mut wind_image = DynamicImage::new_rgb8(256, 256);
    for x in 0..256 {
        for y in 0..256 {
            wind_image.put_pixel(x, y, Rgba([x as u8, y as u8, 0, 1]));
        }
    }
    let wind_image = Image::from_dynamic(wind_image, false);
    let wind_handle = images.add(wind_image);

    // Create a texture to render the new positions to.
    let mut start_pos_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[255, 0, 255, 255],
        TextureFormat::Bgra8UnormSrgb,
    );
    start_pos_image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let mut wind_image = DynamicImage::new_rgb8(256, 256);
    for x in 0..256 {
        for y in 0..256 {
            wind_image.put_pixel(x, y, Rgba([x as u8, y as u8, 0, 1]));
        }
    }
    start_pos_image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    /*
    Positions
    [  (x1, y1) , (), () ]

    Wind vectors
    [
        ......
        ......
        ......
        ......
    ]
    */

    let particle_handle = images.add(start_pos_image);

    // First pass
    let first_pass_layer = RenderLayers::layer(1);

    let quad = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(512.0, 512.0))));
    // mesh
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(quad),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: wind_mat.add(WindSimMaterial {
                color: Color::rgb(0.1, 0.1, 0.1),
                vector_map: Some(wind_handle),
                particle_positions: Some(particle_handle.clone()),
            }),
            ..default()
        },
        first_pass_layer,
        MyLittleImage(particle_handle.clone()),
    ));
    // camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(particle_handle.clone()),
                // target: RenderTarget::TextureView(()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer,
    ));

    // Second pass
    // mesh
    let cube_size = 4.0;
    let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(particle_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Main pass cube, with material containing the rendered first pass texture.
    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 1.5)
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    });

    // The main pass camera.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
