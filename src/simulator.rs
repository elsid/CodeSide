use model::{
    Bullet,
    ExplosionParams,
    Item,
    LootBox,
    Mine,
    MineState,
    Player,
    Properties,
    Tile,
    Unit,
    UnitAction,
    Vec2F64,
    Weapon,
    WeaponType,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
use model::{
    ColorF32,
    CustomData,
};

use crate::my_strategy::{
    Debug as Dbg,
    Level,
    Location,
    Rect,
    Rng,
    Vec2,
    World,
    XorShiftRng,
    minimize1d,
    remove_if,
    root_search,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
use crate::my_strategy::{
    Rectangular,
};

#[derive(Clone)]
pub struct Simulator<'r> {
    players: Vec<Player>,
    units: Vec<UnitExt>,
    bullets: Vec<BulletExt>,
    mines: Vec<MineExt>,
    loot_boxes: Vec<LootBoxExt>,
    properties: Properties,
    level: &'r Level,
    current_tick: i32,
    current_time: f64,
    current_micro_tick: i32,
    unit_index: usize,
    player_index: usize,
    stats: Stats,
}

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub max_number_of_mines: usize,
    pub triggered_mines_by_me: usize,
}

impl<'r> Simulator<'r> {
    pub fn new(world: &'r World, me_id: i32) -> Self {
        let player_id = world.get_unit(me_id).player_id;
        let units: Vec<UnitExt> = world.units().iter()
            .map(|unit| {
                let is_me = unit.id == me_id;
                let is_teammate = unit.player_id == player_id;
                let player_index = world.players().iter().position(|v| unit.player_id == v.id).unwrap();
                UnitExt::new(unit.clone(), is_me, is_teammate, player_index)
            })
            .collect();
        let unit_index = units.iter().position(|v| v.is_me()).unwrap();
        let bullets: Vec<BulletExt> = world.bullets().iter()
            .map(|bullet| {
                let player_index = world.players().iter().position(|v| bullet.player_id == v.id).unwrap();
                BulletExt::new(bullet.clone(), player_index)
            })
            .collect();
        let loot_boxes: Vec<LootBoxExt> = world.loot_boxes().iter()
            .filter(|v| is_loot_box_enabled(v))
            .map(|v| LootBoxExt::new(v.clone()))
            .collect();
        let mines: Vec<MineExt> = world.mines().iter()
            .map(|mine| {
                let player_index = world.players().iter().position(|v| mine.player_id == v.id).unwrap();
                MineExt::new(mine.clone(), player_index)
            })
            .collect();
        Simulator {
            players: world.players().clone(),
            units,
            bullets,
            loot_boxes,
            mines,
            properties: world.properties().clone(),
            level: world.level(),
            current_tick: 0,
            current_time: 0.0,
            current_micro_tick: 0,
            unit_index,
            player_index: world.players().iter().position(|v| v.id == player_id).unwrap(),
            stats: Stats::default(),
        }
    }

