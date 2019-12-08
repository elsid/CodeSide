use model::WeaponParams;

#[derive(Debug)]
pub struct WeaponParamsWrapper<'a>(pub &'a WeaponParams);

impl<'a> PartialEq for WeaponParamsWrapper<'a> {
    fn eq(&self, other: &Self) -> bool {
        let Self(lhs) = self;
        let Self(rhs) = other;
        (
            lhs.magazine_size,
            lhs.fire_rate,
            lhs.reload_time,
            lhs.min_spread,
            lhs.max_spread,
            lhs.recoil,
            // BulletParamsWrapper(&lhs.bullet),
            // ExplosionParamsWrapper(&lhs.explosion),
        ).eq(&(
            rhs.magazine_size,
            rhs.fire_rate,
            rhs.reload_time,
            rhs.min_spread,
            rhs.max_spread,
            rhs.recoil,
            // BulletParamsWrapper(&rhs.bullet),
            // ExplosionParamsWrapper(&rhs.explosion),
        ))
    }
}

impl<'a> Eq for WeaponParamsWrapper<'a> {}
