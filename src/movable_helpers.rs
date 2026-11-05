#![allow(dead_code)]

use bevy::prelude::*;

/// a wrapper for `Transform::translation`, which does FixedUpdate interpolation for smooth-looking movement even when using FixedUpdate, which would usually make movement look jittery
#[derive(Component, Clone, Copy, Default)]
pub struct Position {
	pub current: Vec3,
	pub current_global: Vec3,
	pub previous: Vec3,
}

impl Position {
	pub fn new(vec3: Vec3) -> Self {
		return Self {
			current: vec3,
			current_global: vec3,
			previous: vec3,
		};
	}

	pub fn x(&self) -> f32 {
		self.current.x
	}
	pub fn y(&self) -> f32 {
		self.current.y
	}
	pub fn z(&self) -> f32 {
		self.current.z
	}
	pub fn global_x(&self) -> f32 {
		self.current_global.x
	}
	pub fn global_y(&self) -> f32 {
		self.current_global.y
	}
	pub fn global_z(&self) -> f32 {
		self.current_global.z
	}

	pub fn set_x(&mut self, val: f32) {
		self.current.x = val;
	}
	pub fn set_y(&mut self, val: f32) {
		self.current.y = val;
	}
	pub fn set_z(&mut self, val: f32) {
		self.current.z = val;
	}
	pub fn set_global_x(&mut self, val: f32) {
		self.current_global.x = val;
	}
	pub fn set_global_y(&mut self, val: f32) {
		self.current_global.y = val;
	}
	pub fn set_global_z(&mut self, val: f32) {
		self.current_global.z = val;
	}
}

/// plugin necessary for movement (and interpolation) with `Position`.
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(FixedPreUpdate, advance_position_history);
		app.add_systems(Update, position_interpolation);
	}
}

/// makes the previous position the current position during FixedPreUpdate.
pub fn advance_position_history(query: Query<&mut Position>) {
	for mut position in query {
		position.previous = position.current
	}
}

/// interpolates translation every real frame for smooth movement in Update
pub fn position_interpolation(
	fixed_time: Res<Time<Fixed>>,
	mut query: Query<(&mut Transform, &mut Position)>,
	global_transform_opt: Option<Query<&GlobalTransform>>,
) {
	let alpha = fixed_time.overstep_fraction();

	for (mut transform, mut position) in &mut query {
		// lerp between the previous and current fixed ticks
		let interpolated = position.previous.lerp(position.current, alpha);

		// apply to the actual visual transform
		transform.translation.x = interpolated.x;
		transform.translation.y = interpolated.y;
		transform.translation.z = interpolated.z;

		if let Some(g_transform_query) = global_transform_opt {
			for g_transform in g_transform_query {
				position.current_global = g_transform.translation();
			}
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirX {
	Left = -1,
	Right = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirY {
	Down = -1,
	Up = 1,
}

/// a simple component to demonstrate direction in a visible or movable thing
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Direction(pub DirX, pub DirY);

#[allow(dead_code)]
impl Direction {
	pub fn as_vec2(&self) -> Vec2 {
		return Vec2::new(self.0 as i32 as f32, self.1 as i32 as f32);
	}

	pub fn as_vec3(&self) -> Vec3 {
		return Vec3::new(self.0 as i32 as f32, self.1 as i32 as f32, 0.0);
	}
}
