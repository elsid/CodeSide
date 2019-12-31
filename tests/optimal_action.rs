use aicup2019::{
    my_strategy::{
        Vec2,
        limit_rotation_to,
    },
};

#[test]
fn test_limit_rotation_to_with_max_positive_to_zero() {
    let current_angle = std::f64::consts::PI;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 3.141592653589793);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.2);
    assert_eq!(result.rotation(required_direction), 2.9415926535897934);
}

#[test]
fn test_limit_rotation_to_with_max_negative_to_zero() {
    let current_angle = -std::f64::consts::PI;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 3.141592653589793);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.2);
    assert_eq!(result.rotation(required_direction), 2.9415926535897934);
}

#[test]
fn test_limit_rotation_to_with_large_positive_to_max() {
    let current_angle = 2.9;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(-1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 0.24159265358979315);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.19999999999999946);
    assert_eq!(result.rotation(required_direction), 0.04159265358979282);
}

#[test]
fn test_limit_rotation_to_with_large_negative_to_max() {
    let current_angle = -2.9;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(-1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 0.24159265358979315);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.19999999999999946);
    assert_eq!(result.rotation(required_direction), 0.04159265358979282);
}

#[test]
fn test_limit_rotation_to_with_small_positive_to_max() {
    let current_angle = 0.5;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(-1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 2.641592653589793);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.19999999999999946);
    assert_eq!(result.rotation(required_direction), 2.4415926535897934);
}

#[test]
fn test_limit_rotation_to_with_small_negative_to_max() {
    let current_angle = -0.5;
    let current_direction = Vec2::i().rotated(current_angle);
    let required_direction = Vec2::new(-1.0, 0.0);

    assert_eq!(current_direction.rotation(required_direction), 2.641592653589793);

    let max_rotation = 0.2;
    let result = limit_rotation_to(required_direction, current_direction, max_rotation);

    assert_eq!(result.rotation(current_direction), 0.19999999999999946);
    assert_eq!(result.rotation(required_direction), 2.4415926535897934);
}
