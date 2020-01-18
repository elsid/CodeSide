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
    is_health_pack_item,
    is_mine_item,
    is_weapon_item,
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

pub fn get_pusher_destination(current_unit: &Unit, world: &World) -> Vec2 {
    if current_unit.weapon.is_none() {
        let weapon_position = world.loot_boxes().iter()
            .filter(|v| is_weapon_item(&v.item))
            .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
            .map(|v| v.position());
        if let Some(v) = weapon_position {
            return v;
        }
    }

    if current_unit.health < world.properties().unit_max_health / 2 {
        let health_pack_position = world.loot_boxes().iter()
            .filter(|v| is_health_pack_item(&v.item))
            .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
            .map(|v| v.position());
        if let Some(v) = health_pack_position {
            return v;
        }
    }

    let opponent_unit_position = world.units().iter()
        .filter(|v| world.is_opponent_unit(v))
        .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
        .map(|v| v.position());

    let mine_position = world.loot_boxes().iter()
        .filter(|v| is_mine_item(&v.item))
        .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
        .map(|v| v.position());

    if let (Some(u), Some(m)) = (opponent_unit_position, mine_position) {
        if current_unit.position().distance(u) < current_unit.position().distance(m) {
            return u;
        }
        return m;
    } else if let Some(v) = opponent_unit_position {
        return v;
    } else if let Some(m) = mine_position {
        return m;
    }

    current_unit.position()
}