    pub fn my_player(&self) -> &Player {
        &self.players[self.player_index]
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn unit(&self) -> &UnitExt {
        &self.units[self.unit_index]
    }

    pub fn unit_mut(&mut self) -> &mut UnitExt {
        &mut self.units[self.unit_index]
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

    pub fn mines(&self) -> &Vec<MineExt> {
        &self.mines
    }

    pub fn loot_boxes(&self) -> &Vec<LootBoxExt> {
        &self.loot_boxes
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn player(&self) -> &Player {
        &self.players[self.player_index]
    }

    pub fn opponent(&self) -> &Player {
        &self.players[(self.player_index + 1) % 2]
    }

    pub fn set_unit_action(&mut self, unit_id: i32, action: UnitAction) {
        if let Some(unit) = self.units.iter_mut().find(|v| v.base.id == unit_id) {
            unit.action = action;
        }
    }

    pub fn tick(&mut self, time_interval: f64, micro_ticks_per_tick: usize, rng: &mut XorShiftRng, debug: &mut Option<&mut Dbg>) {
        let micro_tick_time_interval = time_interval / micro_ticks_per_tick as f64;
        for _ in 0..micro_ticks_per_tick {
            self.micro_tick(micro_tick_time_interval, rng, debug);
        }
        self.current_tick += 1;
        self.current_time += time_interval;
        self.unit_index = self.units.iter().position(|v| v.is_me()).unwrap();
        self.stats.max_number_of_mines = self.stats.max_number_of_mines.max(self.mines.len());
        remove_if(&mut self.bullets, |v| v.hit);
        remove_if(&mut self.mines, |v| v.base.state == MineState::Exploded);
        remove_if(&mut self.loot_boxes, |v| v.used);
    }

    fn micro_tick(&mut self, time_interval: f64, rng: &mut XorShiftRng, debug: &mut Option<&mut Dbg>) {
        rng.shuffle(&mut self.units[..]);

        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }

            self.units[unit].reset_velocity();

            let velocity_x = self.units[unit].action.velocity.min(self.properties.unit_max_horizontal_speed) * time_interval;

            if velocity_x != 0.0 {
                self.units[unit].start_move_by_x(velocity_x);
                self.collide_moving_unit_and_tiles_by_x(unit);
                self.collide_units_by_x(unit);
                self.units[unit].finish_move_by_x();
            }

            self.collide_holding_unit_and_tiles(unit);

            if !self.units[unit].base.on_ladder || self.units[unit].action.jump_down || self.units[unit].action.jump {
                if self.units[unit].base.jump_state.can_jump && (self.units[unit].action.jump || !self.units[unit].base.jump_state.can_cancel) {
                    let jump_time = shift_jump_max_time(&mut self.units[unit], time_interval);
                    let velocity_y = self.units[unit].base.jump_state.speed * jump_time;
                    self.units[unit].start_move_by_y(velocity_y);
                    if self.units[unit].base.jump_state.max_time == 0.0 {
                        cancel_jump(&mut self.units[unit]);
                    }
                } else {
                    self.units[unit].start_move_by_y(-self.properties.unit_fall_speed * time_interval);
                    cancel_jump(&mut self.units[unit]);
                }
                self.units[unit].base.on_ground = false;
            }

            if self.units[unit].velocity_y != 0.0 {
                if self.units[unit].velocity_y > 0.0 {
                    self.collide_moving_up_unit_and_tiles_by_y(unit);
                } else {
                    self.collide_moving_down_unit_and_tiles_by_y(unit);
                }

                self.collide_units_by_y(unit);

                self.units[unit].finish_move_by_y();
            }

            self.collide_holding_unit_and_tiles(unit);

            #[cfg(feature = "verify_collisions")]
            self.verify_collisions(unit, "after_y");

            #[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
            {
                if let Some(d) = debug {
                    d.rect_border(&self.units[unit].base.rect(), ColorF32 { a: 0.01, r: 0.8, g: 0.8, b: 0.8 }, 0.1);
                }
            }
        }

        let mut explosions = Vec::new();

        for bullet in 0 .. self.bullets.len() {
            if self.bullets[bullet].hit {
                continue;
            }

            if self.collide_bullet_and_units(bullet, time_interval, &mut explosions, debug) {
                continue;
            }

            if self.collide_bulles_and_tiles(bullet, time_interval, &mut explosions) {
                continue
            }

            if self.collide_bullet_and_mines(bullet, time_interval, &mut explosions) {
                continue;
            }

            self.bullets[bullet].advance(time_interval);

            #[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
            {
                if let Some(d) = debug {
                    d.draw(self.bullets[bullet].rect().as_debug(ColorF32 { a: 0.5, r: 0.8, g: 0.4, b: 0.2 }));
                }
            }
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

        for mine in 0 .. self.mines.len() {
            if self.mines[mine].ignore() {
                continue;
            }
            if let Some(explosion) = update_mine(time_interval, &mut self.mines[mine]) {
                explosions.push(explosion);
            } else if self.mines[mine].base.state == MineState::Idle {
                for unit in 0 .. self.units.len() {
                    if self.units[unit].ignore() {
                        continue;
                    }
                    if activate(&self.properties, &mut self.mines[mine], &mut self.units[unit]) {
                        self.stats.triggered_mines_by_me += self.units[unit].is_me as usize;
                        break;
                    }
                }
            }
        }

        while let Some(explosion) = explosions.pop() {
            self.explode_units(&explosion);
            self.explode_mines(&explosion, &mut explosions);
            #[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
            {
                if let Some(d) = debug {
                    d.rect_border(&explosion.rect(), ColorF32 { a: 0.5, r: 0.8, g: 0.0, b: 0.0 }, 0.1);
                }
            }
        }

        #[cfg(feature = "simulator_weapon")]
        for unit in 0 .. self.units.len() {
            if let Some(weapon) = self.units[unit].base.weapon.as_mut() {
                weapon.fire_timer = if let Some(fire_timer) = weapon.fire_timer {
                    let new_fire_timer = fire_timer - time_interval;
                    if new_fire_timer > 0.0 {
                        Some(new_fire_timer)
                    } else {
                        None
                    }
                } else {
                    None
                };
            }
        }

        #[cfg(all(feature = "simulator_weapon", feature = "simulator_shoot"))]
        for unit in 0 .. self.units.len() {
            if self.units[unit].action.shoot {
                if let Some(bullet) = shoot(&mut self.units[unit]) {
                    self.bullets.push(bullet);
                }
            }
        }

        self.current_micro_tick += 1;
    }

    fn collide_moving_unit_and_tiles_by_x(&mut self, unit: usize) {
        let left = self.units[unit].holding_left();
        let right = self.units[unit].holding_right();
        let min_x = left.min(left + self.units[unit].velocity_x) as usize;
        let max_x = right.max(right + self.units[unit].velocity_x) as usize + 1;
        let min_y = self.units[unit].holding_bottom() as usize;
        let max_y = self.units[unit].holding_top() as usize + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        self.units[unit].collide_with_tile_by_x(x, y);
                    },
                    _ => (),
                }
                if self.units[unit].velocity_x == 0.0 {
                    return;
                }
            }
        }
    }

    fn collide_holding_unit_and_tiles(&mut self, unit: usize) {
        self.units[unit].base.on_ladder = false;

        let min_x = self.units[unit].holding_left() as usize;
        let max_x = self.units[unit].holding_right() as usize + 1;
        let min_y = self.units[unit].holding_bottom() as usize;
        let max_y = self.units[unit].holding_top() as usize + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Ladder => {
                        if !self.units[unit].base.on_ladder && can_use_ladder(&self.units[unit], x, y) {
                            self.units[unit].base.on_ladder = true;
                            self.units[unit].base.on_ground = true;
                            allow_jump(&mut self.units[unit], &self.properties);
                            break;
                        }
                    },
                    Tile::JumpPad => {
                        if !self.units[unit].base.on_ladder {
                            start_pad_jump(&mut self.units[unit], &self.properties);
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    fn collide_units_by_x(&mut self, unit: usize) {
        let (left, right) = self.units.split_at_mut(unit + 1);
        let (left_left, left_right) = left.split_at_mut(left.len() - 1);

        for i in 0 .. left_left.len() {
            if left_left[i].ignore() {
                continue;
            }
            left_right[0].collide_with_unit_by_x(&left_left[i]);
            if left_right[0].velocity_x == 0.0 {
                return;
            }
        }

        for i in 0 .. right.len() {
            if right[i].ignore() {
                continue;
            }
            left_right[0].collide_with_unit_by_x(&right[i]);
            if left_right[0].velocity_x == 0.0 {
                return;
            }
        }
    }

    fn collide_moving_down_unit_and_tiles_by_y(&mut self, unit: usize) {
        let min_x = self.units[unit].holding_left() as usize;
        let max_x = self.units[unit].holding_right() as usize + 1;
        let min_y = self.units[unit].moved_bottom() as usize;
        let max_y = self.units[unit].holding_center_y() as usize + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        if self.units[unit].collide_with_tile_by_y(x, y) {
                            self.units[unit].base.on_ground = true;
                            allow_jump(&mut self.units[unit], &self.properties);
                        }
                    },
                    Tile::Ladder => {
                        if self.units[unit].action.jump_down {
                            if can_use_ladder_moving(&self.units[unit], x, y) {
                                self.units[unit].base.on_ladder = true;
                                self.units[unit].base.on_ground = true;
                                allow_jump(&mut self.units[unit], &self.properties);
                            }
                        } else {
                            if !self.units[unit].base.on_ladder && cross_tile_border(&self.units[unit], y)
                                    && can_use_ladder_moving(&self.units[unit], x, y) && self.units[unit].collide_with_tile_by_y(x, y) {
                                self.units[unit].base.on_ground = true;
                                allow_jump(&mut self.units[unit], &self.properties);
                            }
                        }
                    },
                    Tile::Platform => {
                        if !self.units[unit].action.jump_down && cross_tile_border(&self.units[unit], y) {
                            if self.units[unit].collide_with_tile_by_y(x, y) {
                                self.units[unit].base.on_ground = true;
                                allow_jump(&mut self.units[unit], &self.properties);
                            }
                        }
                    },
                    _ => (),
                }
                if self.units[unit].velocity_y == 0.0 {
                    return;
                }
            }
        }
    }

    fn collide_moving_up_unit_and_tiles_by_y(&mut self, unit: usize) {
        let min_x = self.units[unit].holding_left() as usize;
        let max_x = self.units[unit].holding_right() as usize + 1;
        let min_y = self.units[unit].holding_center_y() as usize;
        let max_y = self.units[unit].moved_top() as usize + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        if self.units[unit].collide_with_tile_by_y(x, y) {
                            cancel_jump(&mut self.units[unit]);
                        }
                    },
                    _ => (),
                }
                if self.units[unit].velocity_y == 0.0 {
                    return;
                }
            }
        }
    }

    fn collide_units_by_y(&mut self, unit: usize) {
        let (left, right) = self.units.split_at_mut(unit + 1);
        let (left_left, left_right) = left.split_at_mut(left.len() - 1);

        for i in 0 .. left_left.len() {
            if left_left[i].ignore() {
                continue;
            }
            if left_right[0].collide_with_unit_by_y(&left_left[i]) {
                if left_right[0].base.position.y > left_left[i].base.position.y {
                    allow_jump(&mut left_right[0], &self.properties);
                } else {
                    cancel_jump(&mut left_right[0]);
                }
            }
            if left_right[0].velocity_y == 0.0 {
                return;
            }
        }

        for i in 0 .. right.len() {
            if right[i].ignore() {
                continue;
            }
            if left_right[0].collide_with_unit_by_y(&right[i]) {
                if left_right[0].base.position.y > right[i].base.position.y {
                    allow_jump(&mut left_right[0], &self.properties);
                } else {
                    cancel_jump(&mut left_right[0]);
                }
            }
            if left_right[0].velocity_y == 0.0 {
                return;
            }
        }
    }

    fn collide_bullet_and_units(&mut self, bullet: usize, time_interval: f64, explosions: &mut Vec<Explosion>, debug: &mut Option<&mut Dbg>) -> bool {
        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }
            if let Some(explosion) = collide_bullet_and_unit(self.properties.kill_score, time_interval,
                    &mut self.bullets[bullet], &mut self.units[unit], &mut self.players, debug) {
                explosions.push(explosion);
            }
            if self.bullets[bullet].hit {
                return true;
            }
        }
        false
    }

    fn collide_bulles_and_tiles(&mut self, bullet: usize, time_interval: f64, explosions: &mut Vec<Explosion>) -> bool {
        let moving_rect = self.bullets[bullet].moving_rect(time_interval);
        let min = moving_rect.min();
        let max = moving_rect.max();

        let min_x = min.x() as usize;
        let max_x = (max.x() as usize + 1).min(self.level.size_x());
        let min_y = min.y() as usize;
        let max_y = (max.y() as usize + 1).min(self.level.size_y());

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        collide_bullet_and_tile(x, y, time_interval, &mut self.bullets[bullet], explosions);
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

    fn collide_bullet_and_mines(&mut self, bullet: usize, time_interval: f64, explosions: &mut Vec<Explosion>) -> bool {
        for mine in 0 .. self.mines.len() {
            if self.mines[mine].ignore() {
                continue;
            }
            collide_bullet_and_mine(time_interval, &mut self.bullets[bullet], &mut self.mines[mine], explosions);
            if self.bullets[bullet].hit {
                return true;
            }
        }
        false
    }

    fn explode_units(&mut self, explosion: &Explosion) {
        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }
            explode_unit(&explosion, self.properties.kill_score, &mut self.units[unit], &mut self.players);
        }
    }

    fn explode_mines(&mut self, explosion: &Explosion, explosions: &mut Vec<Explosion>) {
        for mine in 0 .. self.mines.len() {
            if self.mines[mine].ignore() {
                continue;
            }
            explode_mine(&explosion, &mut self.mines[mine], explosions);
        }
    }

    #[cfg(feature = "verify_collisions")]
    fn verify_collisions(&self, unit: usize, place: &str) {
        let left = self.units[unit].holding_left();
        let right = self.units[unit].holding_right();
        let bottom = self.units[unit].holding_bottom();
        let top = self.units[unit].holding_top();

        for x in left as usize .. right as usize + 1 {
            for y in bottom as usize .. top as usize + 1 {
                assert!(self.level.get_tile(Location::new(x, y)) != Tile::Wall,
                    "{} x={} y={} left={} right={} bottom={} top={}", place, x, y, left, right, bottom, top);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnitExt {
    base: Unit,
    action: UnitAction,
    is_me: bool,
    is_teammate: bool,
    ignore: bool,
    velocity_x: f64,
    velocity_y: f64,
    player_index: usize,
}

impl UnitExt {
    pub fn new(base: Unit, is_me: bool, is_teammate: bool, player_index: usize) -> Self {
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
            velocity_x: 0.0,
            velocity_y: 0.0,
            player_index,
        }
    }

    pub fn base(&self) -> &Unit {
        &self.base
    }

    pub fn is_me(&self) -> bool {
        self.is_me
    }

    pub fn position(&self) -> Vec2 {
        Vec2::from_model(&self.base.position)
    }

    pub fn location(&self) -> Location {
        self.position().as_location()
    }

    pub fn action(&self) -> &UnitAction {
        &self.action
    }

    pub fn action_mut(&mut self) -> &mut UnitAction {
        &mut self.action
    }

    pub fn holding_right(&self) -> f64 {
        self.base.position.x + self.base.size.x / 2.0
    }

    pub fn holding_left(&self) -> f64 {
        self.base.position.x - self.base.size.x / 2.0
    }

    pub fn holding_top(&self) -> f64 {
        self.base.position.y + self.base.size.y
    }

    pub fn holding_bottom(&self) -> f64 {
        self.base.position.y
    }

    pub fn moved_right(&self) -> f64 {
        self.holding_right() + self.velocity_x
    }

    pub fn moved_left(&self) -> f64 {
        self.holding_left() + self.velocity_x
    }

    pub fn moved_top(&self) -> f64 {
        self.holding_top() + self.velocity_y
    }

    pub fn moved_bottom(&self) -> f64 {
        self.holding_bottom() + self.velocity_y
    }

    pub fn holding_half(&self) -> Vec2 {
        Vec2::new(self.holding_size_x(), self.holding_size_y()) / 2.0
    }

    pub fn holding_rect(&self) -> Rect {
        Rect::new(
            Vec2::new(self.holding_center_x(), self.holding_center_y()),
            self.holding_half()
        )
    }

    pub fn health(&self) -> i32 {
        self.base.health
    }

    pub fn is_teammate(&self) -> bool {
        self.is_teammate
    }

    pub fn start_move_by_x(&mut self, value: f64) {
        self.velocity_x = value;
    }

    pub fn start_move_by_y(&mut self, value: f64) {
        self.velocity_y = value;
    }

    pub fn holding_size_x(&self) -> f64 {
        self.base.size.x
    }

    pub fn holding_size_y(&self) -> f64 {
        self.base.size.y
    }

    pub fn moving_size_x(&self) -> f64 {
        self.base.size.x + self.velocity_x.abs()
    }

    pub fn moving_size_y(&self) -> f64 {
        self.base.size.y + self.velocity_y.abs()
    }

    pub fn holding_center_x(&self) -> f64 {
        self.base.position.x
    }

    pub fn holding_center_y(&self) -> f64 {
        self.base.position.y + self.base.size.y / 2.0
    }

    pub fn moving_center_x(&self) -> f64 {
        self.base.position.x + self.velocity_x / 2.0
    }

    pub fn moving_center_y(&self) -> f64 {
        self.base.position.y + (self.velocity_y + self.base.size.y) / 2.0
    }

    pub fn collide_with_tile_by_x(&mut self, x: usize, y: usize) {
        if self.velocity_x == 0.0 {
            return;
        }

        let left = self.moved_left();
        let right = self.moved_right();
        let bottom = self.holding_bottom();
        let top = self.holding_top();

        if left >= (x + 1) as f64 || right <= x as f64 || bottom >= (y + 1) as f64 || top <= y as f64 {
            return;
        }

        if left > x as f64 && left < (x + 1) as f64 {
            self.velocity_x = (self.velocity_x - left + (x + 1) as f64).max(0.0);
        } else {
            self.velocity_x = (self.velocity_x - right + x as f64).min(0.0);
        }
    }

    pub fn collide_with_tile_by_y(&mut self, x: usize, y: usize) -> bool {
        if self.velocity_y == 0.0 {
            return false;
        }

        let left = self.holding_left();
        let right = self.holding_right();
        let bottom = self.moved_bottom();
        let top = self.moved_top();

        if left >= (x + 1) as f64 || right <= x as f64 || bottom >= (y + 1) as f64 || top <= y as f64 {
            return false;
        }

        if bottom > y as f64 && bottom < (y + 1) as f64 {
            self.velocity_y = (self.velocity_y - bottom + (y + 1) as f64).max(0.0);
        } else {
            self.velocity_y = (self.velocity_y - top + y as f64).min(0.0);
        }

        true
    }

    pub fn collide_with_unit_by_x(&mut self, other: &UnitExt) {
        if self.velocity_x == 0.0 {
            return;
        }

        let left = self.moved_left();
        let right = self.moved_right();
        let bottom = self.holding_bottom();
        let top = self.holding_top();

        let other_left = other.holding_left();
        let other_right = other.holding_right();
        let other_bottom = other.holding_bottom();
        let other_top = other.holding_top();

        if left >= other_right || right <= other_left || bottom >= other_top || top <= other_bottom {
            return;
        }

        if left > other_left && left < other_right {
            self.velocity_x = (self.velocity_x - left + other_right).max(0.0);
        } else {
            self.velocity_x = (self.velocity_x - right + other_left).min(0.0);
        }
    }

    pub fn collide_with_unit_by_y(&mut self, other: &UnitExt) -> bool {
        if self.velocity_y == 0.0 {
            return false;
        }

        let left = self.holding_left();
        let right = self.holding_right();
        let bottom = self.moved_bottom();
        let top = self.moved_top();

        let other_left = other.holding_left();
        let other_right = other.holding_right();
        let other_bottom = other.holding_bottom();
        let other_top = other.holding_top();

        if left >= other_right || right <= other_left || bottom >= other_top || top <= other_bottom {
            return false;
        }

        if bottom > other_bottom && bottom < other_top {
            self.velocity_y = (self.velocity_y - bottom + other_top).max(0.0);
        } else {
            self.velocity_y = (self.velocity_y - top + other_bottom).min(0.0);
        }

        true
    }

    pub fn finish_move_by_x(&mut self) {
        self.base.position.x += self.velocity_x;
    }

    pub fn finish_move_by_y(&mut self) {
        self.base.position.y += self.velocity_y;
    }

    pub fn ignore(&self) -> bool {
        self.ignore || self.base.health <= 0
    }

    pub fn weapon(&self) -> &Option<Weapon> {
        &self.base.weapon
    }

    pub fn damage(&mut self, value: i32) -> i32 {
        let health = self.base.health;
        self.base.health = (self.base.health - value).max(0);
        health - self.base.health
    }

    pub fn heal(&mut self, value: i32, properties: &Properties) {
        self.base.health = (self.base.health + value).min(properties.unit_max_health);
    }

    pub fn mines(&self) -> i32 {
        self.base.mines
    }

    pub fn upper_center(&self) -> Vec2 {
        Vec2::new(self.holding_center_x(), self.holding_top() - self.base.size.x / 2.0)
    }

    pub fn lower_center(&self) -> Vec2 {
        Vec2::new(self.holding_center_x(), self.holding_bottom() + self.base.size.x / 2.0)
    }

    pub fn reset_velocity(&mut self) {
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
    }

    pub fn moving_back_rect(&self) -> Rect {
        Rect::new(
            Vec2::new(
                self.base.position.x - self.velocity_x / 2.0,
                self.base.position.y + (self.base.size.y - self.velocity_y) / 2.0
            ),
            Vec2::new(self.moving_size_x(), self.moving_size_y()) / 2.0
        )
    }

    pub fn velocity(&self) -> Vec2 {
        Vec2::new(self.velocity_x, self.velocity_y)
    }

    pub fn holding_center(&self) -> Vec2 {
        Vec2::new(self.holding_center_x(), self.holding_center_y())
    }

    pub fn rect_back_at(&self, time: f64) -> Rect {
        Rect::new(self.holding_center() - self.velocity() * time, self.holding_half())
    }
}

#[derive(Clone, Debug)]
pub struct BulletExt {
    base: Bullet,
    hit: bool,
    player_index: usize,
}

impl BulletExt {
    pub fn new(base: Bullet, player_index: usize) -> Self {
        Self { base, hit: false, player_index }
    }

    pub fn base(&self) -> &Bullet {
        &self.base
    }

    pub fn half_size(&self) -> f64 {
        self.base.size / 2.0
    }

    pub fn center(&self) -> Vec2 {
        Vec2::from_model(&self.base.position)
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

    pub fn velocity(&self) -> Vec2 {
        Vec2::from_model(&self.base.velocity)
    }

    pub fn moving_rect(&self, time_interval: f64) -> Rect {
        let shift = self.velocity() * time_interval / 2.0;
        let half = self.half() + shift.abs();
        Rect::new(self.center() + shift, half)
    }

    pub fn rect_at(&self, time: f64) -> Rect {
        Rect::new(self.center() + self.velocity() * time, self.half())
    }

    pub fn center_at(&self, time: f64) -> Vec2 {
        self.center() + self.velocity() * time
    }
}

#[derive(Clone, Debug)]
pub struct Explosion {
    params: ExplosionParams,
    position: Vec2,
    player_index: usize,
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
    location: Location,
    rect: Rect,
}

impl LootBoxExt {
    pub fn new(base: LootBox) -> Self {
        Self {
            location: Location::from_model(&base.position),
            rect: Rect::new(
                Vec2::from_model(&base.position) + Vec2::only_y(base.size.y / 2.0),
                Vec2::from_model(&base.size) / 2.0
            ),
            base,
            used: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MineExt {
    base: Mine,
    player_index: usize,
}

impl MineExt {
    pub fn new(base: Mine, player_index: usize) -> Self {
        Self { base, player_index }
    }

    pub fn base(&self) -> &Mine {
        &self.base
    }

    pub fn center(&self) -> Vec2 {
        Vec2::from_model(&self.base.position) + Vec2::only_y(self.base.size.y / 2.0)
    }

    pub fn trigger_rect(&self) -> Rect {
        Rect::new(self.center(), Vec2::new(self.base.trigger_radius, self.base.trigger_radius))
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.center(), Vec2::from_model(&self.base.size) / 2.0)
    }

    pub fn ignore(&self) -> bool {
        self.base.state == MineState::Exploded
    }
}

pub fn can_use_ladder(unit: &UnitExt, x: usize, y: usize) -> bool {
    unit.holding_center_x() as usize >= x && unit.holding_center_x() <= (x + 1) as f64
    && (
        (unit.holding_bottom() as usize >= y && (unit.holding_bottom() <= (y + 1) as f64))
        || (unit.holding_center_y() as usize >= y && (unit.holding_center_y() <= (y + 1) as f64))
    )
}

pub fn can_use_ladder_moving(unit: &UnitExt, x: usize, y: usize) -> bool {
    unit.moving_center_x() as usize >= x && unit.moving_center_x() <= (x + 1) as f64
    && (
        (unit.moved_bottom() as usize >= y && (unit.moved_bottom() <= (y + 1) as f64))
        || (unit.moving_center_y() as usize >= y && (unit.moving_center_y() <= (y + 1) as f64))
    )
}

pub fn cross_tile_border(unit: &UnitExt, y: usize) -> bool {
    unit.holding_bottom() as usize >= y + 1 && unit.moved_bottom() < (y + 1) as f64
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

fn collide_bullet_and_unit(kill_score: i32, time_interval: f64, bullet: &mut BulletExt, unit: &mut UnitExt,
        players: &mut Vec<Player>, debug: &mut Option<&mut Dbg>) -> Option<Explosion> {
    if bullet.base.unit_id == unit.base.id || !bullet.moving_rect(time_interval).has_collision(&unit.moving_back_rect()) {
        return None;
    }

    let get_nearest_distance = |time| {
        let bullet_center = bullet.center() + bullet.velocity() * time;
        let unit_shift = unit.velocity() * (time_interval - time);
        let distance_to_upper_center = bullet_center.distance(unit.upper_center() - unit_shift);
        let distance_to_lower_center = bullet_center.distance(unit.lower_center() - unit_shift);
        distance_to_upper_center.min(distance_to_lower_center)
    };

    let nearest_time = minimize1d(0.0, time_interval, 10, get_nearest_distance);

    if !bullet.rect_at(nearest_time).has_collision(&unit.rect_back_at(time_interval - nearest_time)) {
        return None;
    }

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
    {
        if let Some(d) = debug {
            d.draw(bullet.rect_at(nearest_time).as_debug(ColorF32 { a: 0.8, r: 0.8, g: 0.4, b: 0.2 }));
        }
    }

    bullet.hit = true;
    let score = unit.damage(bullet.base.damage);
    if score > 0 && bullet.player_index != unit.player_index {
        players[bullet.player_index].score += score;
    }
    if unit.base.health == 0 {
        players[(unit.player_index + 1) % 2].score += kill_score;
    }
    bullet.explosion_params().as_ref()
        .map(|v| {
            let has_collision = |time| bullet.rect_at(time).has_collision(&unit.rect_back_at(time_interval - time));
            let hit_time = root_search(0.0, nearest_time, 10, has_collision);

            Explosion { params: v.clone(), position: bullet.center_at(hit_time), player_index: bullet.player_index }
        })
}

fn explode_unit(explosion: &Explosion, kill_score: i32, unit: &mut UnitExt, players: &mut Vec<Player>) {
    if !explosion.rect().has_collision(&unit.holding_rect()) {
        return;
    }
    let score = unit.damage(explosion.params.damage);
    if score > 0 && explosion.player_index != unit.player_index {
        players[explosion.player_index].score += score;
    }
    if unit.base.health == 0 {
        players[(unit.player_index + 1) % 2].score += kill_score;
    }
}

fn explode_mine(explosion: &Explosion, mine: &mut MineExt, explosions: &mut Vec<Explosion>) {
    if !explosion.rect().has_collision(&mine.rect()) {
        return;
    }
    mine.base.state = MineState::Exploded;
    explosions.push(Explosion { params: mine.base.explosion_params.clone(), position: mine.center(), player_index: mine.player_index })
}

fn collide_bullet_and_tile(x: usize, y: usize, time_interval: f64, bullet: &mut BulletExt, explosions: &mut Vec<Explosion>) {
    let tile_rect = make_tile_rect(x, y);
    if !bullet.moving_rect(time_interval).has_collision(&tile_rect) {
        return;
    }
    let get_nearest_distance = |time| {
        tile_rect.center().distance(bullet.center() + bullet.velocity() * time)
    };
    let nearest_time = minimize1d(0.0, time_interval, 10, get_nearest_distance);
    if !bullet.rect_at(nearest_time).has_collision(&tile_rect) {
        return;
    }
    bullet.hit = true;
    if let Some(v) = bullet.base.explosion_params.as_ref() {
        explosions.push(Explosion {params: v.clone(), position: bullet.center(), player_index: bullet.player_index});
    }
}

fn collide_bullet_and_mine(time_interval: f64, bullet: &mut BulletExt, mine: &mut MineExt, explosions: &mut Vec<Explosion>) {
    let mine_rect = mine.rect();
    if !bullet.moving_rect(time_interval).has_collision(&mine_rect) {
        return;
    }
    let get_nearest_distance = |time| {
        mine_rect.center().distance(bullet.center() + bullet.velocity() * time)
    };
    let nearest_time = minimize1d(0.0, time_interval, 10, get_nearest_distance);
    if !bullet.rect_at(nearest_time).has_collision(&mine_rect) {
        return;
    }
    bullet.hit = true;
    explosions.push(Explosion { params: mine.base.explosion_params.clone(), position: mine.center(), player_index: mine.player_index });
    mine.base.state = MineState::Exploded;
    if let Some(v) = bullet.base.explosion_params.as_ref() {
        explosions.push(Explosion {params: v.clone(), position: bullet.center(), player_index: bullet.player_index});
    }
}

fn pickup(properties: &Properties, loot_box: &mut LootBoxExt, unit: &mut UnitExt) -> bool {
    if loot_box.location != unit.location() {
        return false;
    }
    if !loot_box.rect.has_collision(&unit.holding_rect()) {
        return false;
    }
    match &mut loot_box.base.item {
        Item::HealthPack {health} => {
            if unit.health() >= properties.unit_max_health {
                false
            } else {
                unit.heal(*health, properties);
                loot_box.used = true;
                true
            }
        },
        #[cfg(feature = "simulator_pickup_weapon")]
        Item::Weapon {weapon_type} => {
            if unit.weapon().is_none() {
                unit.base.weapon = Some(make_weapon(weapon_type.clone(), properties));
                loot_box.used = true;
                true
            } else if unit.action.swap_weapon {
                *weapon_type = unit.base.weapon.as_ref().unwrap().typ.clone();
                unit.base.weapon = Some(make_weapon(weapon_type.clone(), properties));
                false
            } else {
                false
            }
        },
        #[cfg(feature = "simulator_pickup_mine")]
        Item::Mine {} => {
            unit.base.mines += 1;
            loot_box.used = true;
            true
        },
        #[cfg(any(not(feature = "simulator_pickup_weapon"), not(feature = "simulator_pickup_mine")))]
        _ => false,
    }
}

fn activate(properties: &Properties, mine: &mut MineExt, unit: &mut UnitExt) -> bool {
    if mine.base.state != MineState::Idle {
        return false;
    }
    if !mine.trigger_rect().has_collision(&unit.holding_rect()) {
        return false;
    }
    mine.base.state = MineState::Triggered;
    mine.base.timer = Some(properties.mine_trigger_time);
    true
}

fn update_mine(time_interval: f64, mine: &mut MineExt) -> Option<Explosion> {
    if let Some(timer) = mine.base.timer.as_mut() {
        *timer -= time_interval;
        if *timer <= 0.0 {
            mine.base.timer = None;
        }
    }
    if mine.base.timer.is_none() {
        match mine.base.state {
            MineState::Preparing => {
                mine.base.state = MineState::Idle;
                None
            },
            MineState::Triggered => {
                mine.base.state = MineState::Exploded;
                Some(Explosion {params: mine.base.explosion_params.clone(), position: mine.center(), player_index: mine.player_index})
            },
            _ => None,
        }
    } else {
        None
    }
}

fn make_tile_rect(x: usize, y: usize) -> Rect {
    Rect::new(Vec2::new(x as f64 + 0.5, y as f64 + 0.5), Vec2::new(0.5, 0.5))
}

pub fn make_weapon(weapon_type: WeaponType, properties: &Properties) -> Weapon {
    let params = &properties.weapon_params[&weapon_type];
    Weapon {
        params: params.clone(),
        typ: weapon_type,
        magazine: params.magazine_size,
        was_shooting: false,
        spread: params.max_spread,
        fire_timer: None,
        last_angle: None,
        last_fire_tick: None,
    }
}

#[cfg(all(feature = "simulator_weapon", feature = "simulator_shoot"))]
fn shoot(unit: &mut UnitExt) -> Option<BulletExt> {
    if unit.base.weapon.is_none() {
        return None;
    }

    if unit.base.weapon.as_ref().unwrap().fire_timer.is_some() {
        return None;
    }

    let walk_direction = if unit.base.walked_right {
        Vec2::i()
    } else {
        -Vec2::i()
    };

    let weapon = unit.base.weapon.as_ref().unwrap();

    let base = Bullet {
        weapon_type: weapon.typ.clone(),
        unit_id: unit.base.id,
        player_id: unit.base.player_id,
        position: unit.holding_center().as_model(),
        velocity: (walk_direction.rotated(weapon.last_angle.unwrap_or(0.0)) * weapon.params.bullet.speed).as_model(),
        damage: weapon.params.bullet.damage,
        size: weapon.params.bullet.size,
        explosion_params: weapon.params.explosion.clone(),
    };

    let weapon_mut = unit.base.weapon.as_mut().unwrap();

    weapon_mut.magazine -= 1;

    if weapon_mut.magazine == 0 {
        weapon_mut.fire_timer = Some(weapon_mut.params.reload_time);
        weapon_mut.magazine = weapon_mut.params.magazine_size;
    } else {
        weapon_mut.fire_timer = Some(weapon_mut.params.fire_rate);
    }

    Some(BulletExt::new(base, unit.player_index))
}

fn is_loot_box_enabled(loot_box: &LootBox) -> bool {
    match &loot_box.item {
        Item::HealthPack { .. } => true,
        #[cfg(feature = "simulator_pickup_weapon")]
        Item::Weapon { .. } => true,
        #[cfg(feature = "simulator_pickup_mine")]
        Item::Mine { } => true,
        #[cfg(any(not(feature = "simulator_pickup_weapon"), not(feature = "simulator_pickup_mine")))]
        _ => false,
    }
}
