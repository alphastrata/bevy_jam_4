use std::path::Path;

use bevy::prelude::*;

use super::BuildingDefinition;

/// Distributes power to other buildings within a radius.
#[derive(Default)]
pub struct DistributionTower;
impl BuildingDefinition for DistributionTower {
    const SPRITE_PATH: &'static str = "textures/tower.png";
    const BASE_HEALTH: u32 = 100;
    const COST: u32 = 50;
    const NAME: &'static str = "Distribution Tower";
    const DESCRIPTION: &'static str = "";
}
