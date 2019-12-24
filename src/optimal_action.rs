use model::{
    Item,
    Tile,
    Unit,
    UnitAction,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use crate::my_strategy::{
    Debug,
    Location,
    Planner,
    Positionable,
    Rectangular,
    Simulator,
    Vec2,
    World,
    XorShiftRng,
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
pub fn get_optimal_action(current_unit: &Unit, global_destination: Vec2, target: Option<i32>, world: &World,
        rng: &mut XorShiftRng, debug: &mut Debug) -> UnitAction {
    let nearest_opponent = target.map(|unit_id| world.get_unit(unit_id));

    let (shoot, aim) = if let Some(opponent) = nearest_opponent {
        #[cfg(feature = "enable_debug")]
        render_aim(current_unit, opponent, world, debug);
        (true, opponent.position() - current_unit.position())
    } else {
        (false, Vec2::zero())
    };

    let tiles_path = world.find_shortcut_tiles_path(current_unit.id, current_unit.location(), global_destination.as_location());

    #[cfg(feature = "enable_debug")]
    render_tiles_path(current_unit, &tiles_path, debug);

    let local_destination = if !tiles_path.is_empty() {
        tiles_path[0].bottom()
    } else {
        global_destination
    };

    #[cfg(feature = "enable_debug")]
    debug.log(format!("[{}] global_destination: {:?} local_destination: {:?}", current_unit.id, global_destination, local_destination));

    let simulator = Simulator::new(&world, current_unit.id);
    let planner = Planner::new(local_destination, world.config(), simulator, world.max_distance());
    let plan = planner.make(world.current_tick(), rng, debug);

    if plan.transitions.is_empty() {
        return get_quickstart_action(current_unit, local_destination, aim, shoot, world);
    }

    #[cfg(feature = "enable_debug")]
    debug.log(format!("[{}] plan_score={}, transitions: {:?}", current_unit.id, plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>()));

    let mut action = plan.transitions[0].action.clone();
    action.shoot = shoot;
    action.aim = aim.as_model();
    action.swap_weapon = should_swap_weapon(current_unit, shoot, world);
    action.plant_mine = should_plant_mine(current_unit, world);

    action
}

fn get_quickstart_action(current_unit: &Unit, target: Vec2, aim: Vec2, shoot: bool, world: &World) -> UnitAction {
    let mut jump = target.y() > current_unit.position.y;
    if target.x() > current_unit.position.x
        && world.get_tile(Location::new((current_unit.position.x + 1.0) as usize, (current_unit.position.y) as usize))
            == Tile::Wall
    {
        jump = true
    }
    if target.x() < current_unit.position.x
        && world.get_tile(Location::new((current_unit.position.x - 1.0) as usize, (current_unit.position.y) as usize))
            == Tile::Wall
    {
        jump = true
    }
    UnitAction {
        velocity: target.x() - current_unit.position.x,
        jump,
        jump_down: target.y() < current_unit.position.y,
        shoot,
        aim: aim.as_model(),
        reload: false,
        swap_weapon: false,
        plant_mine: false,
    }
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
fn render_tiles_path(unit: &Unit, tiles_path: &Vec<Location>, debug: &mut Debug) {
    if tiles_path.is_empty() {
        return;
    }

    debug.draw(CustomData::Line {
        p1: unit.rect().center().as_model_f32(),
        p2: tiles_path[0].center().as_model_f32(),
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
    });

    for tile in 0 .. tiles_path.len() - 1 {
        debug.draw(CustomData::Line {
            p1: tiles_path[tile].center().as_model_f32(),
            p2: tiles_path[tile + 1].center().as_model_f32(),
            width: 0.1,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
        });
    }
}

#[cfg(feature = "enable_debug")]
fn render_aim(unit: &Unit, opponent: &Unit, world: &World, debug: &mut Debug) {
    let mut s = Vec::new();
    for position in WalkGrid::new(unit.rect().center(), opponent.rect().center()) {
        s.push(position);
        debug.draw(CustomData::Rect {
            pos: position.as_location().as_model_f32(),
            size: Vec2F32 { x: 1.0, y: 1.0 },
            color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.66 },
        });
    }
    if let Some(weapon) = unit.weapon.as_ref() {
        let source = unit.rect().center();
        let direction = (opponent.rect().center() - source).normalized();
        let to_target = direction * world.max_distance();
        let left = direction.left() * weapon.params.bullet.size;
        let right = direction.right() * weapon.params.bullet.size;
        let number_of_directions = world.config().optimal_action_number_of_directions;

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
                #[cfg(feature = "enable_debug")]
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
