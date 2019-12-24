use aicup2019::my_strategy::{
    normalize_angle,
};

#[test]
fn test_normalize_angle_zero() {
    assert_eq!(normalize_angle(0.0), 0.0);
}

#[test]
fn test_normalize_angle_positive_less_than_period() {
    assert_eq!(normalize_angle(0.5 * std::f64::consts::PI), 0.5 * std::f64::consts::PI);
}

#[test]
fn test_normalize_angle_negative_less_than_period() {
    assert_eq!(normalize_angle(-0.5 * std::f64::consts::PI), -0.5 * std::f64::consts::PI);
}

#[test]
fn test_normalize_angle_positive_greater_than_period() {
    assert_eq!(normalize_angle(2.5 * std::f64::consts::PI), 0.5 * std::f64::consts::PI);
}

#[test]
fn test_normalize_angle_negative_greater_than_period() {
    assert_eq!(normalize_angle(-2.5 * std::f64::consts::PI), - 0.5 * std::f64::consts::PI);
}
