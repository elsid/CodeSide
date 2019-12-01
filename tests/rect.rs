use my_strategy::my_strategy::rect::Rect;
use my_strategy::my_strategy::vec2::Vec2;

#[test]
fn test_rect_collide() {
    assert_eq!(
        Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))),
        Vec2::new(-2.0, -2.0)
    );
    assert_eq!(
        Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(4.0, 2.0), Vec2::new(1.0, 1.0))),
        Vec2::new(0.0, -2.0)
    );
    assert_eq!(
        Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(2.0, 4.0), Vec2::new(1.0, 1.0))),
        Vec2::new(-2.0, 0.0)
    );
    assert_eq!(
        Rect::new(Vec2::new(4.0, 2.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))),
        Vec2::new(0.0, -2.0)
    );
    assert_eq!(
        Rect::new(Vec2::new(2.0, 4.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))),
        Vec2::new(-2.0, 0.0)
    );
    assert_eq!(
        Rect::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 1.0))
            .collide(&Rect::new(Vec2::new(4.0, 4.0), Vec2::new(1.0, 1.0))),
        Vec2::new(0.0, 0.0)
    );
}
