use model::ColorF32;
use crate::my_strategy::Clamp1;

pub fn color_from_heat(alpha: f32, mut value: f32) -> ColorF32 {
    value = value.clamp1(0.0, 1.0);
    if value < 0.25 {
        ColorF32 { a: alpha, r: 0.0, g: 4.0 * value, b: 1.0}
    } else if value < 0.5 {
        ColorF32 { a: alpha, r: 0.0, g: 1.0, b: 1.0 - 4.0 * (value - 0.5)}
    } else if value < 0.75 {
        ColorF32 { a: alpha, r: 4.0 * (value - 0.5), g: 1.0, b: 0.0}
    } else {
        ColorF32 { a: alpha, r: 1.0, g: 1.0 - 4.0 * (value - 0.75), b: 0.0}
    }
}
