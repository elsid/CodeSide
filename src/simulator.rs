use model::{
    Bullet,
    Level,
    Player,
    Properties,
    Tile,
    Unit,
    UnitAction,
};
use crate::my_strategy::level::get_tile;
use crate::my_strategy::random::{Rng, XorShiftRng};
use crate::my_strategy::rect::Rect;
use crate::my_strategy::vec2::Vec2;
use crate::my_strategy::world::World;

#[derive(Clone)]
pub struct Simulator {
    players: Vec<Player>,
    units: Vec<UnitExt>,
    bullets: Vec<BulletExt>,
    properties: Properties,
    level: Level,
    borders: Vec2,
    current_tick: i32,
    current_time: f64,
    current_micro_tick: i32,
    me_index: usize,
}

impl Simulator {
    pub fn new(world: &World, me_id: i32) -> Self {
        let units: Vec<UnitExt> = world.units().iter()
            .map(|unit| {
                let mut base = unit.clone();
                base.on_ladder = is_on_ladder(&unit, &world);
                base.on_ground = is_on_ground(&unit, &world);
                UnitExt {
                    base,
                    action: UnitAction {
                        velocity: 0.0,
                        jump: false,
                        jump_down: false,
                        aim: model::Vec2F64 {
                            x: 0.0,
                            y: 0.0,
                        },
                        shoot: false,
                        swap_weapon: false,
                        plant_mine: false,
                    },
                    is_me: unit.id == me_id,
                    ignore: false,
                }
            })
            .collect();
        let me_index = units.iter().position(|v| v.is_me()).unwrap();
        let bullets: Vec<BulletExt> = world.bullets().iter()
            .map(|bullet| {
                BulletExt {
                    base: bullet.clone(),
                }
            })
            .collect();
        Simulator {
            players: world.players().clone(),
            units,
            bullets,
            properties: world.properties().clone(),
            level: world.level().clone(),
            borders: world.size(),
            current_tick: 0,
            current_time: 0.0,
            current_micro_tick: 0,
            me_index,
        }
    }

    pub fn me(&self) -> &UnitExt {
        &self.units[self.me_index]
    }

    pub fn me_mut(&mut self) -> &mut UnitExt {
        &mut self.units[self.me_index]
    }

    pub fn tick(&mut self, time_interval: f64, micro_ticks_per_tick: usize, rng: &mut XorShiftRng) {
        let micro_tick_time_interval = time_interval / micro_ticks_per_tick as f64;
        for _ in 0..micro_ticks_per_tick {
            self.micro_tick(micro_tick_time_interval, rng);
        }
        self.current_tick += 1;
        self.current_time += time_interval;
        self.me_index = self.units.iter().position(|v| v.is_me()).unwrap();
    }

    fn micro_tick(&mut self, time_interval: f64, rng: &mut XorShiftRng) {
        rng.shuffle(&mut self.units[..]);
        rng.shuffle(&mut self.bullets[..]);

        for unit in self.units.iter_mut() {
            if unit.ignore() {
                continue;
            }
            if unit.action().velocity != 0.0 {
                unit.shift_x(unit.action().velocity.min(self.properties.unit_max_horizontal_speed) * time_interval);
            }
        }

        for unit in self.units.iter_mut() {
            if unit.ignore {
                continue;
            }
            let tile_y = unit.position().y() as usize;
            let min_y = if tile_y > 0 { tile_y - 1 } else { 0 };
            let left = unit.left() as usize;
            let right = unit.right() as usize;
            for y in min_y .. tile_y + 2 {
                if get_tile(&self.level, left, y) == Tile::Wall {
                    let penetration = make_tile_rect(left, y).collide(&unit.rect());
                    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
                        unit.shift_x(-penetration.x());
                    }
                }
                if get_tile(&self.level, right, y) == Tile::Wall {
                    let penetration = make_tile_rect(right, y).collide(&unit.rect());
                    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
                         unit.shift_x(penetration.x());
                    }
                }
            }
        }

        for unit in self.units.iter_mut() {
            if unit.ignore() {
                continue;
            }
            if unit.action.jump_down {
                unit.shift_y(-self.properties.unit_jump_speed * time_interval);
            } else if unit.action.jump {
                unit.shift_y(self.properties.unit_jump_speed * time_interval);
            } else {
                unit.shift_y(-self.properties.unit_fall_speed * time_interval);
            }
            unit.set_on_ladder(false);
            unit.set_on_ground(false);
        }

        for unit in self.units.iter_mut() {
            if unit.ignore() {
                continue;
            }
            let tile_x = unit.position().x() as usize;
            let top = unit.top() as usize;
            let bottom = unit.bottom() as usize;
            for x in tile_x - 1 .. tile_x + 1 {
                let top_tile = get_tile(&self.level, x, top);
                let on_ladder = unit.on_ladder() || can_use_ladder(&unit, x, top);
                unit.set_on_ladder(on_ladder);
                if top_tile == Tile::Wall
                    || (top_tile == Tile::Ladder && on_ladder && !unit.action().jump_down) {
                    let penetration = make_tile_rect(x, top).collide(&unit.rect());
                    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
                        unit.shift_y(penetration.y());
                    }
                }
                let bottom_tile = get_tile(&self.level, x, bottom);
                if bottom_tile == Tile::Wall
                    || ((bottom_tile == Tile::Ladder || bottom_tile == Tile::Platform) && !unit.action().jump_down) {
                    let penetration = make_tile_rect(x, bottom).collide(&unit.rect());
                    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
                        unit.shift_y(-penetration.y());
                        unit.set_on_ground(true);
                    }
                }
            }
        }

        self.current_micro_tick += 1;
    }
}

