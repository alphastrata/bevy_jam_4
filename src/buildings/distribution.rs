use std::path::Path;

use bevy::prelude::*;

use crate::game::power::SupplyRadius;

use super::BuildingDefinition;

/// Distributes power to other buildings within a radius.
#[derive(Default)]
pub struct DistributionTower;
impl BuildingDefinition for DistributionTower {
    const SPRITE_PATH: &'static str = "textures/tower.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 50;
    // build time
    const NAME: &'static str = "Distribution Tower";
    const DESCRIPTION: &'static str = "";

    fn add_extra_components(commands: &mut Commands, ent_id: Entity) {
        commands.entity(ent_id).insert(SupplyRadius(300.0));
    }
}
