use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Inventory {
    power: i64,
    wood: i64,
    water: i64
}

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
    }
}