#[derive(Clone, Debug)]
pub struct UnitExt {
    base: Unit,
    action: UnitAction,
    is_me: bool,
    ignore: bool,
}

impl UnitExt {
    pub fn is_me(&self) -> bool {
        self.is_me
    }

    pub fn ignore(&self) -> bool {
        self.ignore
    }

    pub fn position(&self) -> Vec2 {
        Vec2::from_model(&self.base.position)
    }

    pub fn set_position(&mut self, value: Vec2) {
        self.base.position = value.as_model();
    }

    pub fn size(&self) -> Vec2 {
        Vec2::from_model(&self.base.size)
    }

    pub fn action(&self) -> &UnitAction {
        &self.action
    }

    pub fn action_mut(&mut self) -> &mut UnitAction {
        &mut self.action
    }

    pub fn walked_right(&self) -> bool {
        self.base.walked_right
    }

    pub fn set_walked_right(&mut self, value: bool) {
        self.base.walked_right = value
    }

    pub fn on_ladder(&self) -> bool {
        self.base.on_ladder
    }

    pub fn set_on_ladder(&mut self, value: bool) {
        self.base.on_ladder = value
    }

    pub fn on_ground(&self) -> bool {
        self.base.on_ground
    }

    pub fn set_on_ground(&mut self, value: bool) {
        self.base.on_ground = value
    }

    pub fn shift_x(&mut self, value: f64) {
        self.base.position.x += value;
    }

    pub fn shift_y(&mut self, value: f64) {
        self.base.position.y += value;
    }

    pub fn right(&self) -> f64 {
        self.base.position.x + self.half_width()
    }

    pub fn left(&self) -> f64 {
        self.base.position.x - self.half_width()
    }

    pub fn top(&self) -> f64 {
        self.base.position.y + self.base.size.y
    }

    pub fn bottom(&self) -> f64 {
        self.base.position.y
    }

    pub fn half_width(&self) -> f64 {
        self.base.size.x / 2.0
    }

    pub fn half_height(&self) -> f64 {
        self.base.size.y / 2.0
    }

    pub fn set_x(&mut self, value: f64) {
        self.base.position.x = value;
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.base.position.x, self.base.position.y + self.half_height())
    }

    pub fn half(&self) -> Vec2 {
        Vec2::new(self.half_width(), self.half_height())
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.center(), self.half())
    }
}

#[derive(Clone, Debug)]
pub struct BulletExt {
    pub base: Bullet,
}

fn is_on_ladder(unit: &Unit, world: &World) -> bool {
    world.tile_by_position(Vec2::from_model(&unit.position)) == Tile::Ladder
}

fn is_on_ground(unit: &Unit, world: &World) -> bool {
    unit.on_ground
}

fn can_use_ladder(unit: &UnitExt, x: usize, y: usize) -> bool {
    let center = unit.center();
    (center.x() - (x as f64 + 0.5)).abs() <= 0.5
        && unit.top() - y as f64 >= 0.0
        && (y + 1) as f64 - center.y() >= 0.0
}

fn make_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(Vec2::new(x as f64 + 0.5, y as f64 + 0.5), Vec2::new(0.5, 0.5))
}
