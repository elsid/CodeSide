use crate::my_strategy::{
    Debug,
};

pub struct MyStrategyImpl {}

impl MyStrategyImpl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_action(&mut self, unit: &model::Unit, game: &model::Game, debug: &mut Debug) -> model::UnitAction {
        if game.current_tick == 0 {
            println!("{}", rustc_serialize::json::encode(&game.properties).unwrap());
            std::process::exit(0);
        }
        model::UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: model::Vec2F64 { x: 0.0, y: 0.0 },
            shoot: false,
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }
}
