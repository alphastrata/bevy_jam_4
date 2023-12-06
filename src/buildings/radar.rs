use crate::game::power::SupplyRadius;
use bevy::prelude::*;

use super::BuildingDefinition;

/// Core building that the player starts with
#[derive(Default)]
pub struct RadarTower;
impl BuildingDefinition for RadarTower {
    const SPRITE_PATH: &'static str = "textures/radar.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 100; // Core is free since you start with it
    const BUILD_TIME: u32 = 10;
    const NAME: &'static str = "Radar Tower";
    const DESCRIPTION: &'static str = ""; // TODO

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands.entity(ent_id).insert(RequiresPower);
    }
}
