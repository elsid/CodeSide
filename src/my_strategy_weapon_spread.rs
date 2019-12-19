use model::{
    Game,
    Unit,
    UnitAction,
    Vec2F64,
    WeaponType,
};

use crate::Debug;

use crate::my_strategy::{
    Vec2,
};

pub struct MyStrategyImpl {
    last_tick: i32,
    angle: f64,
    rotate_speed: f64,
}

impl MyStrategyImpl {
    pub fn new() -> Self {
        Self {
            last_tick: -1,
            angle: 0.0,
            rotate_speed: 0.016,
        }
    }

    pub fn get_action(&mut self, me: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
        }
        if me.weapon.is_none() || me.weapon.as_ref().unwrap().typ != WeaponType::Pistol {
            return UnitAction {
                velocity: -game.properties.unit_max_horizontal_speed,
                jump: false,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: true,
                plant_mine: false,
            };
        }
        println!("{} {}", self.rotate_speed, me.weapon.as_ref().unwrap().spread);
        let aim = Vec2::i().rotated(self.angle);
        self.rotate_speed *= 1.0001;
        if self.angle >= 2.0 * std::f64::consts::PI {
            self.angle -= 2.0 * std::f64::consts::PI;
        }
        self.angle += self.rotate_speed;
        UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: aim.as_model(),
            shoot: false,
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }
}
