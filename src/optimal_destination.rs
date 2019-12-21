use model::{
    Item,
    Unit,
};

use crate::my_strategy::{
    Location,
    Positionable,
    Vec2,
    World,
    as_score,
};

#[inline(never)]
pub fn get_optimal_destination(current_unit: &Unit, optimal_location: &Option<Location>, world: &World) -> Vec2 {
    if let Some(location) = optimal_location {
        Vec2::new(location.x() as f64 + 0.5, location.y() as f64)
    } else {
        let nearest_weapon = world.loot_boxes().iter()
            .filter(|v| {
                if let Item::Weapon { .. } = v.item {
                    true
                } else {
                    false
                }
            })
            .min_by_key(|v| as_score(current_unit.position().distance(v.position())));

        let nearest_health_pack = world.loot_boxes().iter()
            .filter(|v| {
                if let Item::HealthPack { .. } = v.item {
                    true
                } else {
                    false
                }
            })
            .min_by_key(|v| as_score(current_unit.position().distance(v.position())));

        if let (&None, Some(weapon)) = (&current_unit.weapon, nearest_weapon) {
            weapon.position()
        } else if let (true, Some(health_pack)) = (current_unit.health < world.properties().unit_max_health / 2, nearest_health_pack) {
            health_pack.position()
        } else {
            current_unit.position()
        }
    }
}
