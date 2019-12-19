#[derive(Debug, Clone)]
pub struct Config {
    pub plan_max_iterations: usize,
    pub plan_min_ticks_per_transition: usize,
    pub plan_max_ticks_per_transition: usize,
    pub plan_microticks_per_tick: usize,
    pub plan_max_state_depth: usize,
    pub plan_distance_score_weight: f64,
    pub plan_health_diff_score_weight: f64,
    pub plan_game_score_diff_score_weight: f64,
    pub optimal_tile_distance_to_position_score_weight: f64,
    pub optimal_tile_distance_to_opponent_score_weight: f64,
    pub optimal_tile_health_pack_score_weight: f64,
    pub optimal_tile_first_weapon_score_weight: f64,
    pub optimal_tile_swap_weapon_score_weight: f64,
    pub optimal_tile_hit_by_opponent_score_weight: f64,
    pub optimal_tile_opponent_obstacle_score_weight: f64,
    pub optimal_tile_hit_nearest_opponent_score_weight: f64,
    pub optimal_tile_loot_box_mine_score_weight: f64,
    pub optimal_tile_height_score_weight: f64,
    pub optimal_tile_over_ground_score_weight: f64,
    pub optimal_tile_bullets_score_weight: f64,
    pub optimal_tile_mines_score_weight: f64,
    pub optimal_tile_mine_obstacle_score_weight: f64,
    pub optimal_tile_hit_teammates_score_weight: f64,
    pub optimal_tile_distance_to_teammates_score_weight: f64,
    pub min_hit_probability_by_spread_to_shoot: f64,
    pub min_hit_probability_over_obstacles_to_shoot: f64,
    pub max_hit_teammates_probability_to_shoot: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            plan_max_iterations: 60,
            plan_min_ticks_per_transition: 3,
            plan_max_ticks_per_transition: 7,
            plan_microticks_per_tick: 3,
            plan_max_state_depth: 30,
            plan_distance_score_weight: 0.25,
            plan_health_diff_score_weight: 1.0,
            plan_game_score_diff_score_weight: 1.0,
            optimal_tile_distance_to_position_score_weight: -0.25,
            optimal_tile_distance_to_opponent_score_weight: 0.5,
            optimal_tile_health_pack_score_weight: 1.5,
            optimal_tile_first_weapon_score_weight: 3.0,
            optimal_tile_swap_weapon_score_weight: 1.0,
            optimal_tile_hit_by_opponent_score_weight: -1.0,
            optimal_tile_opponent_obstacle_score_weight: -1.0,
            optimal_tile_hit_nearest_opponent_score_weight: 2.0,
            optimal_tile_loot_box_mine_score_weight: 0.1,
            optimal_tile_height_score_weight: 0.1,
            optimal_tile_over_ground_score_weight: 0.1,
            optimal_tile_bullets_score_weight: -0.5,
            optimal_tile_mines_score_weight: -0.5,
            optimal_tile_mine_obstacle_score_weight: -1.0,
            optimal_tile_hit_teammates_score_weight: -1.0,
            optimal_tile_distance_to_teammates_score_weight: -1.0,
            min_hit_probability_by_spread_to_shoot: 0.3,
            min_hit_probability_over_obstacles_to_shoot: 0.1,
            max_hit_teammates_probability_to_shoot: 0.1,
        }
    }
}
