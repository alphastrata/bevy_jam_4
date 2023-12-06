use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
};

use crate::{game::camera::CameraState, AppState};

pub struct DevScenePlugin;
impl Plugin for DevScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
            .add_systems(
                OnEnter(AppState::MainMenu),
                setup.run_if(in_state(AppState::DevScene)),
            )
            .add_systems(
                OnExit(AppState::MainMenu),
                setup.run_if(in_state(AppState::DevScene)),
            );
    }
}

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

#[derive(Component)]
struct DevSceneState;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut q_camera: Query<&mut Camera, With<CameraState>>,
) {
    let mut camera = q_camera.single_mut();
    camera.is_active = false;

    commands.spawn((Camera3dBundle::default(), DevSceneState));

    let mesh = Mesh::from(shape::Cube { size: 1.0 }) // Sets the custom attribute
        .with_inserted_attribute(
            ATTRIBUTE_BLEND_COLOR,
            // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
            vec![[1.0, 0.0, 0.0, 1.0]; 24],
        );
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(CustomMaterial {
                color: Color::WHITE,
            }),
            ..default()
        },
        DevSceneState,
    ));
}

fn teardown(
    mut commands: Commands,
    mut q_camera: Query<&mut Camera, With<CameraState>>,
    nodes: Query<Entity, With<DevSceneState>>,
) {
    let mut camera = q_camera.single_mut();
    camera.is_active = true;

    for ent in &nodes {
        commands.entity(ent).despawn_recursive();
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_vertex_attribute.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
