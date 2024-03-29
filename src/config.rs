#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "read_config", derive(RustcDecodable))]
#[cfg_attr(feature = "dump_config", derive(RustcEncodable))]
pub struct Config {
    pub plan_max_iterations: usize,
    pub plan_microticks_per_tick: usize,
    pub plan_min_state_depth: usize,
    pub plan_max_state_depth: usize,
    pub plan_distance_score_weight: f64,
    pub plan_health_diff_score_weight: f64,
    pub plan_game_score_diff_score_weight: f64,
    pub plan_triggered_mines_by_me_score_weight: f64,
    pub plan_distance_to_nearest_bullet_score_weight: f64,
    pub plan_distance_to_nearest_opponent_score_weight: f64,
    pub plan_time_interval_factor: f64,
    pub optimal_location_distance_to_position_score_weight: f64,
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
    pub optimal_location_teammate_obstacle_score_weight: f64,
    pub optimal_location_bullet_obstacle_score_weight: f64,
    pub optimal_location_opponent_mine_explosion_score_weight: f64,
    pub optimal_location_min_fire_timer: f64,
    pub optimal_location_number_of_directions: usize,
    pub min_hit_probability_by_spread_to_shoot: f64,
    pub optimal_action_number_of_directions: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            plan_max_iterations: 160,
            plan_microticks_per_tick: 9,
            plan_min_state_depth: 3,
            plan_max_state_depth: 25,
            plan_distance_score_weight: 1.0,
            plan_health_diff_score_weight: 100.0,
            plan_game_score_diff_score_weight: 100.0,
            plan_triggered_mines_by_me_score_weight: 100.0,
            plan_distance_to_nearest_bullet_score_weight: 1.0,
            plan_distance_to_nearest_opponent_score_weight: 1.0,
            plan_time_interval_factor: 3.0,
            optimal_location_distance_to_position_score_weight: -0.25,
            optimal_location_health_pack_score_weight: 16.0,
            optimal_location_first_weapon_score_weight: 7.0,
            optimal_location_swap_weapon_score_weight: 1.0,
            optimal_location_hit_by_opponent_score_weight: -300.0,
            optimal_location_opponent_obstacle_score_weight: -4.0,
            optimal_location_hit_nearest_opponent_score_weight: 600.0,
            optimal_location_loot_box_mine_score_weight: 0.1,
            optimal_location_height_score_weight: 0.1,
            optimal_location_over_ground_score_weight: 0.5,
            optimal_location_bullets_score_weight: -0.5,
            optimal_location_mines_score_weight: -10.0,
            optimal_location_mine_obstacle_score_weight: -3.0,
            optimal_location_hit_teammates_score_weight: -1.0,
            optimal_location_teammate_obstacle_score_weight: -1.0,
            optimal_location_bullet_obstacle_score_weight: -1.0,
            optimal_location_opponent_mine_explosion_score_weight: -6.0,
            optimal_location_min_fire_timer: 0.5,
            optimal_location_number_of_directions: 3,
            min_hit_probability_by_spread_to_shoot: 0.30000000000000004,
            optimal_action_number_of_directions: 9,
        }
    }

    pub fn adjusted(mut self, team_size: i32) -> Self {
        self.plan_max_iterations /= team_size as usize;
        self.optimal_location_health_pack_score_weight *= team_size as f64;
        self
    }
}
