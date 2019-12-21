#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    TextAlignment,
    Vec2F32,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::Clamp1;

pub struct Debug<'r, 'd> {
    base: &'r mut crate::Debug<'d>,
    next_y: f32,
}

impl<'r, 'd> Debug<'r, 'd> {
    pub fn new(base: &'r mut crate::Debug<'d>) -> Self {
        Self { base, next_y: 0.0 }
    }

    pub fn with_next_y(next_y: f32, base: &'r mut crate::Debug<'d>) -> Self {
        Self { base, next_y }
    }

    pub fn next_y(&self) -> f32 {
        self.next_y
    }

    #[cfg(feature = "enable_debug")]
    pub fn draw(&mut self, data: CustomData) {
        self.base.draw(data);
    }

    #[cfg(feature = "enable_debug")]
    pub fn log(&mut self, text: String) {
        let y = self.next_y;
        self.next_y += 0.45;
        self.base.draw(CustomData::PlacedText {
            text,
            pos: Vec2F32 { x: -2.5, y },
            alignment: TextAlignment::Left,
            size: 20.5,
            color: ColorF32 { a: 1.0, r: 1.0, g: 1.0, b: 1.0 },
        });
    }
}

#[cfg(feature = "enable_debug")]
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
