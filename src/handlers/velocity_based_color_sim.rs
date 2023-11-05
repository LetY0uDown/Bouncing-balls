use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::window::VirtualKeyCode::{Space};
use crate::{BACKGROUND, DELTA_TIME, HEIGHT, WIDTH};
use crate::math::color::color_lerp;
use crate::math::curve::{Curve, evaluate_curve};
use crate::math::math::{normalize_between, ZERO};
use crate::math::vectors::{vec_clamp_coords, VEC_ZERO};
use crate::random::random::{random_sign, random01, rand_num, random_direction};
use crate::shapes::circle::{Circle, MAX_VELOCITY};
const POS_DELTA: f32 = 50.0;

pub struct CircleWithVelocityColorSim {
	pub circles: Vec<Circle>,
	pub clamp_velocity: bool,
	pub zero_speed_color: Color
}

impl WindowHandler for CircleWithVelocityColorSim
{
	fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
	{
		graphics.clear_screen(BACKGROUND);

		// Made nested iteration possible
		let mut others = self.circles.clone();

		for circle in &mut self.circles {
			// Check if current circle collides with another
			for mut other in &mut others {
				_ = circle.handle_collision_with(&mut other);
			}

			// Adjust velocity so circle doesn't go outside of the screen
			circle.handle_window_collision();

			if self.clamp_velocity {
				circle.velocity = vec_clamp_coords(circle.velocity, -MAX_VELOCITY, MAX_VELOCITY);
			}

			let speed = circle.velocity.magnitude();
			let scale = normalize_between(ZERO, MAX_VELOCITY, f32::abs(speed));

			circle.color = color_lerp(self.zero_speed_color, circle.orig_color, scale);

			// Move circle and draw it in a new position
			circle.pos += circle.velocity * DELTA_TIME;

			graphics.draw_circle(circle.pos, circle.radius, circle.color);
		}

		helper.request_redraw();
	}

	fn on_key_down(&mut self, _helper: &mut WindowHelper<()>,
	               virtual_key_code: Option<VirtualKeyCode>,
	               _scancode: KeyScancode) {
		if virtual_key_code.unwrap() == Space {
			for circle in &mut self.circles {
				if circle.velocity != VEC_ZERO {
					circle.velocity = VEC_ZERO;
				} else {
					let range = 35.0..=65.0;

					circle.velocity = random_direction();
					circle.velocity.x *= rand_num(range.clone());
					circle.velocity.y *= rand_num(range);
				}
			}
		}
	}
}

impl CircleWithVelocityColorSim {
	pub fn init(circles_count: usize, zero_speed_color: Color, distribution: &Curve) -> CircleWithVelocityColorSim {
		let mut circles: Vec<Circle> = Vec::new();

		let mut prev_circle: Circle = Circle::empty();

		let mut position = Vec2::from((0.0, Circle::RADIUS_MAX + POS_DELTA));

		for i in 0..circles_count {
			let mut circle = Circle::empty();

			circle.id = i as u16;
			circle.orig_color = Color::WHITE;
			circle.color = circle.orig_color;

			// Generate random mass for a circle
			let mass = random01();
			circle.mass = mass * 10.0;

			// Calculate radius. Bigger mass - bigger radius
			circle.radius = evaluate_curve(Circle::RADIUS_MIN, Circle::RADIUS_MAX, mass, distribution);

			// Calculate new position
			position.x += circle.radius + prev_circle.radius + POS_DELTA;

			if position.x > WIDTH as f32 {
				position.x = ZERO + circle.radius + POS_DELTA;
				position.y += Circle::RADIUS_MAX * 2.0 + POS_DELTA;
			}

			circle.pos = position;

			let range = 50.0..75.0;

			// Set init velocity
			circle.velocity = Vec2::from((random_sign() as f32 * rand_num(range.clone()),
			                              (random_sign() as f32 * rand_num(range))));

			_ = &circles.push(circle.clone());

			prev_circle = circle.clone();
		}

		return CircleWithVelocityColorSim {
			circles, clamp_velocity: false, zero_speed_color
		};
	}

	pub fn start(mut self, fullscreen: bool, clamp_velocity: bool) {
		let window = if fullscreen {
			Window::new_fullscreen_borderless("Bouncing balls").unwrap()
		} else {
			Window::new_centered("Bouncing balls", (WIDTH, HEIGHT)).unwrap()
		};

		self.clamp_velocity = clamp_velocity;

		window.run_loop(self);
	}
}