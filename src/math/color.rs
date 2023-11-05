use speedy2d::color::Color;
use crate::math::math::{clamp01, lerp};

pub fn color_lerp(start: Color, end: Color, scale: f32) -> Color {
	return Color::from_rgba(
		lerp(start.r(), end.r(), scale),
		lerp(start.g(), end.g(), scale),
		lerp(start.b(), end.b(), scale),
		lerp(start.a(), end.a(), scale)
	);
}

pub fn color_add(first: Color, second: Color) -> Color {
	return Color::from_rgb(
		clamp01(first.r() + second.r()),
		clamp01(first.g() + second.g()),
		clamp01(first.b() + second.b())
	);
}