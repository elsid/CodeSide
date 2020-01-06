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
    let begin = Vec2::new(38.000000000000455, 12.066666666666693);
    let end = Vec2::new(1.549171296296291, 11.9);
    let mut previous = begin;
    for position in WalkGrid::new(begin, end) {
        assert!((position.x() - previous.x()).abs() <= 1.0 && (position.y() - previous.y()).abs() <= 1.0,
            "{:?} {:?} {}", position, previous, (position.x() - previous.x()).abs());
        previous = position;
    }
    assert_eq!(previous, Vec2::new(1.000000001, 11.066666666666693));
}

#[test]
pub fn test_walk_grid_2() {
    let begin = Vec2::new(25.121666666678834, 18.066666667666514);
    let end = Vec2::new(9.549999999, 16.516666666667387);
    let mut previous = begin;
    for position in WalkGrid::new(begin, end) {
        assert!((position.x() - previous.x()).abs() <= 1.0 && (position.y() - previous.y()).abs() <= 1.0,
            "{:?} {:?} {}", position, previous, (position.x() - previous.x()).abs());
        previous = position;
    }
    assert_eq!(previous, Vec2::new(9.121666666678834, 16.066666667666514));
}

#[test]
pub fn test_walk_grid_3() {
    let begin = Vec2::new(25.788333333344895, 17.900000001);
    let end = Vec2::new(9.831666665666711, 17.900000001);
    let mut previous = begin;
    for position in WalkGrid::new(begin, end) {
        assert!((position.x() - previous.x()).abs() <= 1.0 && (position.y() - previous.y()).abs() <= 1.0,
            "{:?} {:?} {}", position, previous, (position.x() - previous.x()).abs());
        previous = position;
    }
    assert_eq!(previous, Vec2::new(9.788333333344895, 17.900000001));
}

#[test]
pub fn test_walk_grid_4() {
    let begin = Vec2::new(4.999999999999999, 22.10000000000406);
    let end = Vec2::new(31.185097964514384, 30.2);
    let mut previous = begin;
    for position in WalkGrid::new(begin, end) {
        assert!((position.x() - previous.x()).abs() <= 1.0 && (position.y() - previous.y()).abs() <= 1.0,
            "{:?} {:?} {}", position, previous, (position.x() - previous.x()).abs());
        previous = position;
    }
    assert_eq!(previous, Vec2::new(31.999999999, 30.10000000000406));
}

#[test]
pub fn test_walk_grid_5() {
    let begin = Vec2::new(17.999999999999847, 16.90000000000372);
    let end = Vec2::new(18.000000000082558, 0.0);
    let mut previous = begin;
    for position in WalkGrid::new(begin, end) {
        assert!((position.x() - previous.x()).abs() <= 1.0 && (position.y() - previous.y()).abs() <= 1.0,
            "{:?} {:?} {}", position, previous, (position.x() - previous.x()).abs());
        previous = position;
    }
    assert_eq!(previous, Vec2::new(18.999999999, 0.9000000000037183));
}
