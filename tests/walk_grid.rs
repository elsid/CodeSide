use aicup2019::my_strategy::{
    Vec2,
    WalkGrid,
};

#[test]
pub fn test_walk_grid_center_by_horizontal() {
    assert_eq!(
        WalkGrid::new(Vec2::new(0.5, 0.5), Vec2::new(2.5, 0.5)).collect::<Vec<_>>(),
        vec![Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2.5, 0.5)]
    );
}

#[test]
pub fn test_walk_grid_center_by_vertical() {
    assert_eq!(
        WalkGrid::new(Vec2::new(0.5, 0.5), Vec2::new(0.5, 2.5)).collect::<Vec<_>>(),
        vec![Vec2::new(0.5, 0.5), Vec2::new(0.5, 1.5), Vec2::new(0.5, 2.5)]
    );
}

#[test]
pub fn test_walk_grid_center() {
    assert_eq!(
        WalkGrid::new(Vec2::new(0.5, 0.5), Vec2::new(1.5, 2.5)).collect::<Vec<_>>(),
        vec![Vec2::new(0.5, 0.5), Vec2::new(0.5, 1.5), Vec2::new(1.5, 1.5), Vec2::new(1.5, 2.5)]
    );
}

#[test]
pub fn test_walk_grid_from_border_to_border() {
    assert_eq!(
        WalkGrid::new(Vec2::new(0.5, 0.1), Vec2::new(2.5, 0.9)).collect::<Vec<_>>(),
        vec![Vec2::new(0.5, 0.1), Vec2::new(1.5, 0.1), Vec2::new(2.5, 0.1)]
    );
}

#[test]
pub fn test_walk_grid_from_center_to_border() {
    assert_eq!(
        WalkGrid::new(Vec2::new(0.5, 0.5), Vec2::new(2.5, 0.9)).collect::<Vec<_>>(),
        vec![Vec2::new(0.5, 0.5), Vec2::new(1.5, 0.5), Vec2::new(2.5, 0.5)]
    );
}

#[test]
pub fn test_walk_grid_1() {
    assert_eq!(
        WalkGrid::new(Vec2::new(38.000000000000455, 12.066666666666693), Vec2::new(1.549171296296291, 11.9)).collect::<Vec<_>>().last(),
        Some(&Vec2::new(1.0000000000004547, 11.066666666666693))
    );
}

#[test]
pub fn test_walk_grid_2() {
    assert_eq!(
        WalkGrid::new(Vec2::new(25.121666666678834, 18.066666667666514), Vec2::new(9.549999999, 16.516666666667387)).collect::<Vec<_>>().last(),
        Some(&Vec2::new(9.121666666678834, 16.066666667666514))
    );
}

#[test]
pub fn test_walk_grid_3() {
    assert_eq!(
        WalkGrid::new(Vec2::new(25.788333333344895, 17.900000001), Vec2::new(9.831666665666711, 17.900000001)).collect::<Vec<_>>().last(),
        Some(&Vec2::new(9.788333333344895, 17.900000001))
    );
}
