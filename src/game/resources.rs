use bevy::prelude::*;

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
    }
}

/// What the player currently has in the BANK
#[derive(Resource, Default)]
pub struct Inventory {
    money: u32,
}

enum ResourceType {
    Plant,
    Wood,
}
