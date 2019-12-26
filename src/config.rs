#[derive(Debug, Clone)]
#[cfg_attr(feature = "read_config", derive(RustcDecodable))]
#[cfg_attr(feature = "dump_config", derive(RustcEncodable))]
pub struct Config {
    pub plan_max_iterations: usize,
    pub plan_min_ticks_per_transition: usize,
    pub plan_max_ticks_per_transition: usize,
    pub plan_microticks_per_tick: usize,
    pub plan_min_state_depth: usize,
    pub plan_max_state_depth: usize,
    pub plan_distance_score_weight: f64,
    pub plan_health_diff_score_weight: f64,
    pub plan_game_score_diff_score_weight: f64,
    pub plan_triggered_mines_by_me_score_weight: f64,
    pub plan_time_interval_factor: f64,
    pub optimal_location_distance_to_position_score_weight: f64,
    pub optimal_location_distance_to_opponent_score_weight: f64,
    pub optimal_location_health_pack_score_weight: f64,
    pub optimal_location_first_weapon_score_weight: f64,
    pub optimal_location_swap_weapon_score_weight: f64,
    pub optimal_location_hit_by_opponent_score_weight: f64,
    pub optimal_location_opponent_obstacle_score_weight: f64,
    pub optimal_location_hit_nearest_opponent_score_weight: f64,
    pub optimal_location_loot_box_mine_score_weight: f64,
    pub optimal_location_height_score_weight: f64,
    pub optimal_location_over_ground_score_weight: f64,
    pub optimal_location_bullets_score_weight: f64,
    pub optimal_location_mines_score_weight: f64,
    pub optimal_location_mine_obstacle_score_weight: f64,
    pub optimal_location_hit_teammates_score_weight: f64,
    pub optimal_location_hit_probability_score_weight: f64,
    pub optimal_location_min_fire_timer: f64,
    pub optimal_location_number_of_directions: usize,
    pub optimal_location_min_target_hits_to_shoot: usize,
    pub optimal_location_max_teammates_hits_to_shoot: usize,
    pub optimal_location_min_hit_probability_by_spread_to_shoot: f64,
    pub optimal_target_min_target_hits_to_shoot: usize,
    pub optimal_target_max_teammates_hits_to_shoot: usize,
    pub optimal_target_number_of_directions: usize,
    pub optimal_target_min_hit_probability_by_spread_to_shoot: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            plan_max_iterations: 120,
            plan_min_ticks_per_transition: 1,
            plan_max_ticks_per_transition: 1,
            plan_microticks_per_tick: 3,
            plan_min_state_depth: 3,
            plan_max_state_depth: 30,
            plan_distance_score_weight: 0.25,
            plan_health_diff_score_weight: 1.0,
            plan_game_score_diff_score_weight: 1.0,
            plan_triggered_mines_by_me_score_weight: 1.0,
            plan_time_interval_factor: 3.0,
            optimal_location_distance_to_position_score_weight: -0.25,
            optimal_location_distance_to_opponent_score_weight: 0.5,
            optimal_location_health_pack_score_weight: 2.0,
            optimal_location_first_weapon_score_weight: 3.0,
            optimal_location_swap_weapon_score_weight: 1.0,
            optimal_location_hit_by_opponent_score_weight: -1.0,
            optimal_location_opponent_obstacle_score_weight: -1.0,
            optimal_location_hit_nearest_opponent_score_weight: 2.0,
            optimal_location_loot_box_mine_score_weight: 0.1,
            optimal_location_height_score_weight: 0.1,
            optimal_location_over_ground_score_weight: 0.5,
            optimal_location_bullets_score_weight: -0.5,
            optimal_location_mines_score_weight: -3.0,
            optimal_location_mine_obstacle_score_weight: -3.0,
            optimal_location_hit_teammates_score_weight: -1.0,
            optimal_location_hit_probability_score_weight: 1.0,
            optimal_location_min_fire_timer: 0.5,
            optimal_location_number_of_directions: 3,
            optimal_location_min_target_hits_to_shoot: 1,
            optimal_location_max_teammates_hits_to_shoot: 0,
            optimal_location_min_hit_probability_by_spread_to_shoot: 0.3,
            optimal_target_number_of_directions: 11,
            optimal_target_min_target_hits_to_shoot: 1,
            optimal_target_max_teammates_hits_to_shoot: 0,
            optimal_target_min_hit_probability_by_spread_to_shoot: 0.3,
        }
    }

    pub fn adjusted(mut self, team_size: i32) -> Self {
        self.optimal_location_health_pack_score_weight *= (team_size * team_size) as f64;
        self.optimal_location_opponent_obstacle_score_weight *= team_size as f64;
        self
    }
}
