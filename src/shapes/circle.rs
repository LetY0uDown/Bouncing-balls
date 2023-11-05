use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use crate:: { WIDTH, HEIGHT };
use crate::math::math::ZERO;
use crate::math::vectors::{vec_dot, vec_multiply, vec_norm, VEC_ZERO};
pub const MAX_VELOCITY: f32 = 350.0;
#[derive(Clone)]
pub struct Circle {
	pub id: u16,
	pub pos: Vec2,
	pub velocity: Vec2,
	pub mass: f32,
	pub radius: f32,
	pub orig_color: Color,
	pub color: Color
}

impl Circle {
	pub const RADIUS_MAX: f32 = 80.0;
	pub const RADIUS_MIN: f32 = 10.0;
	pub fn new(id: u16, pos: Vec2,
	           radius: f32, velocity: Vec2,
	           color: Color, orig_color: Color, mass: f32) -> Circle
	{
		return Circle {
			id, pos, radius, velocity, color, mass, orig_color
		}
	}

	pub fn empty() -> Circle {
		return Circle::new(0,   VEC_ZERO,
		                   0.0, VEC_ZERO,
		                   Color::WHITE, Color::WHITE,
		                   0.0);
	}

	// Checks if circle is inside the screen. Inverting it's velocity if it goes out
	pub fn handle_window_collision(&mut self) {
		let out_hor = self.pos.x + self.radius > WIDTH as f32
						 || self.pos.x - self.radius < ZERO;

		let out_vert= self.pos.y + self.radius > HEIGHT as f32
						 || self.pos.y - self.radius < ZERO;

		if out_hor {
			self.velocity.x *= -1.0;
		}

		if out_vert {
			self.velocity.y *= -1.0;
		}
	}

	// Checks if two circles does intersect with each other
	pub fn intersects_with(&self, other: &Circle) -> bool {
		if self.id == other.id { return false; }

		let dy = other.pos.y - self.pos.y;
		let dx = other.pos.x - self.pos.x;
		let dist = self.radius + other.radius;

		return dx * dx + dy * dy <= dist * dist;
	}
	pub fn handle_collision_with (&mut self, other: &mut Circle) -> bool {
		let collide = self.intersects_with(other);

		// If circles collides - change velocities. It's just math, nothing scary.. I promise..
		if collide {
			let masses = (2.0 * other.mass) / (self.mass + other.mass);

			/* calculate new velocity for self */
			let mut dot = vec_dot(self.velocity - other.velocity, self.pos - other.pos);

			let mut div = dot / (vec_norm(self.pos - other.pos) * vec_norm(self.pos - other.pos));

			let mut velocity = self.velocity - vec_multiply(self.pos - other.pos, masses * div);

			self.velocity = velocity;

			/* calculate new velocity for other */
			dot = vec_dot(other.velocity - self.velocity, other.pos - self.pos);

			div = dot / (vec_norm(other.pos - self.pos) * vec_norm(other.pos - self.pos));

			velocity = self.velocity - vec_multiply(other.pos - self.pos, masses * div);

			other.velocity = velocity;
		}

		return collide;
	}
}