use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::Rng;
use speedy2d::dimen::Vec2;

pub fn random01() -> f32 {
	return rand_num(0.0 ..= 1.0);
}

pub fn rand_num<T, R>(range: R) -> T where T: SampleUniform,
                                           R: SampleRange<T>
{
	let mut rng = rand::thread_rng();

	return rng.gen_range(range);
}

pub fn random_sign() -> i8 {
	let mut rng = rand::thread_rng();

	return if rng.gen_bool(0.5) { 1 } else { -1 };
}

pub fn random_direction() -> Vec2 {
	let range = -1.0..=1.0;
	return Vec2::from((
		rand_num(range.clone()),
		rand_num(range)
	));
}