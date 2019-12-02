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

    pub fn current_tick(&self) -> i32 {
        self.current_tick
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
            if unit.ignore {
                continue;
            }
            unit.base.on_ladder = false;
            unit.shift_x(unit.action.velocity.min(self.properties.unit_max_horizontal_speed) * time_interval);
        }

        for unit in self.units.iter_mut() {
            if unit.ignore {
                continue;
            }
            let min_y = unit.bottom() as usize;
            let max_y = unit.top() as usize + 1;
            let left = unit.left() as usize;
            let right = unit.right() as usize;
            for y in min_y .. max_y {
                for &(x, sign) in &[(left, -1.0), (right, 1.0)] {
                    match get_tile(&self.level, left, y) {
                        Tile::Wall => {
                            collide_by_x(unit, x, y, sign);
                        },
                        Tile::Ladder => {
                            unit.base.on_ladder = unit.base.on_ladder || can_use_ladder(&unit, x, y);
                        },
                        Tile::JumpPad => {
                            start_pad_jump(unit, &self.properties);
                        },
                        _ => (),
                    }
                }
            }
        }

        for unit in self.units.iter_mut() {
            if unit.ignore {
                continue;
            }
            if unit.base.jump_state.can_jump && (unit.action.jump || !unit.base.jump_state.can_cancel) {
                let jump_time = shift_jump_max_time(unit, time_interval);
                unit.shift_y(unit.base.jump_state.speed * jump_time);
                if unit.base.jump_state.max_time == 0.0 {
                    cancel_jump(unit, &self.properties);
                }
            } else {
                unit.shift_y(-self.properties.unit_fall_speed * time_interval);
            }
            unit.base.on_ground = false;
        }

        for unit in self.units.iter_mut() {
            if unit.ignore {
                continue;
            }
            let min_x = unit.left() as usize;
            let max_x = unit.right() as usize + 1;
            let top = unit.top() as usize;
            let bottom = unit.bottom() as usize;
            for x in min_x .. max_x {
                match get_tile(&self.level, x, bottom) {
                    Tile::Wall => {
                        collide_by_y(unit, x, bottom, -1.0);
                        allow_jump(unit, &self.properties);
                    },
                    Tile::Ladder => {
                        unit.base.on_ladder = unit.base.on_ladder || can_use_ladder(&unit, x, bottom);
                        if !unit.base.on_ladder {
                            collide_by_y(unit, x, bottom, -1.0);
                            allow_jump(unit, &self.properties);
                        }
                    },
                    Tile::Platform => {
                        if !unit.action.jump_down {
                            collide_by_y(unit, x, bottom, -1.0);
                            allow_jump(unit, &self.properties);
                        }
                    },
                    Tile::JumpPad => {
                        start_pad_jump(unit, &self.properties);
                    },
                    _ => (),
                }
                match get_tile(&self.level, x, top) {
                    Tile::Wall => {
                        collide_by_y(unit, x, top, 1.0);
                        unit.base.on_ground = true;
                        cancel_jump(unit, &self.properties);
                    },
                    Tile::Ladder => {
                        unit.base.on_ladder = unit.base.on_ladder || can_use_ladder(&unit, x, top);
                        unit.base.on_ground = true;
                        allow_jump(unit, &self.properties);
                    },
                    Tile::JumpPad => {
                        start_pad_jump(unit, &self.properties);
                    },
                    _ => (),
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

fn cancel_jump(unit: &mut UnitExt, properties: &Properties) {
    unit.base.jump_state.can_jump = false;
    unit.base.jump_state.speed = 0.0;
    unit.base.jump_state.max_time = 0.0;
    unit.base.jump_state.can_cancel = false;
}

fn allow_jump(unit: &mut UnitExt, properties: &Properties) {
    unit.base.jump_state.can_jump = true;
    unit.base.jump_state.speed = properties.unit_jump_speed;
    unit.base.jump_state.max_time = properties.unit_jump_time;
    unit.base.jump_state.can_cancel = true;
}

fn start_pad_jump(unit: &mut UnitExt, properties: &Properties) {
    unit.base.jump_state.can_jump = true;
    unit.base.jump_state.speed = properties.jump_pad_jump_speed;
    unit.base.jump_state.max_time = properties.jump_pad_jump_time;
    unit.base.jump_state.can_cancel = false;
}

pub fn shift_jump_max_time(unit: &mut UnitExt, time_interval: f64) -> f64 {
    let max_time = unit.base.jump_state.max_time;
    unit.base.jump_state.max_time = unit.base.jump_state.max_time - time_interval;
    if unit.base.jump_state.max_time < 0.0 {
        unit.base.jump_state.max_time = 0.0;
        max_time
    } else {
        time_interval
    }
}

fn collide_by_x(unit: &mut UnitExt, x: usize, y: usize, sign: f64) {
    let penetration = make_tile_rect(x, y).collide(&unit.rect());
    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
        unit.shift_x(sign * penetration.x());
    }
}

fn collide_by_y(unit: &mut UnitExt, x: usize, y: usize, sign: f64) {
    let penetration = make_tile_rect(x, y).collide(&unit.rect());
    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
        unit.shift_y(sign * penetration.y());
    }
}

fn make_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(Vec2::new(x as f64 + 0.5, y as f64 + 0.5), Vec2::new(0.5, 0.5))
}
