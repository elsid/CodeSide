use model::{
    Item,
    LootBox,
};

use crate::my_strategy::{
    Positionable,
    Vec2,
};

impl Positionable for LootBox {
    fn position(&self) -> Vec2 {
        Vec2::from_model(&self.position)
    }
}

pub fn is_weapon_item(item: &Item) -> bool {
    match item {
        Item::Weapon { .. } => true,
        _ => false,
    }
}

pub fn is_health_pack_item(item: &Item) -> bool {
    match item {
        Item::HealthPack { .. } => true,
        _ => false,
    }
}

pub fn is_mine_item(item: &Item) -> bool {
    match item {
        Item::Mine { .. } => true,
        _ => false,
    }
}
