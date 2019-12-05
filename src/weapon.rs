use model::Weapon;
use crate::my_strategy::WeaponParamsWrapper;

#[derive(Debug)]
pub struct WeaponWrapper<'a>(pub &'a Weapon);

impl<'a> PartialEq for WeaponWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        let Self(lhs) = self;
        let Self(rhs) = other;
        (
            WeaponParamsWrapper(&lhs.params),
            &lhs.typ,
            lhs.magazine,
            lhs.was_shooting,
            lhs.spread,
            lhs.fire_timer,
            lhs.last_angle,
            lhs.last_fire_tick,
        ).eq(&(
            WeaponParamsWrapper(&rhs.params),
            &rhs.typ,
            rhs.magazine,
            rhs.was_shooting,
            rhs.spread,
            rhs.fire_timer,
            rhs.last_angle,
            rhs.last_fire_tick,
        ))
    }
}

impl<'a> Eq for WeaponWrapper<'a> {}
