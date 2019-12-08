use model::{
    Bullet,
    ExplosionParams,
    Item,
    Level,
    LootBox,
    Player,
    Properties,
    Tile,
    Unit,
    UnitAction,
    Vec2F64,
    Weapon,
    WeaponType,
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
    loot_boxes: Vec<LootBoxExt>,
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
            .map(|bullet| BulletExt::new(bullet.clone()))
            .collect();
        let loot_boxes: Vec<LootBoxExt> = world.loot_boxes().iter()
            .map(|v| LootBoxExt::new(v.clone()))
            .collect();
        Simulator {
            players: world.players().clone(),
            units,
            bullets,
            loot_boxes,
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

    pub fn bullets(&self) -> &Vec<BulletExt> {
        &self.bullets
    }

    pub fn loot_boxes(&self) -> &Vec<LootBoxExt> {
        &self.loot_boxes
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

        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }

            self.units[unit].base.on_ladder = false;

            let velocity_x = self.units[unit].action.velocity.min(self.properties.unit_max_horizontal_speed) * time_interval;
            self.units[unit].move_by_x(velocity_x);

            let (left, right) = self.units.split_at_mut(unit + 1);
            let (left_left, left_right) = left.split_at_mut(left.len() - 1);

            for i in 0 .. left_left.len() {
                if left_left[i].ignore() {
                    continue;
                }
                collide_units_by_x(&mut left_right[0], &mut left_left[i]);
            }

            for i in 0 .. right.len() {
                if right[i].ignore() {
                    continue;
                }
                collide_units_by_x(&mut left_right[0], &mut right[i]);
            }

            let min_y = self.units[unit].bottom() as usize;
            let max_y = (self.units[unit].top() as usize + 1).min(self.level.tiles[0].len());
            let left = self.units[unit].left() as usize;
            let right = self.units[unit].right() as usize;
            for y in min_y .. max_y {
                for &x in &[left, right] {
                    match get_tile(&self.level, x, y) {
                        Tile::Wall => {
                            collide_by_x(&mut self.units[unit], x, y);
                        },
                        Tile::Ladder => {
                            self.units[unit].base.on_ladder = self.units[unit].base.on_ladder || can_use_ladder(&self.units[unit], x, y);
                        },
                        Tile::JumpPad => {
                            start_pad_jump(&mut self.units[unit], &self.properties);
                        },
                        _ => (),
                    }
                }
            }

            if self.units[unit].base.jump_state.can_jump && (self.units[unit].action.jump || !self.units[unit].base.jump_state.can_cancel) {
                let jump_time = shift_jump_max_time(&mut self.units[unit], time_interval);
                let velocity_y = self.units[unit].base.jump_state.speed * jump_time;
                self.units[unit].move_by_y(velocity_y);
                if self.units[unit].base.jump_state.max_time == 0.0 {
                    cancel_jump(&mut self.units[unit]);
                }
            } else {
                self.units[unit].move_by_y(-self.properties.unit_fall_speed * time_interval);
            }
            self.units[unit].base.on_ground = false;

            let (left, right) = self.units.split_at_mut(unit + 1);
            let (left_left, left_right) = left.split_at_mut(left.len() - 1);

            for i in 0 .. left_left.len() {
                if left_left[i].ignore() {
                    continue;
                }
                collide_units_by_y(&mut left_right[0], &mut left_left[i]);
            }

            for i in 0 .. right.len() {
                if right[i].ignore() {
                    continue;
                }
                collide_units_by_y(&mut left_right[0], &mut right[i]);
            }

            let min_x = self.units[unit].left() as usize;
            let max_x = (self.units[unit].right() as usize + 1).min(self.level.tiles.len());
            let top = self.units[unit].top() as usize;
            let bottom = self.units[unit].bottom() as usize;
            for x in min_x .. max_x {
                match get_tile(&self.level, x, bottom) {
                    Tile::Wall => {
                        collide_by_y(&mut self.units[unit], x, bottom);
                        allow_jump(&mut self.units[unit], &self.properties);
                    },
                    Tile::Ladder => {
                        self.units[unit].base.on_ladder = self.units[unit].base.on_ladder || can_use_ladder(&self.units[unit], x, bottom);
                        if !self.units[unit].base.on_ladder {
                            collide_by_y(&mut self.units[unit], x, bottom);
                            allow_jump(&mut self.units[unit], &self.properties);
                        }
                    },
                    Tile::Platform => {
                        if !self.units[unit].action.jump_down {
                            collide_by_y(&mut self.units[unit], x, bottom);
                            allow_jump(&mut self.units[unit], &self.properties);
                        }
                    },
                    Tile::JumpPad => {
                        start_pad_jump(&mut self.units[unit], &self.properties);
                    },
                    _ => (),
                }
                match get_tile(&self.level, x, top) {
                    Tile::Wall => {
                        collide_by_y(&mut self.units[unit], x, top);
                        self.units[unit].base.on_ground = true;
                        cancel_jump(&mut self.units[unit]);
                    },
                    Tile::Ladder => {
                        self.units[unit].base.on_ladder = self.units[unit].base.on_ladder || can_use_ladder(&self.units[unit], x, top);
                        self.units[unit].base.on_ground = true;
                        allow_jump(&mut self.units[unit], &self.properties);
                    },
                    Tile::JumpPad => {
                        start_pad_jump(&mut self.units[unit], &self.properties);
                    },
                    _ => (),
                }
            }
        }

        for bullet in 0 .. self.bullets.len() {
            if self.bullets[bullet].hit {
                continue;
            }
            self.bullets[bullet].advance(time_interval);
            if self.collide_bullet_and_units(bullet) {
                continue;
            }
            self.collide_bulles_and_tiles(bullet);
        }

        for loot_box in 0 .. self.loot_boxes.len() {
            if self.loot_boxes[loot_box].used {
                continue;
            }
            for unit in 0 .. self.units.len() {
                if self.units[unit].ignore() {
                    continue;
                }
                if pickup(&self.properties, &mut self.loot_boxes[loot_box], &mut self.units[unit]) {
                    break;
                }
            }
        }

        self.bullets = self.bullets.iter().filter(|v| !v.hit).map(|v| v.clone()).collect();

        self.current_micro_tick += 1;
    }

    fn collide_bullet_and_units(&mut self, bullet: usize) -> bool {
        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }
            if let Some(explosion) = collide_unit_and_bullet(&mut self.bullets[bullet], &mut self.units[unit]) {
                for i in 0 .. self.units.len() {
                    if self.units[i].ignore() {
                        continue;
                    }
                    explode(&explosion, &mut self.units[i]);
                }
            }
            if self.bullets[bullet].hit {
                return true;
            }
        }
        false
    }

    fn collide_bulles_and_tiles(&mut self, bullet: usize) -> bool {
        let min_x = self.bullets[bullet].left() as usize;
        let max_x = (self.bullets[bullet].right() as usize + 1).min(self.level.tiles.len());
        let min_y = self.bullets[bullet].bottom() as usize;
        let max_y = (self.bullets[bullet].top() as usize + 1).min(self.level.tiles[0].len());
        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match get_tile(&self.level, x, y) {
                    Tile::Wall => {
                        if let Some(explosion) = collide_unit_and_tile(x, y, &mut self.bullets[bullet]) {
                            for unit in 0 .. self.units.len() {
                                if self.units[unit].ignore() {
                                    continue;
                                }
                                explode(&explosion, &mut self.units[unit]);
                            }
                        }
                        if self.bullets[bullet].hit {
                            return true;
                        }
                    },
                    _ => (),
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct UnitExt {
    base: Unit,
    action: UnitAction,
    is_me: bool,
    is_teammate: bool,
    ignore: bool,
    velocity: Vec2,
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
            velocity: Vec2::zero(),
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
        self.velocity.set_x(value);
    }

    pub fn move_by_y(&mut self, value: f64) {
        self.shift_by_y(value);
        self.velocity.set_y(value);
    }

    pub fn ignore(&self) -> bool {
        self.ignore || self.base.health <= 0
    }

    pub fn weapon(&self) -> &Option<Weapon> {
        &self.base.weapon
    }

    pub fn damage(&mut self, value: i32) {
        self.base.health -= value;
    }

    pub fn heal(&mut self, value: i32, properties: &Properties) {
        self.base.health = (self.base.health + value).min(properties.unit_max_health);
    }

    pub fn mines(&self) -> i32 {
        self.base.mines
    }
}

#[derive(Clone, Debug)]
pub struct BulletExt {
    base: Bullet,
    hit: bool,
}

impl BulletExt {
    pub fn new(base: Bullet) -> Self {
        Self { base, hit: false }
    }

    pub fn half_size(&self) -> f64 {
        self.base.size * 0.5
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.base.position.x, self.base.position.y)
    }

    pub fn half(&self) -> Vec2 {
        let half_size = self.half_size();
        Vec2::new(half_size, half_size)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.center(), self.half())
    }

    pub fn explosion_params(&self) -> &Option<ExplosionParams> {
        &self.base.explosion_params
    }

    pub fn advance(&mut self, time_interval: f64) {
        self.base.position.x += self.base.velocity.x * time_interval;
        self.base.position.y += self.base.velocity.y * time_interval;
    }

    pub fn right(&self) -> f64 {
        self.base.position.x + self.half_size()
    }

    pub fn left(&self) -> f64 {
        self.base.position.x - self.half_size()
    }

    pub fn top(&self) -> f64 {
        self.base.position.y + self.half_size()
    }

    pub fn bottom(&self) -> f64 {
        self.base.position.y - self.half_size()
    }
}

