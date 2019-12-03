#[derive(Debug, Clone)]
pub struct Config {
    pub max_plan_iterations: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_plan_iterations: 1000,
        }
    }
}
