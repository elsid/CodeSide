use crate::Debug;
use crate::my_strategy::config::Config;
use crate::my_strategy::world::World;

pub struct MyStrategyImpl {
    has_loot_box: bool,
    has_bullet: bool,
    has_mine: bool,
}

impl MyStrategyImpl {
    pub fn new() -> Self {
        Self {
            has_loot_box: false,
            has_bullet: false,
            has_mine: false,
        }
    }

    pub fn get_action(&mut self, me: &model::Unit, game: &model::Game, debug: &mut Debug) -> model::UnitAction {
        if game.current_tick == 0 {
            println!("player: {:?}", game.players[0]);
            println!("me: {:?}", me);
            println!("unit0: {:?}", game.units[0]);
            println!("unit1: {:?}", game.units[1]);
            println!("properties: {:?}", game.properties);
            println!("game: {:?}", get_game_without_repeatable(game.clone()));
            println!("level: {:?}", game.level);
        }
        if !self.has_loot_box && game.loot_boxes.len() > 0 {
            self.has_loot_box = true;
            println!("loot_box: {:?}", game.loot_boxes[0]);
        }
        if !self.has_bullet && game.bullets.len() > 0 {
            self.has_bullet = true;
            println!("bullet: {:?}", game.bullets[0]);
        }
        if !self.has_mine && game.mines.len() > 0 {
            self.has_mine = true;
            println!("mine: {:?}", game.mines[0]);
        }
        if self.has_loot_box && self.has_bullet && self.has_mine {
            std::process::exit(0);
        }
        model::UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: model::Vec2F64 { x: 0.0, y: 0.0 },
            shoot: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }
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
