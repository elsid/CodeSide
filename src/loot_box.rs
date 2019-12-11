use model::LootBox;
use crate::my_strategy::{
    Positionable,
    Vec2,
};

impl Positionable for LootBox {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}
