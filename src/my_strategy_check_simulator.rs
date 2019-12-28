use model::{
    Game,
    Unit,
    UnitAction,
};

use crate::my_strategy::{
    Config,
    Debug,
    Positionable,
    SeedableRng,
    Simulator,
    World,
    XorShiftRng,
    my_strategy_impl,
};

pub struct MyStrategyImpl {
    last_tick: i32,
    rng: XorShiftRng,
    world: World,
    strategy_impl: my_strategy_impl::MyStrategyImpl,
    actions: Vec<(i32, UnitAction)>,
    predicted_units: Vec<(Unit, Unit)>,
    predicted_number_of_opponent_bullets: usize,
    predicted_number_of_teammate_bullets: usize,
    predicted_number_of_opponent_mines: usize,
    predicted_number_of_teammate_mines: usize,
}

impl MyStrategyImpl {
    pub fn new(config: Config, unit: Unit, game: Game) -> Self {
        Self {
            last_tick: -1,
            rng: XorShiftRng::from_seed([
                3918248293,
                2127433321,
                1841971383,
                1904458926,
            ]),
            world: World::new(config.clone(), unit.player_id, game.clone()),
            predicted_number_of_opponent_bullets: game.bullets.iter().filter(|v| v.player_id != unit.player_id).count(),
            predicted_number_of_teammate_bullets: game.bullets.iter().filter(|v| v.player_id == unit.player_id).count(),
            predicted_number_of_opponent_mines: game.mines.iter().filter(|v| v.player_id != unit.player_id).count(),
            predicted_number_of_teammate_mines: game.mines.iter().filter(|v| v.player_id == unit.player_id).count(),
            strategy_impl: my_strategy_impl::MyStrategyImpl::new(config, unit, game),
            actions: Vec::new(),
            predicted_units: Vec::new(),
        }
    }

    pub fn get_action(&mut self, unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
            self.world.update(game);
            self.actions.clear();
        }

        let action = self.strategy_impl.get_action(unit, game, debug);

        self.actions.push((unit.id, action.clone()));

        if self.actions.len() == game.units.iter().filter(|v| v.player_id == unit.player_id).count() {
            if self.predicted_number_of_opponent_bullets > game.bullets.iter().filter(|v| v.player_id != unit.player_id).count() {
                println!("[{}] opponent_bullets current={} predicted={}",
                    self.world.current_tick(), game.bullets.iter().filter(|v| v.player_id != unit.player_id).count(), self.predicted_number_of_opponent_bullets);
            }

            if self.predicted_number_of_teammate_bullets > game.bullets.iter().filter(|v| v.player_id == unit.player_id).count() {
                println!("[{}] teammate_bullets current={} predicted={}",
                    self.world.current_tick(), game.bullets.iter().filter(|v| v.player_id == unit.player_id).count(), self.predicted_number_of_teammate_bullets);
            }

            if self.predicted_number_of_opponent_mines > game.mines.iter().filter(|v| v.player_id != unit.player_id).count() {
                println!("[{}] opponent_mines current={} predicted={}",
                    self.world.current_tick(), game.mines.iter().filter(|v| v.player_id != unit.player_id).count(), self.predicted_number_of_opponent_mines);
            }

            if self.predicted_number_of_teammate_mines > game.mines.iter().filter(|v| v.player_id == unit.player_id).count() {
                println!("[{}] teammate_mines current={} predicted={}",
                    self.world.current_tick(), game.mines.iter().filter(|v| v.player_id == unit.player_id).count(), self.predicted_number_of_teammate_mines);
            }

            for (previous_unit, predicted_unit) in self.predicted_units.iter() {
                let action = self.actions.iter().find(|(id, _)| *id == previous_unit.id);

                if action.is_none() {
                    continue;
                }

                let current_unit = game.units.iter().find(|v| v.id == previous_unit.id).unwrap();
                let mut is_equivalent = true;

                if predicted_unit.jump_state.can_jump != current_unit.jump_state.can_jump {
                    println!("jump_state.can_jump");
                    is_equivalent = false;
                }

                if predicted_unit.jump_state.can_cancel != current_unit.jump_state.can_cancel {
                    println!("jump_state.can_cancel");
                    is_equivalent = false;
                }

                if predicted_unit.jump_state.max_time != current_unit.jump_state.max_time {
                    println!("jump_state.max_time");
                    is_equivalent = false;
                }

                if predicted_unit.jump_state.speed != current_unit.jump_state.speed {
                    println!("jump_state.speed");
                    is_equivalent = false;
                }

                if predicted_unit.on_ground != current_unit.on_ground {
                    println!("on_ground");
                    is_equivalent = false;
                }

                if predicted_unit.on_ladder != current_unit.on_ladder {
                    println!("on_ladder");
                    is_equivalent = false;
                }

                if predicted_unit.weapon.is_some() != current_unit.weapon.is_some() {
                    println!("weapon");
                    is_equivalent = false;
                }

                if predicted_unit.health != current_unit.health {
                    println!("health");
                    is_equivalent = false;
                }

                // if predicted_unit.position() != current_unit.position() {
                //     println!("position");
                //     is_equivalent = false;
                // }

                if !is_equivalent {
                    println!("[{}][{}]   action {:?}", current_unit.id, self.world.current_tick(), action);
                    println!("[{}][{}] previous {:?}", current_unit.id, self.world.current_tick(), previous_unit);
                    println!("[{}][{}]  current {:?}", current_unit.id, self.world.current_tick(), current_unit);
                    println!("[{}][{}]  predict {:?}", current_unit.id, self.world.current_tick(), predicted_unit);
                }
            }

            let mut simulator = Simulator::new(&self.world, unit.id);

            for (id, action) in self.actions.iter() {
                simulator.set_unit_action(*id, action.clone());
            }

            simulator.tick(self.world.tick_time_interval(), self.world.properties().updates_per_tick as usize, &mut self.rng, &mut Some(debug));

            self.predicted_units = simulator.units().iter()
                .map(|predicted_unit| {
                    let current_unit = game.units.iter().find(|v| v.id == predicted_unit.base().id).unwrap();
                    (current_unit.clone(), predicted_unit.base().clone())
                })
                .collect();

            self.predicted_number_of_opponent_bullets = simulator.bullets().iter().filter(|v| v.base().player_id != unit.player_id).count();
            self.predicted_number_of_teammate_bullets = simulator.bullets().iter().filter(|v| v.base().player_id == unit.player_id).count();
            self.predicted_number_of_opponent_mines = simulator.mines().iter().filter(|v| v.base().player_id != unit.player_id).count();
            self.predicted_number_of_teammate_mines = simulator.mines().iter().filter(|v| v.base().player_id == unit.player_id).count();
        }

        action
    }
}
