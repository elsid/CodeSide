use aicup2019::my_strategy::{
    Rect,
    Vec2,
};

#[test]
fn test_rect_has_intersection_with_line_cross_rect() {
    let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    for i in -10 .. 11 {
        let angle = i as f64 * std::f64::consts::PI / 10.0;
        let direction = Vec2::i().rotated(angle);
        let a = -direction * 10.0;
        let b = direction * 10.0;
        assert!(rect.has_intersection_with_line(a, b), "{} {:?} {:?}", angle, a, b);
    }
}

#[test]
fn test_rect_has_intersection_with_line_outside() {
    let shifts = [
        Vec2::new(-20.0, 0.0),
        Vec2::new(20.0, 0.0),
        Vec2::new(0.0, -20.0),
        Vec2::new(0.0, 20.0),
    ];
    let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    for &shift in &shifts {
        for i in -10 .. 11 {
            let angle = i as f64 * std::f64::consts::PI / 10.0;
            let direction = Vec2::i().rotated(angle);
            let a = shift - direction * 10.0;
            let b = shift + direction * 10.0;
            assert!(!rect.has_intersection_with_line(a, b), "{} {:?} {:?}", angle, a, b);
        }
    }
}
