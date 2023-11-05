use crate::math::math::lerp;

pub enum Curve {
	Linear,
	CubicIn,
	CubicOut,
	CubicInOut,
	QuadIn,
	QuadOut,
	QuadInOut,
	ExpoIn,
	ExpoOut,
	ExpoInOut
}

pub fn evaluate_curve(min: f32, max: f32, value: f32, curve: &Curve) -> f32 {
	let func: fn(f32, f32, f32) -> f32 = get_curve_func(curve);

	return func(min, max, value);
}

// For visualization and better understanding visit https://easings.net/
pub fn get_curve_func(curve: &Curve) -> fn(f32, f32, f32) -> f32 {
	match curve {
		Curve::Linear => linear,
		Curve::CubicIn => cubic_in,
		Curve::CubicOut => cubic_out,
		Curve::CubicInOut => cubic_in_out,
		Curve::QuadIn => quad_in,
		Curve::QuadOut => quad_out,
		Curve::QuadInOut => quad_in_out,
		Curve::ExpoIn => expo_in,
		Curve::ExpoOut => expo_out,
		Curve::ExpoInOut => expo_in_out
	}
}

pub fn linear(min: f32, max: f32, val: f32) -> f32 {
	return lerp(min, max, val);
}

// Quad curves
pub fn quad_in(min: f32, mut max: f32, val: f32) -> f32 {
	max -= min;
	return max * val * val + min;
}
pub fn quad_out(min: f32, mut max: f32, val: f32) -> f32 {
	max -= min;
	return -max * val * (val - 2.0) + min;
}
pub fn quad_in_out(min: f32, mut max: f32, mut val: f32) -> f32 {
	val /= 0.5;
	max -= min;

	if val < 1.0 {
		return max * 0.5 * val * val + min;
	}

	val -= 1.0;
	return -max * 0.5 * (val * (val - 2.0) - 1.0) + min;
}

// Cubic curves
pub fn cubic_in(min: f32, mut max: f32, val: f32) -> f32 {
	max -= min;
	return max  * val * val * val + min;
}

pub fn cubic_out(min: f32, max: f32, mut val: f32) -> f32 {
	val -= 1.0;

	return (max - min) * (val * val * val + 1.0) + min;
}

pub fn cubic_in_out(min: f32, mut max: f32, mut val: f32) -> f32 {
	val /= 0.5;
	max -= min;

	if val < 1.0 {
		return max * 0.5 * val * val * val + min;
	}

	val -= 2.0;
	return max * 0.5 * (val * val * val + 2.0) + min;
}

// Expo curves
pub fn expo_in (min: f32, mut max: f32, val: f32) -> f32 {
	max -= min;
	return max * f32::powf(2.0, 10.0 * (val - 1.0)) + min;
}
pub fn expo_out (min: f32, mut max: f32, val: f32) -> f32 {
	max -= min;
	return max * (-f32::powf(2.0, -10.0 * val) + 1.0) + min;
}
pub fn expo_in_out (min: f32, mut max: f32, mut val: f32) -> f32 {
	val /= 0.5;
	max -= min;

	if val < 1.0 {
		return  max * 0.5 * f32::powf(2.0, 10.0 * (val - 1.0)) + min;
	}

	val -= 1.0;
	return max * 0.5 * (-f32::powf(2.0, -10.0 * val) + 2.0) + min;
}