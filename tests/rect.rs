use aicup2019::my_strategy::{
    Rect,
    Vec2,
};

#[test]
fn test_has_intersection_with_line_horizontal() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, 0.0), Vec2::new(2.0, 0.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 0.0), Vec2::new(-2.0, 0.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 0.5), Vec2::new(-2.0, 0.5)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 1.0), Vec2::new(-2.0, 1.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 2.0), Vec2::new(-2.0, 2.0)),
        false
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-3.0, 0.0), Vec2::new(-2.0, 0.0)),
        false
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 0.0), Vec2::new(3.0, 0.0)),
        false
    );
}

#[test]
fn test_has_intersection_with_line_vertical() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(0.0, -2.0), Vec2::new(0.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(0.0, 2.0), Vec2::new(0.0, -2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(0.5, 2.0), Vec2::new(0.5, -2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(1.0, 2.0), Vec2::new(1.0, -2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(2.0, 2.0), Vec2::new(2.0, -2.0)),
        false
    );
}

#[test]
fn test_has_intersection_with_line_left_bottom_to_top_right() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, -2.0), Vec2::new(2.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-3.0, -2.0), Vec2::new(2.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, -3.0), Vec2::new(2.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, -2.0), Vec2::new(2.0, 3.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, -2.0), Vec2::new(3.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-1.0, -2.0), Vec2::new(3.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(-2.0, -1.0), Vec2::new(2.0, 3.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(0.0, -2.0), Vec2::new(4.0, 2.0)),
        true
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)).has_intersection_with_line(Vec2::new(1.0, -2.0), Vec2::new(5.0, 4.0)),
        false
    );
}
