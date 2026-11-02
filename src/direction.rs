use bevy::prelude::*;

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
