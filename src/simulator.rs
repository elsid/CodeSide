use model::{
    Bullet,
    Level,
    Player,
    Properties,
    Tile,
    Unit,
    UnitAction,
    Vec2F64,
};
use crate::my_strategy::{
    Rect,
    Rng,
    Vec2,
    World,
    XorShiftRng,
    get_tile,
};

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
        let player_id = world.get_unit(me_id).player_id;
        let units: Vec<UnitExt> = world.units().iter()
            .map(|unit| {
                let is_me = unit.id == me_id;
                let is_teammate = unit.player_id == player_id;
                UnitExt::new(unit.clone(), is_me, is_teammate)
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

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn units(&self) -> &Vec<UnitExt> {
        &self.units
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
            unit.move_by_x(unit.action.velocity.min(self.properties.unit_max_horizontal_speed) * time_interval);
        }

        for i in 0 .. self.units.len() - 1 {
            if self.units[i].ignore {
                continue;
            }
            let (left, right) = self.units.split_at_mut(i + 1);
            for j in 0 .. right.len() {
                if right[j].ignore {
                    continue;
                }
                collide_units_by_x(&mut left[i], &mut right[j]);
            }
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
                    match get_tile(&self.level, x, y) {
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
                unit.move_by_y(unit.base.jump_state.speed * jump_time);
                if unit.base.jump_state.max_time == 0.0 {
                    cancel_jump(unit);
                }
            } else {
                unit.move_by_y(-self.properties.unit_fall_speed * time_interval);
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
                        cancel_jump(unit);
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

        for i in 0 .. self.units.len() - 1 {
            if self.units[i].ignore {
                continue;
            }
            let (left, right) = self.units.split_at_mut(i + 1);
            for j in 0 .. right.len() {
                if right[j].ignore {
                    continue;
                }
                collide_units_by_y(&mut left[i], &mut right[j]);
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
    is_teammate: bool,
    ignore: bool,
    moved: Vec2,
}

impl UnitExt {
    pub fn new(base: Unit, is_me: bool, is_teammate: bool) -> Self {
        Self {
            base,
            action: UnitAction {
                velocity: 0.0,
                jump: false,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0,
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            },
            is_me,
            is_teammate,
            ignore: false,
            moved: Vec2::zero(),
        }
    }

    pub fn is_me(&self) -> bool {
        self.is_me
    }

    pub fn position(&self) -> Vec2 {
        Vec2::from_model(&self.base.position)
    }

    pub fn action_mut(&mut self) -> &mut UnitAction {
        &mut self.action
    }

    pub fn shift_by_x(&mut self, value: f64) {
        self.base.position.x += value;
    }

    pub fn shift_by_y(&mut self, value: f64) {
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
        self.base.size.x * 0.5
    }

    pub fn half_height(&self) -> f64 {
        self.base.size.y * 0.5
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

    pub fn health(&self) -> i32 {
        self.base.health
    }

    pub fn is_teammate(&self) -> bool {
        self.is_teammate
    }

    pub fn move_by_x(&mut self, value: f64) {
        self.shift_by_x(value);
        self.moved.set_x(value);
    }

    pub fn move_by_y(&mut self, value: f64) {
        self.shift_by_y(value);
        self.moved.set_y(value);
    }

    pub fn moved(&self) -> Vec2 {
        self.moved
    }
}

#[derive(Clone, Debug)]
pub struct BulletExt {
    base: Bullet,
}

fn can_use_ladder(unit: &UnitExt, x: usize, y: usize) -> bool {
    let center = unit.center();
    (center.x() - (x as f64 + 0.5)).abs() <= 0.5
        && unit.top() - y as f64 >= 0.0
        && (y + 1) as f64 - center.y() >= 0.0
}

fn cancel_jump(unit: &mut UnitExt) {
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
        unit.shift_by_x(sign * penetration.x());
    }
}

fn collide_by_y(unit: &mut UnitExt, x: usize, y: usize, sign: f64) {
    let penetration = make_tile_rect(x, y).collide(&unit.rect());
    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
        let dy = sign * penetration.y();
        unit.shift_by_y(dy);
        unit.moved.add_y(dy);
    }
}

pub fn collide_units_by_x(a: &mut UnitExt, b: &mut UnitExt) {
    let penetration = a.rect().collide(&b.rect());
    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
        let sum = a.moved().x().abs() + b.moved().x().abs();
        let (a_weight, b_weight) = if sum == 0.0 {
            if a.position().x() < b.position().x() {
                (0.5, -0.5)
            } else {
                (-0.5, 0.5)
            }
        } else {
            (a.moved().x() / sum, b.moved().x() / sum)
        };
        a.shift_by_x(penetration.x() * a_weight);
        b.shift_by_x(penetration.x() * b_weight);
    }
}

fn collide_units_by_y(a: &mut UnitExt, b: &mut UnitExt) {
    let penetration = a.rect().collide(&b.rect());
    if penetration.x() < -std::f64::EPSILON && penetration.y() < -std::f64::EPSILON {
        let sum = a.moved().y().abs() + b.moved().y().abs();
        let (a_weight, b_weight) = if sum == 0.0 {
            if a.position().y() < b.position().y() {
                (0.5, -0.5)
            } else {
                (-0.5, 0.5)
            }
        } else {
            (a.moved().y() / sum, b.moved().y() / sum)
        };
        a.shift_by_y(penetration.y() * a_weight);
        b.shift_by_y(penetration.y() * b_weight);
    }
}

fn make_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(Vec2::new(x as f64 + 0.5, y as f64 + 0.5), Vec2::new(0.5, 0.5))
}
