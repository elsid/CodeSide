use model::{
    Bullet,
    Item,
    JumpState,
    LootBox,
    Mine,
    MineState,
    Properties,
    Unit,
    Weapon,
    WeaponParams,
    WeaponType,
};

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
    },
    my_strategy::{
        Level,
        Rect,
        UnitExt,
        Vec2,
        World,
        make_weapon,
    }
};

#[derive(Debug)]
pub struct WeaponWrapper<'a>(pub &'a Weapon);

impl<'a> PartialEq for WeaponWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        let Self(lhs) = self;
        let Self(rhs) = other;
        (
            WeaponParamsWrapper(&lhs.params),
            &lhs.typ,
            lhs.magazine,
            lhs.was_shooting,
            lhs.spread,
            lhs.fire_timer,
            lhs.last_angle,
            lhs.last_fire_tick,
        ).eq(&(
            WeaponParamsWrapper(&rhs.params),
            &rhs.typ,
            rhs.magazine,
            rhs.was_shooting,
            rhs.spread,
            rhs.fire_timer,
            rhs.last_angle,
            rhs.last_fire_tick,
        ))
    }
}

impl<'a> Eq for WeaponWrapper<'a> {}

#[derive(Debug)]
pub struct WeaponParamsWrapper<'a>(pub &'a WeaponParams);

impl<'a> PartialEq for WeaponParamsWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        let Self(lhs) = self;
        let Self(rhs) = other;
        (
            lhs.magazine_size,
            lhs.fire_rate,
            lhs.reload_time,
            lhs.min_spread,
            lhs.max_spread,
            lhs.recoil,
            // BulletParamsWrapper(&lhs.bullet),
            // ExplosionParamsWrapper(&lhs.explosion),
        ).eq(&(
            rhs.magazine_size,
            rhs.fire_rate,
            rhs.reload_time,
            rhs.min_spread,
            rhs.max_spread,
            rhs.recoil,
            // BulletParamsWrapper(&rhs.bullet),
            // ExplosionParamsWrapper(&rhs.explosion),
        ))
    }
}

impl<'a> Eq for WeaponParamsWrapper<'a> {}

pub fn make_unit_rect(position: Vec2, properties: &Properties) -> Rect {
    Rect::new(
        position + Vec2::new(0.0, properties.unit_size.y / 2.0),
        Vec2::from_model(&properties.unit_size)
    )
}

pub fn updated_world(mut world: World) -> World {
    let game = world.game().clone();
    world.update(&game);
    world
}

pub fn with_my_unit_with_weapon(world: World, weapon_type: WeaponType) -> World {
    let mut game = world.game().clone();
    let me_index = game.units.iter().position(|v| v.id == EXAMPLE_MY_UNIT_ID).unwrap();
    game.units[me_index].weapon = Some(make_weapon(weapon_type.clone(), world.properties()));
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_opponent_unit_with_weapon_type(world: World, weapon_type: WeaponType) -> World {
    let mut game = world.game().clone();
    let unit_index = game.units.iter().position(|v| v.player_id != world.player_id()).unwrap();
    game.units[unit_index].weapon = Some(make_weapon(weapon_type.clone(), world.properties()));
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_my_position(world: World, position: Vec2) -> World {
    with_unit_position(world, EXAMPLE_MY_UNIT_ID, position)
}

pub fn with_unit_position(world: World, unit_id: i32, position: Vec2) -> World {
    let mut game = world.game().clone();
    let index = game.units.iter().position(|v| v.id == unit_id).unwrap();
    game.units[index].position = position.as_model();
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_bullet(world: World, weapon_type: WeaponType, position: Vec2, direction: Vec2, unit_id: i32) -> World {
    let mut game = world.game().clone();
    let params = &world.properties().weapon_params.get(&weapon_type).unwrap();
    game.bullets.push(Bullet {
        weapon_type: weapon_type,
        unit_id: unit_id,
        player_id: world.get_unit(unit_id).player_id,
        position: position.as_model(),
        velocity: (direction.normalized() * params.bullet.speed).as_model(),
        damage: params.bullet.damage,
        size: params.bullet.size,
        explosion_params: params.explosion.clone(),
    });
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_mine(world: World, position: Vec2, player_id: i32) -> World {
    let mut game = world.game().clone();
    game.mines.push(Mine {
        player_id,
        position: position.as_model(),
        size: world.properties().mine_size.clone(),
        state: MineState::Preparing,
        timer: Some(world.properties().mine_prepare_time),
        trigger_radius: world.properties().mine_trigger_radius,
        explosion_params: world.properties().mine_explosion_params.clone(),
    });
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_loot_box(world: World, item: Item, position: Vec2) -> World {
    let mut game = world.game().clone();
    game.loot_boxes.push(LootBox {
        position: position.as_model(),
        size: world.properties().loot_box_size.clone(),
        item: item,
    });
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn make_unit_ext(position: Vec2, properties: &Properties) -> UnitExt {
    let base = Unit {
        player_id: 0,
        id: 0,
        health: properties.unit_max_health,
        position: position.as_model(),
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: false,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    };
    UnitExt::new(base, false, false, 0)
}

pub fn with_level(world: World, level: Level) -> World {
    let mut game = world.game().clone();
    game.level = level.as_model();
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_unit_health(world: World, unit_id: i32, health: i32) -> World {
    let mut game = world.game().clone();
    let index = game.units.iter().position(|v| v.id == unit_id).unwrap();
    game.units[index].health = health;
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_unit_with_mines(world: World, unit_id: i32, mines: i32) -> World {
    let mut game = world.game().clone();
    let index = game.units.iter().position(|v| v.id == unit_id).unwrap();
    game.units[index].mines = mines;
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_unit_weapon_fire_timer(world: World, unit_id: i32, fire_timer: Option<f64>) -> World {
    let mut game = world.game().clone();
    let index = game.units.iter().position(|v| v.id == unit_id).unwrap();
    if let Some(weapon) = game.units[index].weapon.as_mut() {
        weapon.fire_timer = fire_timer;
    }
    World::new(world.config().clone(), world.player_id(), game)
}

pub fn with_unit_with_weapon(world: World, unit_id: i32, weapon_type: WeaponType) -> World {
    let mut game = world.game().clone();
    let index = game.units.iter().position(|v| v.id == unit_id).unwrap();
    game.units[index].weapon = Some(make_weapon(weapon_type.clone(), world.properties()));
    World::new(world.config().clone(), world.player_id(), game)
}
