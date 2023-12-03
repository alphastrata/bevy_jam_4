use bevy::asset::{ErasedAssetLoader, Handle};
use bevy::pbr::{MeshPipeline, MeshPipelineKey};
use bevy::prelude::{AssetServer, FromWorld, Resource, Shader, World};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipeline, SpecializedMeshPipelineError, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use crate::gpu_entities::instancing::GpuInstanceData;

#[derive(Resource)]
pub struct GpuInstancePipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for GpuInstancePipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let shader = asset_server.load("shaders/instancing.wgsl");

        let mesh_pipeline = world.resource::<MeshPipeline>();

        GpuInstancePipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedMeshPipeline for GpuInstancePipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        // meshes typically live in bind group 2. because we are using bindgroup 1
        // we need to add MESH_BINDGROUP_1 shader def so that the bindings are correctly
        // linked in the shader
        descriptor
            .vertex
            .shader_defs
            .push("MESH_BINDGROUP_1".into());

        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuInstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        let desc_opt = descriptor.fragment.as_mut();

        desc_opt.unwrap().shader = self.shader.clone();
        Ok(descriptor)
    }
}