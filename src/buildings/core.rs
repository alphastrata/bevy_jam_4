use crate::game::power::SupplyRadius;
use bevy::prelude::*;

use super::BuildingDefinition;

/// Core building that the player starts with
#[derive(Default)]
pub struct TheCore;
impl BuildingDefinition for TheCore {
    const SPRITE_PATH: &'static str = "textures/core.png";
    const BASE_HEALTH: u32 = 1000;
    const COST: u32 = 0; // Core is free since you start with it
    const BUILD_TIME: u32 = 0;
    const NAME: &'static str = "Core";
    const DESCRIPTION: &'static str = ""; // TODO

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands.entity(ent_id).insert(SupplyRadius(500.0));
    }
}
