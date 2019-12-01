use model::UnitAction;
use crate::my_strategy::vec2_f64::Vec2F64Wrapper;

#[derive(Debug)]
pub struct UnitActionWrapper<'a>(pub &'a UnitAction);

impl<'a> PartialEq for UnitActionWrapper<'a> {
    fn eq(&self, other: &UnitActionWrapper) -> bool {
        let UnitActionWrapper(lhs) = self;
        let UnitActionWrapper(rhs) = other;
        (
            lhs.velocity,
            lhs.jump,
            lhs.jump_down,
            Vec2F64Wrapper(lhs.aim.clone()),
            lhs.shoot,
            lhs.swap_weapon,
            lhs.plant_mine,
        ).eq(&(
            rhs.velocity,
            rhs.jump,
            rhs.jump_down,
            Vec2F64Wrapper(rhs.aim.clone()),
            rhs.shoot,
            rhs.swap_weapon,
            rhs.plant_mine,
        ))
    }
}

impl<'a> Eq for UnitActionWrapper<'a> {}
