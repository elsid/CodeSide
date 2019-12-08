#[derive(Debug, Clone)]
pub struct Config {
    pub max_plan_iterations: usize,
    pub min_ticks_per_transition: usize,
    pub max_ticks_per_transition: usize,
    pub microticks_per_tick: usize,
    pub distance_score_weight: f64,
    pub health_diff_score_weight: f64,
    pub optimal_tile_distance_to_position_score_weight: f64,
    pub optimal_tile_distance_to_opponent_score_weight: f64,
    pub optimal_tile_health_pack_score_weight: f64,
    pub optimal_tile_first_weapon_score_weight: f64,
    pub optimal_tile_swap_weapon_score_weight: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_plan_iterations: 100,
            min_ticks_per_transition: 5,
            max_ticks_per_transition: 30,
            microticks_per_tick: 3,
            distance_score_weight: -1.0,
            health_diff_score_weight: 0.5,
            optimal_tile_distance_to_position_score_weight: -1.0,
            optimal_tile_distance_to_opponent_score_weight: 0.5,
            optimal_tile_health_pack_score_weight: 2.0,
            optimal_tile_first_weapon_score_weight: 3.0,
            optimal_tile_swap_weapon_score_weight: 1.0,
        }
    }
}
