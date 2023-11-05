#![allow(dead_code)]
mod shapes;
mod random;
mod math;
mod handlers;

use speedy2d::color::Color;

use crate::handlers::default_sim::DefaultSim;
use crate::handlers::velocity_based_color_sim::CircleWithVelocityColorSim;
use crate::handlers::sim_with_masses::CircleMassesSim;

use crate::math::curve::Curve;
use crate::random::random_color::RandomColor;

const FULL_SCREEN: bool = true;

const FACTOR: u32 = 110;
const WIDTH: u32 = if FULL_SCREEN { 3440 } else { 21 * FACTOR };
const HEIGHT: u32 = if FULL_SCREEN { 1440 } else { 9 * FACTOR };

const FPS: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / FPS;

const BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);

const CIRCLES_COUNT: usize = 100;

/* TODO:
     1. Implement better collision function - circles getting stuck in each other after some time
     2. Clean up code. Here's a lot of repetition in simulations which looks bad :(
 */
fn main() {
	let velocity_sim = CircleWithVelocityColorSim::init(CIRCLES_COUNT, Color::TRANSPARENT, &Curve::CubicIn);
	let def_sim = DefaultSim::init(CIRCLES_COUNT, || RandomColor::rgb());
	let masses_sim = CircleMassesSim::init(CIRCLES_COUNT, || RandomColor::rgb(), &Curve::Linear);

	// def_sim.start(FULL_SCREEN, true, true);
	// masses_sim.start(FULL_SCREEN, true, true);
	velocity_sim.start(FULL_SCREEN, true);
}