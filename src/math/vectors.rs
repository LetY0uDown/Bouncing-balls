use speedy2d::dimen::Vec2;
use crate::math::math::{lerp, ZERO};

pub const VEC_ZERO: Vec2 = Vec2 {
	x: ZERO,
	y: ZERO,
};

pub fn vec_multiply(vec: Vec2, value: f32) -> Vec2 {
	return Vec2::from((vec.x * value, vec.y * value));
}

pub fn vec_dot(first: Vec2, second: Vec2) -> f32 {
	return first.x * second.x + first.y * second.y;
}
pub fn vec_norm(vec: Vec2) -> f32 {
	return f32::sqrt(vec.x * vec.x + vec.y * vec.y);
}

pub fn vec_lerp(start: Vec2, end: Vec2, scale: f32) -> Vec2 {
	return Vec2::from((
		lerp(start.x, end.x, scale),
		lerp(start.y, end.y, scale)
	));
}

pub fn vec_clamp_coords(mut orig: Vec2, min: f32, max: f32) -> Vec2 {
	orig.x = f32::clamp(orig.x, min, max);
	orig.y = f32::clamp(orig.y, min, max);

	return orig;
}