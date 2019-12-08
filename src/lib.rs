pub mod my_strategy;

pub struct Debug<'a>(pub &'a mut dyn std::io::Write);

impl Debug<'_> {
    fn draw(&mut self, data: model::CustomData) {
        use trans::Trans;
        model::PlayerMessageGame::CustomDataMessage { data }
            .write_to(&mut self.0)
            .expect("Failed to write custom debug data");
    }
}
