use bytemuck::{Pod, Zeroable};
use bevy::ecs::query::QueryItem;
use bevy::math::Vec2;
use bevy::prelude::{Component, Deref, Vec3};
use bevy::render::extract_component::ExtractComponent;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct GpuInstanceData {
    pub position: Vec2,
    pub scale: f32,
    pub color: [f32; 4],
}

#[derive(Component, Deref)]
pub struct GpuInstanceSet(pub Vec<GpuInstanceData>);

impl ExtractComponent for GpuInstanceSet {
    type Query = &'static GpuInstanceSet;
    type Filter = ();
    type Out = Self; // can this just be GpuInstanceData?

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        Some(GpuInstanceSet(item.0.clone()))
    }
}
