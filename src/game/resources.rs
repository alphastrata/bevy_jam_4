use bevy::prelude::*;

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>();
    }
}

/// What the player currently has in the BANK
#[derive(Resource, Clone, Default)]
pub struct Inventory {
    money: u32,
    plant: u32,
}

pub enum ResourceType {
    CorporationPoints,
    Plant,
    Wood,
}

/// This event should be fired when a resource was harvested
/// (resource, money_earned)
#[derive(Event)]
pub struct Harvest(pub ResourceType, pub u32);

/// System that adds all harvested resources to the players inventory
fn add_harvest_to_inventory(mut inventory: ResMut<Inventory>, mut harvests: EventReader<Harvest>) {
    *inventory = harvests.read().fold(inventory.clone(), |mut inv, harvest| {
        match harvest.0 {
            ResourceType::CorporationPoints => inv.money += harvest.1,
            ResourceType::Plant => inv.plant += harvest.1,
            _ => (),
        };
        inv
    });
}
