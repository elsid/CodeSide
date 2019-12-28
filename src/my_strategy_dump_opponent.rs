use model::{
    Game,
    Unit,
    UnitAction,
    Vec2F64,
};

use crate::my_strategy::{
    Debug,
};

pub struct MyStrategyImpl {
    last_tick: i32,
}

impl MyStrategyImpl {
    pub fn new() -> Self {
        Self {
            last_tick: -1,
        }
    }

    pub fn get_action(&mut self, unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
            for opponent in game.units.iter().filter(|v| v.player_id != unit.player_id) {
                println!("[{}] opponent: {:?}", game.current_tick, opponent);
            }
        }
        UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: Vec2F64 {
                x: 0.0,
                y: 0.0
            },
            shoot: false,
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }
}
