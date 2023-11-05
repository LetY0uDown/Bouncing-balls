use speedy2d::color::Color;
use crate::math::math::ZERO;
use crate::random::random::random01;

pub struct RandomColor { }
impl RandomColor {
	pub fn rgb() -> Color {
		return Color::from_rgb(
			random01(), random01(), random01()
		);
	}

	pub fn rand_weighted(r: f32, g: f32, b: f32) -> Color {
		return Color::from_rgb(
			random01() * r, random01() * g, random01() * b
		);
	}
	pub fn red() -> Color {
		return Color::from_rgb(random01(), ZERO, ZERO);
	}
	pub fn green() -> Color {
		return Color::from_rgb(ZERO, random01(), ZERO);
	}
	pub fn blue() -> Color {
		return Color::from_rgb(ZERO, ZERO, random01());
	}
	pub fn gray() -> Color {
		let shade = random01();
		return Color::from_rgb(shade, shade, shade);
	}
}