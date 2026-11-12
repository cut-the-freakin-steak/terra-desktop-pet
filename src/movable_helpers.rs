#![allow(dead_code)]

use bevy::prelude::*;

/// wrapper for `Transform`, which does `FixedUpdate` interpolation for smooth-looking movement,
/// because it would usually look jittery
///
/// requires entity to have the `Transform` component, preferably with the same starting position.
/// this process can be made trivial by using `FixedTransformBundle` like this:
/// ```rs
/// Transform::from_xyz(x, y, z).into_fixed_bundle();
/// ```
#[derive(Component, Clone, Copy, Default, PartialEq)]
pub struct FixedTransform {
	pub current: Transform,
	pub previous: Transform,
}

impl FixedTransform {
	pub fn new(transform: Transform) -> Self {
		return Self {
			current: transform,
			previous: transform,
		};
	}

	pub fn translation(&self) -> Vec3 {
		return self.current.translation;
	}
	pub fn translation_mut(&mut self) -> &mut Vec3 {
		return &mut self.current.translation;
	}

	pub fn scale(&self) -> Vec3 {
		return self.current.scale;
	}
	pub fn scale_mut(&mut self) -> &mut Vec3 {
		return &mut self.current.scale;
	}

	pub fn rotation(&self) -> Quat {
		return self.current.rotation;
	}
	pub fn rotation_mut(&mut self) -> &mut Quat {
		return &mut self.current.rotation;
	}
	pub fn rotate(&mut self, quat: Quat) {
		self.current.rotate(quat);
	}
}

/// plugin necessary for movement (and interpolation) with `Position`.
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			FixedPreUpdate,
			FixedTransformSystems::advance_transform_history,
		);
		app.add_systems(Update, FixedTransformSystems::transform_interpolation);
	}
}

pub struct FixedTransformSystems;
impl FixedTransformSystems {
	/// makes the previous position the current position during FixedPreUpdate.
	pub fn advance_transform_history(query: Query<&mut FixedTransform>) {
		for mut transform in query {
			transform.previous = transform.current;
		}
	}

	/// interpolates translation every real frame for smooth movement in Update
	pub fn transform_interpolation(
		fixed_time: Res<Time<Fixed>>,
		mut query: Query<(&FixedTransform, &mut Transform)>,
	) {
		let alpha = fixed_time.overstep_fraction();

		for (fixed_transform, mut transform) in &mut query {
			// lerp between the previous and current fixed ticks
			let interpolated_pos = fixed_transform
				.previous
				.translation
				.lerp(fixed_transform.current.translation, alpha);

			let interpolated_scale = fixed_transform
				.previous
				.scale
				.lerp(fixed_transform.current.scale, alpha);

			let interpolated_rotation = fixed_transform
				.previous
				.rotation
				.lerp(fixed_transform.current.rotation, alpha);

			// apply to the actual visual transform
			transform.translation = interpolated_pos;
			transform.scale = interpolated_scale;
			transform.rotation = interpolated_rotation;
		}
	}
}

#[derive(Bundle)]
pub struct FixedTransformBundle {
	pub transform: Transform,
	pub fixed_transform: FixedTransform,
}

impl From<Transform> for FixedTransformBundle {
	fn from(transform: Transform) -> Self {
		return Self {
			fixed_transform: FixedTransform::new(transform),
			transform,
		};
	}
}

pub trait IntoFixedTransform {
	fn into_fixed(self) -> FixedTransform;
}

impl IntoFixedTransform for Transform {
	/// turn a `Transform` into a `FixedTransform`, for interpolation
	fn into_fixed(self) -> FixedTransform {
		return FixedTransform {
			current: self,
			previous: self,
		};
	}
}

pub trait IntoFixedTransformBundle {
	fn into_fixed_bundle(self) -> FixedTransformBundle;
}

impl IntoFixedTransformBundle for Transform {
	/// turn a `Transform` into a `FixedTransformBundle`, for interpolation
	/// differs from `IntoFixedTransform` by including a `Transform` for entity initialization
	fn into_fixed_bundle(self) -> FixedTransformBundle {
		return FixedTransformBundle {
			transform: self,
			fixed_transform: FixedTransform {
				current: self,
				previous: self,
			},
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DirX {
	Left = -1,
	Right = 1,
	#[default]
	None = 0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DirY {
	Down = -1,
	Up = 1,
	#[default]
	None = 0,
}

/// a simple component to demonstrate direction in a visible or movable thing
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Direction(pub DirX, pub DirY);

#[allow(dead_code)]
impl Direction {
	pub fn new(x: DirX, y: DirY) -> Self {
		return Direction(x, y);
	}

	pub fn as_vec2(&self) -> Vec2 {
		return Vec2::new(self.0 as i32 as f32, self.1 as i32 as f32);
	}

	pub fn as_vec3(&self) -> Vec3 {
		return Vec3::new(self.0 as i32 as f32, self.1 as i32 as f32, 0.0);
	}

	/// returns the opposite of both x and y directions in a `Direction`
	pub fn opposite(&self) -> Self {
		return Self(self.0.opposite(), self.1.opposite());
	}

	/// returns the opposite of the x direction in a `Direction`
	pub fn opposite_x(&self) -> Self {
		return Self(self.0.opposite(), self.1);
	}

	/// returns the opposite of the y direction in a `Direction`
	pub fn opposite_y(&self) -> Self {
		return Self(self.0, self.1.opposite());
	}
}

impl DirX {
	pub fn opposite(&self) -> Self {
		match self {
			Self::Left => return Self::Right,
			Self::Right => return Self::Left,
			Self::None => return Self::None,
		}
	}
}

impl DirY {
	pub fn opposite(&self) -> Self {
		match self {
			Self::Up => return Self::Down,
			Self::Down => return Self::Up,
			Self::None => return Self::None,
		}
	}
}
