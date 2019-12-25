use model::Properties;

pub trait ImplicitProperties {
    fn max_jump_pad_height(&self) -> f64;
    fn max_unit_jump_height(&self) -> f64;
    fn max_jump_pad_length(&self) -> f64;
    fn max_unit_jump_length(&self) -> f64;
    fn tick_time_interval(&self) -> f64;
}

impl ImplicitProperties for Properties {
    fn max_jump_pad_height(&self) -> f64 {
        self.jump_pad_jump_speed * self.jump_pad_jump_time + 1.0
    }

    fn max_unit_jump_height(&self) -> f64 {
        self.unit_jump_speed * self.unit_jump_time
    }

    fn max_jump_pad_length(&self) -> f64 {
        self.unit_max_horizontal_speed * self.jump_pad_jump_time
    }

    fn max_unit_jump_length(&self) -> f64 {
        self.unit_max_horizontal_speed * self.unit_jump_time
    }

    fn tick_time_interval(&self) -> f64 {
        1.0 / self.ticks_per_second as f64
    }
}
