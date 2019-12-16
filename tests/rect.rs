use aicup2019::my_strategy::{
    Rect,
    Vec2,
};

#[test]
fn test_get_max_cross_section_from_same_position() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        1.0
    );
}

#[test]
fn test_get_max_cross_section_from_on_left_move_by_x() {
    assert_eq!(
        Rect::new(Vec2::new(2.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        1.0
    );
    assert_eq!(
        Rect::new(Vec2::new(4.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.7350032544344594
    );
    assert_eq!(
        Rect::new(Vec2::new(8.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.34787457375638914
    );
    assert_eq!(
        Rect::new(Vec2::new(16.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.1656894153708425
    );
}

#[test]
fn test_get_max_cross_section_from_on_left_move_by_y() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 2.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        1.0
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 4.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.5795595112510077
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 8.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.2064358467682835
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 16.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.08913433098161297
    );
}

#[test]
fn test_get_max_cross_section_from_on_top_move_by_x() {
    assert_eq!(
        Rect::new(Vec2::new(2.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        1.0
    );
    assert_eq!(
        Rect::new(Vec2::new(4.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.7350032544344594
    );
    assert_eq!(
        Rect::new(Vec2::new(8.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.34787457375638914
    );
    assert_eq!(
        Rect::new(Vec2::new(16.0, 0.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.1656894153708425
    );
}

#[test]
fn test_get_max_cross_section_from_on_top_move_by_y() {
    assert_eq!(
        Rect::new(Vec2::new(0.0, 2.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        1.0
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 4.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.5795595112510077
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 8.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.2064358467682835
    );
    assert_eq!(
        Rect::new(Vec2::new(0.0, 16.0), Vec2::new(1.0, 2.0)).get_max_cross_section_from(Vec2::new(0.0, 0.0), 0.8),
        0.08913433098161297
    );
}

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
