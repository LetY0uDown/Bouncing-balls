pub const ZERO: f32 = 0.0;

pub fn lerp(start: f32, end: f32, scale: f32) -> f32 {
	return  start * (1.0 - scale) + end * scale;
}

pub fn clamp01(num: f32) -> f32 {
	return f32::clamp(num, ZERO, 1.0);
}

pub fn normalize_between(min: f32, max: f32, value: f32) -> f32 {
	return (value - min) / (max - min);
}