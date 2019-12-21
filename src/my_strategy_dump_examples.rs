use crate::my_strategy::{
    Debug,
};

pub struct MyStrategyImpl {
    has_loot_box: bool,
    has_bullet: bool,
    has_mine: bool,
    has_weapon: bool,
}

impl MyStrategyImpl {
    pub fn new() -> Self {
        Self {
            has_loot_box: false,
            has_bullet: false,
            has_mine: false,
            has_weapon: false,
        }
    }

    pub fn get_action(&mut self, unit: &model::Unit, game: &model::Game, debug: &mut Debug) -> model::UnitAction {
        if game.current_tick == 0 {
            println!("fn player() {{ {:?}; }}", game.players[0]);
            println!("fn unit0() {{ {:?}; }}", game.units[0]);
            println!("fn unit1() {{ {:?}; }}", game.units[1]);
            println!("// fn properties() {{ {:?}; }}", game.properties);
            println!("fn properties_without_weapon_params() {{ {:?}; }}", get_properties_without_weapon_params(game.properties.clone()));
            println!("// fn game() {{ {:?}; }}", get_game_without_repeatable(game.clone()));
            println!("fn level() {{ {:?}; }}", game.level);
            println!("fn weapon_params() {{ {:?}; }}", game.properties.weapon_params.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>());
            println!("fn loot_boxes() {{ {:?}; }}", game.loot_boxes);
        }
        if !self.has_loot_box && game.loot_boxes.len() > 0 {
            self.has_loot_box = true;
            println!("fn loot_box() {{ {:?}; }}", game.loot_boxes[0]);
        }
        if !self.has_bullet && game.bullets.len() > 0 {
            self.has_bullet = true;
            println!("fn bullet() {{ {:?}; }}", game.bullets[0]);
        }
        if !self.has_mine && game.mines.len() > 0 {
            self.has_mine = true;
            println!("fn mine() {{ {:?}; }}", game.mines[0]);
        }
        let opponent = game.units.iter().find(|v| v.id != unit.id).unwrap();
        if !self.has_weapon && opponent.weapon.is_some() {
            self.has_weapon = true;
            println!("fn weapon() {{ {:?}; }}", opponent.weapon.as_ref().unwrap());
        }
        if self.has_loot_box && self.has_bullet && self.has_mine && self.has_weapon {
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

fn get_properties_without_weapon_params(mut properties: model::Properties) -> model::Properties {
    properties.weapon_params.clear();
    properties
}

fn get_game_without_repeatable(mut game: model::Game) -> model::Game {
    game.level.tiles.clear();
    game.bullets.clear();
    game.units.clear();
    game.mines.clear();
    game.players.clear();
    game.loot_boxes.clear();
    game
}
