use model::{
    Item,
    Unit,
    UnitAction,
    Weapon,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use crate::my_strategy::{
    AimTarget,
    Config,
    Debug,
    HitProbabilities,
    Plan,
    Positionable,
    Rectangular,
    Vec2,
    World,
    get_weapon_score,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    ObjectType,
    Target,
    WalkGrid,
    get_nearest_hit,
    normalize_angle,
};

#[inline(never)]
pub fn get_optimal_action(current_unit: &Unit, plan: &Plan, target: &Option<AimTarget>, world: &World,
        debug: &mut Debug) -> UnitAction {
    let (shoot, aim) = if let (Some(target), Some(weapon)) = (target, current_unit.weapon.as_ref()) {
        #[cfg(feature = "enable_debug")]
        render_aim(current_unit, target.position, world.get_unit(target.unit_id), world, debug);
        (
            should_shoot(&target.hit_probabilities, weapon, world.config()),
            target.position - current_unit.center()
        )
    } else {
        (false, Vec2::zero())
    };

    #[cfg(feature = "enable_debug")]
    debug.log(format!("[{}] plan_score={}, transitions: {:?}", current_unit.id, plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>()));

    if plan.transitions.is_empty() {
        return UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            shoot,
            aim: aim.as_model(),
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }

    let mut action = plan.transitions[0].action.clone();
    action.shoot = shoot;
    action.aim = aim.as_model();
    action.swap_weapon = should_swap_weapon(current_unit, shoot, world);
    action.plant_mine = should_plant_mine(current_unit, world);

    action
}

fn should_shoot(hit_probabilities: &HitProbabilities, weapon: &Weapon, config: &Config) -> bool {
    if let (Some(explosion), Some(min_distance)) = (weapon.params.explosion.as_ref(), hit_probabilities.min_distance) {
        if min_distance < explosion.radius + 2.0 {
            return false;
        }
    }

    (hit_probabilities.target + hit_probabilities.opponent_units) >= config.optimal_action_min_opponents_hits_to_shoot
    && hit_probabilities.teammate_units <= config.optimal_action_max_teammates_hits_to_shoot
}

fn should_swap_weapon(current_unit: &Unit, should_shoot: bool, world: &World) -> bool {
    if let Some(weapon) = current_unit.weapon.as_ref() {
        if should_shoot && weapon.magazine > 0 {
            return false;
        }
        match world.tile_item(current_unit.location()) {
            Some(&Item::Weapon { ref weapon_type }) => {
                get_weapon_score(&weapon.typ) < get_weapon_score(weapon_type)
            }
            _ => false,
        }
    } else {
        false
    }
}

fn should_plant_mine(current_unit: &Unit, world: &World) -> bool {
    if !current_unit.on_ground || current_unit.on_ladder || current_unit.mines == 0 {
        return false;
    }
    if world.number_of_teammates() > 0 {
        let will_explode_teammate = world.units().iter()
            .find(|v| world.is_teammate_unit(v) && v.rect().center().distance(current_unit.position()) < 2.0 * world.properties().mine_explosion_params.radius)
            .is_some();
        if will_explode_teammate {
            return false;
        }
    }
    let number_of_exploded_opponents = world.units().iter()
        .filter(|v| world.is_opponent_unit(v) && v.rect().center().distance(current_unit.position()) < world.properties().mine_explosion_params.radius)
        .count();
    number_of_exploded_opponents >= 2
}

#[cfg(feature = "enable_debug")]
fn render_aim(unit: &Unit, destination: Vec2, opponent: &Unit, world: &World, debug: &mut Debug) {
    for position in WalkGrid::new(unit.rect().center(), destination) {
        debug.draw(CustomData::Rect {
            pos: position.as_location().as_model_f32(),
            size: Vec2F32 { x: 1.0, y: 1.0 },
            color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.66 },
        });
    }
    if let Some(weapon) = unit.weapon.as_ref() {
        let source = unit.rect().center();
        let direction = (destination - source).normalized();
        let to_target = direction * world.max_distance();
        let left = direction.left() * weapon.params.bullet.size;
        let right = direction.right() * weapon.params.bullet.size;
        let number_of_directions = world.config().optimal_target_number_of_directions;

        for i in 0 .. number_of_directions {
            let angle = ((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * weapon.spread;
            let destination = source + to_target.rotated(normalize_angle(angle));
            let (src, dst) = if i == 0 {
                (source + right, destination + right)
            } else if i == number_of_directions - 1 {
                (source + left, destination + left)
            } else {
                (source, destination)
            };
            if let Some(hit) = get_nearest_hit(unit.id, src, dst, &Target::from_unit(opponent), &world) {
                let color = match hit.object_type {
                    ObjectType::Wall => ColorF32 { a: 0.5, r: 0.66, g: 0.66, b: 0.66 },
                    ObjectType::Unit => if hit.is_teammate {
                        ColorF32 { a: 0.5, r: 0.66, g: 0.33, b: 0.0 }
                    } else {
                        ColorF32 { a: 0.5, r: 0.0, g: 0.66, b: 0.33 }
                    },
                    ObjectType::Mine => if hit.is_teammate {
                        ColorF32 { a: 0.5, r: 0.33, g: 0.5, b: 0.0 }
                    } else {
                        ColorF32 { a: 0.5, r: 0.5, g: 0.33, b: 0.0 }
                    },
                };
                debug.draw(CustomData::Line {
                    p1: src.as_model_f32(),
                    p2: (src + (dst - src).normalized() * hit.distance).as_model_f32(),
                    width: 0.075,
                    color,
                });
            }
        }
    }
}
