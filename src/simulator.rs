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
    remove_if,
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
    counters: Counters,
}

#[derive(Debug, Clone, Default)]
pub struct Counters {
    pub max_number_of_mines: usize,
    pub triggered_mines_by_me: usize,
}

const EPSILON: f64 = 1e-9;

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
            counters: Counters::default(),
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

    pub fn counters(&self) -> &Counters {
        &self.counters
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
        self.counters.max_number_of_mines = self.counters.max_number_of_mines.max(self.mines.len());
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
                    let rect = self.units[unit].base.rect();
                    let color = ColorF32 { a: 0.01, r: 0.8, g: 0.8, b: 0.8 };
                    d.draw(CustomData::Line {
                        p1: rect.bottom_left().as_debug(),
                        p2: rect.top_left().as_debug(),
                        width: 0.1,
                        color: color.clone(),
                    });
                    d.draw(CustomData::Line {
                        p1: rect.top_left().as_debug(),
                        p2: rect.top_right().as_debug(),
                        width: 0.1,
                        color: color.clone(),
                    });
                    d.draw(CustomData::Line {
                        p1: rect.top_right().as_debug(),
                        p2: rect.bottom_right().as_debug(),
                        width: 0.1,
                        color: color.clone(),
                    });
                    d.draw(CustomData::Line {
                        p1: rect.bottom_right().as_debug(),
                        p2: rect.bottom_left().as_debug(),
                        width: 0.1,
                        color: color.clone(),
                    });
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
            if self.collide_bulles_and_tiles(bullet) {
                continue
            }

            #[cfg(all(feature = "enable_debug", feature = "enable_debug_simulator"))]
            {
                if let Some(d) = debug {
                    d.draw(self.bullets[bullet].rect().as_debug(ColorF32 { a: 0.01, r: 0.8, g: 0.8, b: 0.8 }));
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
            if self.mines[mine].base.state == MineState::Exploded {
                continue;
            }
            if let Some(explosion) = update_mine(time_interval, &mut self.mines[mine]) {
                for unit in 0 .. self.units.len() {
                    if self.units[unit].ignore() {
                        continue;
                    }
                    explode(&explosion, self.properties.kill_score, &mut self.units[unit], &mut self.players);
                }
            } else if self.mines[mine].base.state == MineState::Idle {
                for unit in 0 .. self.units.len() {
                    if self.units[unit].ignore() {
                        continue;
                    }
                    if activate(&self.properties, &mut self.mines[mine], &mut self.units[unit]) {
                        self.counters.triggered_mines_by_me += self.units[unit].is_me as usize;
                        break;
                    }
                }
            }
        }

        self.current_micro_tick += 1;
    }

    fn collide_moving_unit_and_tiles_by_x(&mut self, unit: usize) {
        let min_x = self.units[unit].moving_left().max(0.0) as usize;
        let max_x = (self.units[unit].moving_right() as usize + 1).min(self.level.size_x());
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
        let min_y = self.units[unit].moving_bottom().max(0.0) as usize;
        let max_y = (self.units[unit].holding_center_y() as usize + 1).min(self.level.size_y());

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
                        if !self.units[unit].base.on_ladder && !self.units[unit].action.jump_down {
                            if can_use_ladder(&self.units[unit], x, y) && self.units[unit].collide_with_tile_by_y(x, y) {
                                self.units[unit].base.on_ladder = true;
                                self.units[unit].base.on_ground = true;
                                allow_jump(&mut self.units[unit], &self.properties);
                            }
                        }
                    },
                    Tile::Platform => {
                        if !self.units[unit].action.jump_down && cross_platform(&self.units[unit], y) {
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
        let min_y = self.units[unit].holding_center_y().max(0.0) as usize;
        let max_y = (self.units[unit].moving_top() as usize + 1).min(self.level.size_y());

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
            left_right[0].collide_with_unit_by_y(&left_left[i]);
            if left_right[0].velocity_y == 0.0 {
                return;
            }
        }

        for i in 0 .. right.len() {
            if right[i].ignore() {
                continue;
            }
            left_right[0].collide_with_unit_by_y(&right[i]);
            if left_right[0].velocity_y == 0.0 {
                return;
            }
        }
    }

    fn collide_bullet_and_units(&mut self, bullet: usize) -> bool {
        for unit in 0 .. self.units.len() {
            if self.units[unit].ignore() {
                continue;
            }
            if let Some(explosion) = collide_unit_and_bullet(self.properties.kill_score, &mut self.bullets[bullet], &mut self.units[unit], &mut self.players) {
                for i in 0 .. self.units.len() {
                    if self.units[i].ignore() {
                        continue;
                    }
                    explode(&explosion, self.properties.kill_score, &mut self.units[i], &mut self.players);
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
        let max_x = (self.bullets[bullet].right() as usize + 1).min(self.level.size_x());
        let min_y = self.bullets[bullet].bottom() as usize;
        let max_y = (self.bullets[bullet].top() as usize + 1).min(self.level.size_y());

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        if let Some(explosion) = collide_unit_and_tile(x, y, &mut self.bullets[bullet]) {
                            for unit in 0 .. self.units.len() {
                                if self.units[unit].ignore() {
                                    continue;
                                }
                                explode(&explosion, self.properties.kill_score, &mut self.units[unit], &mut self.players);
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

    #[cfg(feature = "verify_collisions")]
    fn verify_collisions(&self, unit: usize, place: &str) {
        assert_eq!(self.units[unit].velocity_x, 0.0);
        assert_eq!(self.units[unit].velocity_y, 0.0);

        for x in 0 .. self.level.size_x() {
            for y in 0 .. self.level.size_y() {
                match self.level.get_tile(Location::new(x, y)) {
                    Tile::Wall => {
                        let tile_size = 1.0;
                        let tile_y = y as f64 + 0.5;
                        let half_size_sum_y = (self.units[unit].holding_size_y() + tile_size) / 2.0;
                        let distance_by_y = (self.units[unit].holding_center_y() - tile_y).abs();
                        let penetration_by_y = half_size_sum_y - distance_by_y;
                        let tile_x = x as f64 + 0.5;
                        let half_size_sum_x = (self.units[unit].holding_size_x() + tile_size) / 2.0;
                        let distance_by_x = (self.units[unit].holding_center_x() - tile_x).abs();
                        let penetration_by_x = half_size_sum_x - distance_by_x;
                        assert!(
                            penetration_by_x <= EPSILON || penetration_by_y <= EPSILON,
                            "\n[{}] {} x={} y={} penetration_by_x={} penetration_by_y={}\n{:?}\n", self.units[unit].base.id, place, x, y, penetration_by_x, penetration_by_y, self.units[unit]
                        );
                    },
                    _ => (),
                }
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

    pub fn moving_right(&self) -> f64 {
        self.moving_center_x() + self.moving_size_x() / 2.0
    }

    pub fn moving_left(&self) -> f64 {
        self.moving_center_x() - self.moving_size_x() / 2.0
    }

    pub fn moving_top(&self) -> f64 {
        self.moving_center_y() + self.moving_size_y() / 2.0
    }

    pub fn moving_bottom(&self) -> f64 {
        self.moving_center_y() - self.moving_size_y() / 2.0
    }

    pub fn holding_rect(&self) -> Rect {
        Rect::new(
            Vec2::new(self.holding_center_x(), self.holding_center_y()),
            Vec2::new(self.holding_size_x(), self.holding_size_y()) / 2.0
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
        let tile_size = 1.0;
        let tile_y = y as f64 + 0.5;
        let half_size_sum_y = (self.holding_size_y() + tile_size) / 2.0;
        let distance_by_y = (self.holding_center_y() - tile_y).abs();
        let penetration_by_y = half_size_sum_y - distance_by_y;
        if penetration_by_y <= EPSILON {
            return;
        }
        let tile_x = x as f64 + 0.5;
        let half_size_sum_x = (self.moving_size_x() + tile_size) / 2.0;
        let distance_by_x = (self.moving_center_x() - tile_x).abs();
        let penetration_by_x = half_size_sum_x - distance_by_x;
        if penetration_by_x <= 0.0 {
            return;
        }
        self.velocity_x -= (penetration_by_x + EPSILON).copysign(self.velocity_x);
    }

    pub fn collide_with_tile_by_y(&mut self, x: usize, y: usize) -> bool {
        if self.velocity_y == 0.0 {
            return false;
        }
        let tile_size = 1.0;
        let tile_x = x as f64 + 0.5;
        let half_size_sum_x = (self.holding_size_x() + tile_size) / 2.0;
        let distance_by_x = (self.holding_center_x() - tile_x).abs();
        let penetration_by_x = half_size_sum_x - distance_by_x;
        if penetration_by_x <= EPSILON {
            return false;
        }
        let tile_y = y as f64 + 0.5;
        let half_size_sum_y = (self.moving_size_y() + tile_size) / 2.0;
        let distance_by_y = (self.moving_center_y() - tile_y).abs();
        let penetration_by_y = half_size_sum_y - distance_by_y;
        if penetration_by_y <= 0.0 {
            return false;
        }
        self.velocity_y -= (penetration_by_y + EPSILON).copysign(self.velocity_y);
        true
    }

    pub fn collide_with_unit_by_x(&mut self, other: &UnitExt) {
        let half_size_sum_y = (self.holding_size_y() + other.holding_size_y()) / 2.0;
        let distance_by_y = (self.holding_center_y() - other.holding_center_y()).abs();
        let penetration_by_y = half_size_sum_y - distance_by_y;
        if penetration_by_y <= EPSILON {
            return;
        }
        let half_size_sum_x = (self.moving_size_x() + other.holding_size_x()) / 2.0;
        let distance_by_x = (self.moving_center_x() - other.holding_center_x()).abs();
        let penetration_by_x = half_size_sum_x - distance_by_x;
        if penetration_by_x <= 0.0 {
            return;
        }
        self.velocity_x -= (penetration_by_x + EPSILON).copysign(self.velocity_x);
    }

    pub fn collide_with_unit_by_y(&mut self, other: &UnitExt) {
        let half_size_sum_x = (self.holding_size_x() + other.holding_size_x()) / 2.0;
        let distance_by_x = (self.holding_center_x() - other.holding_center_x()).abs();
        let penetration_by_x = half_size_sum_x - distance_by_x;
        if penetration_by_x <= EPSILON {
            return;
        }
        let half_size_sum_y = (self.moving_size_y() + other.holding_size_y()) / 2.0;
        let distance_by_y = (self.moving_center_y() - other.holding_center_y()).abs();
        let penetration_by_y = half_size_sum_y - distance_by_y;
        if penetration_by_y <= 0.0 {
            return;
        }
        self.velocity_y -= (penetration_by_y + EPSILON).copysign(self.velocity_y);
    }

    pub fn finish_move_by_x(&mut self) {
        self.base.position.x += self.velocity_x;
        self.velocity_x = 0.0;
    }

    pub fn finish_move_by_y(&mut self) {
        self.base.position.y += self.velocity_y;
        self.velocity_y = 0.0;
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

    pub fn half_size(&self) -> f64 {
        self.base.size * 0.5
    }

    pub fn center(&self) -> Vec2 {
        Vec2::from_model(&self.base.position)
    }

    pub fn rect(&self) -> Rect {
        let half_size = self.base.size * 0.5;
        Rect::new(self.center(), Vec2::new(half_size, half_size))
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
}

impl LootBoxExt {
    pub fn new(base: LootBox) -> Self {
        Self { base, used: false }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            Vec2::from_model(&self.base.position) + Vec2::only_y(self.base.size.y / 2.0),
            Vec2::from_model(&self.base.size) / 2.0
        )
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

    pub fn rect(&self, is_me: bool) -> Rect {
        let size = if self.base.state == MineState::Idle {
            let factor = 2.0 * is_me as i32 as f64;
            Vec2::new(self.base.trigger_radius, self.base.trigger_radius) * factor
        } else {
            Vec2::from_model(&self.base.size) / 2.0
        };
        Rect::new(self.center(), size)
    }
}

pub fn can_use_ladder(unit: &UnitExt, x: usize, y: usize) -> bool {
    unit.holding_center_x() as usize >= x && unit.holding_center_x() <= (x + 1) as f64
    && (
        (unit.holding_bottom() as usize >= y && (unit.holding_bottom() <= (y + 1) as f64))
        || (unit.holding_center_y() as usize >= y && (unit.holding_center_y() <= (y + 1) as f64))
    )
}

pub fn cross_platform(unit: &UnitExt, y: usize) -> bool {
    unit.holding_bottom() as usize >= y + 1 && unit.moving_bottom() < (y + 1) as f64
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

fn collide_unit_and_bullet(kill_score: i32, bullet: &mut BulletExt, unit: &mut UnitExt, players: &mut Vec<Player>) -> Option<Explosion> {
    if bullet.base.unit_id == unit.base.id || !bullet.rect().has_collision(&unit.holding_rect()) {
        return None;
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
        .map(|v| Explosion {params: v.clone(), position: bullet.center(), player_index: bullet.player_index})
}

fn explode(explosion: &Explosion, kill_score: i32, unit: &mut UnitExt, players: &mut Vec<Player>) {
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

fn collide_unit_and_tile(x: usize, y: usize, bullet: &mut BulletExt) -> Option<Explosion> {
    if !make_tile_rect(x, y).has_collision(&bullet.rect()) {
        return None;
    }
    bullet.hit = true;
    bullet.explosion_params().as_ref()
        .map(|v| Explosion {params: v.clone(), position: bullet.center(), player_index: bullet.player_index})
}

fn pickup(properties: &Properties, loot_box: &mut LootBoxExt, unit: &mut UnitExt) -> bool {
    if !loot_box.rect().has_collision(&unit.holding_rect()) {
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

fn activate(properties: &Properties, mine: &mut MineExt, unit: &mut UnitExt) -> bool {
    if !mine.rect(unit.is_me).has_collision(&unit.holding_rect()) {
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
