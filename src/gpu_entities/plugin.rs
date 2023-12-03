use crate::gpu_entities::{prepare_instance_buffers, queue_custom, DrawCustom, GpuInstancePipeline, setup_instances};
use bevy::app::{App, Plugin};
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::prelude::{IntoSystemConfigs, OnEnter};
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedMeshPipelines;
use bevy::render::Render;
use bevy::render::{RenderApp, RenderSet};
use bevy::render::extract_component::ExtractComponentPlugin;
use crate::AppState;
use crate::gpu_entities::instancing::GpuInstanceSet;

pub struct GpuInstancingPlugin;

impl Plugin for GpuInstancingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<GpuInstanceSet>::default());
        app.add_systems(OnEnter(AppState::Playing), setup_instances);
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<GpuInstancePipeline>>()
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<GpuInstancePipeline>();
    }
}