#[derive(Clone, Debug)]
pub struct Explosion {
    params: ExplosionParams,
    position: Vec2,
}

impl Explosion {
    pub fn rect(&self) -> Rect {
        Rect::new(self.position, Vec2::new(self.params.radius, self.params.radius))
    }
}

#[derive(Clone, Debug)]
pub struct LootBoxExt {
    base: LootBox,
    used: bool,
}

impl LootBoxExt {
    pub fn new(base: LootBox) -> Self {
        Self { base, used: false }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(Vec2::from_model(&self.base.position), Vec2::from_model(&self.base.size))
    }
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

fn collide_by_x(unit: &mut UnitExt, x: usize, y: usize) {
    let penetration = make_tile_rect(x, y).collide(&unit.rect());
    if penetration.x() >= -std::f64::EPSILON
        || penetration.y() >= -std::f64::EPSILON
        || unit.velocity.x() == 0.0 {
        return;
    }
    let dx = -penetration.x().abs().min(unit.velocity.x().abs()).copysign(unit.velocity.x());
    unit.shift_by_x(dx);
    unit.velocity.add_x(dx);
}

fn collide_by_y(unit: &mut UnitExt, x: usize, y: usize) {
    let penetration = make_tile_rect(x, y).collide(&unit.rect());
    if penetration.x() >= -std::f64::EPSILON
        || penetration.y() >= -std::f64::EPSILON
        || unit.velocity.y() == 0.0 {
        return;
    }
    let dy = -penetration.y().abs().min(unit.velocity.y().abs()).copysign(unit.velocity.y());
    unit.shift_by_y(dy);
    unit.velocity.add_y(dy);
}

pub fn collide_units_by_x(a: &mut UnitExt, b: &mut UnitExt) {
    let penetration = a.rect().collide(&b.rect());
    if penetration.x() >= -std::f64::EPSILON
        || penetration.y() >= -std::f64::EPSILON
        || (a.velocity.x() == 0.0 && b.velocity.x() == 0.0) {
        return;
    }
    let (a_vel, b_vel) = if a.position().x() < b.position().x() {
        get_shift_factors(a.velocity.x(), b.velocity.x())
    } else {
        let (b_vel, a_vel) = get_shift_factors(b.velocity.x(), a.velocity.x());
        (a_vel, b_vel)
    };
    a.shift_by_x(-penetration.x() * a_vel);
    b.shift_by_x(-penetration.x() * b_vel);
}

fn collide_units_by_y(a: &mut UnitExt, b: &mut UnitExt) {
    let penetration = a.rect().collide(&b.rect());
    if penetration.x() >= -std::f64::EPSILON
        || penetration.y() >= -std::f64::EPSILON
        || (a.velocity.y() == 0.0 && b.velocity.y() == 0.0) {
        return;
    }
    let (a_vel, b_vel) = if a.position().y() < b.position().y() {
        get_shift_factors(a.velocity.y(), b.velocity.y())
    } else {
        let (b_vel, a_vel) = get_shift_factors(b.velocity.y(), a.velocity.y());
        (a_vel, b_vel)
    };
    a.shift_by_y(-penetration.y() * a_vel);
    b.shift_by_y(-penetration.y() * b_vel);
}

pub fn get_shift_factors(a_vel: f64, b_vel: f64) -> (f64, f64) {
    if a_vel == 0.0 && b_vel == 0.0 {
        (-0.5, 0.5)
    } else if a_vel == 0.0 {
        (0.0, 1.0)
    } else if b_vel == 0.0 {
        (-1.0, 0.0)
    } else if a_vel < 0.0 {
        (0.0, 1.0)
    } else if b_vel > 0.0 {
        (-1.0, 0.0)
    } else {
        let sum = a_vel.abs() + b_vel.abs();
        (-a_vel / sum, -b_vel / sum)
    }
}

fn collide_unit_and_bullet(bullet: &mut BulletExt, unit: &mut UnitExt) -> Option<Explosion> {
    if !bullet.rect().has_collision(&unit.rect()) {
        return None;
    }
    bullet.hit = true;
    unit.damage(bullet.base.damage);
    bullet.explosion_params().as_ref()
        .map(|v| Explosion {params: v.clone(), position: bullet.center()})
}

fn explode(explosion: &Explosion, unit: &mut UnitExt) {
    if !explosion.rect().has_collision(&unit.rect()) {
        return;
    }
    unit.damage(explosion.params.damage);
}

fn collide_unit_and_tile(x: usize, y: usize, bullet: &mut BulletExt) -> Option<Explosion> {
    if !make_tile_rect(x, y).has_collision(&bullet.rect()) {
        return None;
    }
    bullet.hit = true;
    bullet.explosion_params().as_ref()
        .map(|v| Explosion {params: v.clone(), position: bullet.center()})
}

fn pickup(properties: &Properties, loot_box: &mut LootBoxExt, unit: &mut UnitExt) -> bool {
    if !loot_box.rect().has_collision(&unit.rect()) {
        return false;
    }
    match &loot_box.base.item {
        Item::HealthPack {health} => {
            if unit.health() >= properties.unit_max_health {
                false
            } else {
                unit.heal(*health, properties);
                loot_box.used = true;
                true
            }
        },
        Item::Weapon {weapon_type} => {
            if unit.action.swap_weapon || unit.weapon().is_none() {
                unit.base.weapon = Some(make_weapon(weapon_type.clone(), properties));
                loot_box.used = true;
                true
            } else {
                false
            }
        },
        Item::Mine {} => {
            unit.base.mines += 1;
            loot_box.used = true;
            true
        },
    }
}

fn make_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(Vec2::new(x as f64 + 0.5, y as f64 + 0.5), Vec2::new(0.5, 0.5))
}

fn make_weapon(weapon_type: WeaponType, properties: &Properties) -> Weapon {
    let params = &properties.weapon_params[&weapon_type];
    Weapon {
        params: params.clone(),
        typ: weapon_type,
        magazine: params.magazine_size,
        was_shooting: false,
        spread: 0.0,
        fire_timer: None,
        last_angle: None,
        last_fire_tick: None,
    }
}
