use aicup2019::my_strategy::{
    Rect,
    Sector,
    Vec2,
};

#[test]
fn test_sector_from_direction_and_spread_i() {
    assert_eq!(
        Sector::from_direction_and_spread(Vec2::new(1.0, 0.0), 0.5),
        Sector::new(-0.5, 1.0),
    );
}

#[test]
fn test_sector_from_direction_and_spread_j() {
    assert_eq!(
        Sector::from_direction_and_spread(Vec2::new(0.0, 1.0), 0.5),
        Sector::new(1.0707963267948966, 1.0),
    );
}

#[test]
fn test_sector_from_direction_and_spread_over_period() {
    assert_eq!(
        Sector::from_direction_and_spread(Vec2::new(-1.0, -0.1), 0.5),
        Sector::new(2.741261306080955, 1.0),
    );
}

#[test]
fn test_sector_from_source_and_rect_1() {
    assert_eq!(
        Sector::from_source_and_rect(Vec2::new(0.5, 0.3), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9))),
        Sector::new(-0.07677189126977804, 0.19706736819773218),
    );
}

#[test]
fn test_sector_from_source_and_rect_2() {
    assert_eq!(
        Sector::from_source_and_rect(Vec2::new(19.5, 0.7), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9))),
        Sector::new(3.064820762320015, 0.1970673681977324),
    );
}

#[test]
fn test_sector_get_intersection_fraction_ordered_within_period() {
    assert_eq!(
        Sector::new(-0.25, 0.5).get_intersection_fraction(Sector::new(0.125, 0.5)),
        0.25
    );
}

#[test]
fn test_sector_get_intersection_fraction_reverse_ordered_within_period() {
    assert_eq!(
        Sector::new(0.125, 0.5).get_intersection_fraction(Sector::new(-0.25, 0.5)),
        0.25
    );
}

#[test]
fn test_sector_get_intersection_fraction_ordered_not_intersected() {
    assert_eq!(
        Sector::new(0.0, 0.25).get_intersection_fraction(Sector::new(0.5, 0.25)),
        0.0
    );
}

#[test]
fn test_sector_get_intersection_fraction_reverse_not_ordered_not_intersected() {
    assert_eq!(
        Sector::new(0.5, 0.25).get_intersection_fraction(Sector::new(0.0, 0.25)),
        0.0
    );
}

#[test]
fn test_sector_get_intersection_fraction_ordered_outside_period() {
    assert_eq!(
        Sector::new(std::f64::consts::PI - 0.25, 0.5).get_intersection_fraction(Sector::new(- std::f64::consts::PI + 0.125, 0.5)),
        0.25
    );
}

#[test]
fn test_sector_get_intersection_fraction_reverse_ordered_outside_period() {
    assert_eq!(
        Sector::new(- std::f64::consts::PI + 0.125, 0.5).get_intersection_fraction(Sector::new(std::f64::consts::PI - 0.25, 0.5)),
        0.25
    );
}

#[test]
fn test_sector_get_intersection_fraction_containing_other() {
    assert_eq!(
        Sector::new(-0.5, 1.25).get_intersection_fraction(Sector::new(-0.25, 0.5)),
        0.4
    );
}

#[test]
fn test_sector_get_intersection_fraction_contained_by_other() {
    assert_eq!(
        Sector::new(-0.25, 0.5).get_intersection_fraction(Sector::new(-0.5, 1.25)),
        1.0
    );
}
