use aicup2019::my_strategy::{
    Rect,
    Vec2,
};

#[test]
fn test_rect_get_intersection_with_line_cross_rect() {
    let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    for i in -10 .. 11 {
        let angle = i as f64 * std::f64::consts::PI / 10.0;
        let direction = Vec2::i().rotated(angle);
        let a = -direction * 10.0;
        let b = direction * 10.0;
        let factor = rect.get_intersection_with_line(a, b);
        assert!(factor.is_some(), "{} {:?} {:?}", angle, a, b);
        let result = a + (b - a) * factor.unwrap();
        assert!(result.x() == 1.0 || result.x() == -1.0 || result.y() == 1.0 || result.y() == -1.0,
            "{:?} {} {:?} {:?}", result, angle, a, b);
    }
}

#[test]
fn test_rect_get_intersection_with_line_outside() {
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
            let factor = rect.get_intersection_with_line(a, b);
            assert!(!factor.is_some(), "{:?} {} {:?} {:?}", factor, angle, a, b);
        }
    }
}

#[test]
fn test_rect_get_intersection_with_line_from_inside() {
    let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    for i in -10 .. 11 {
        let angle = i as f64 * std::f64::consts::PI / 10.0;
        let direction = Vec2::i().rotated(angle);
        let a = Vec2::new(0.0, 0.0);
        let b = direction * 10.0;
        let factor = rect.get_intersection_with_line(a, b);
        assert!(factor.is_some(), "{} {:?} {:?}", angle, a, b);
        let result = a + (b - a) * factor.unwrap();
        assert!(
            (1.0 - result.x()).abs() <= 1e-10
            || (-1.0 - result.x()).abs() <= 1e-10
            || (1.0 - result.y()).abs() <= 1e-10
            || (-1.0 - result.y()).abs() <= 1e-10,
            "{:?} {} {:?} {:?}", result, angle, a, b
        );
    }
}

#[test]
fn test_rect_get_intersection_with_line_1() {
    let rect = Rect::new(Vec2::new(0.5, 3.5), Vec2::new(0.5, 0.5));
    let src = Vec2::new(3.9999999999997913, 1.9);
    let dst = Vec2::new(0.0000000000000004440892098500626, 3.5740928184624208);
    let factor = rect.get_intersection_with_line(src, dst);
    assert!(factor.is_some());
}
