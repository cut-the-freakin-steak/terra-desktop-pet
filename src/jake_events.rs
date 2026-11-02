use bevy::prelude::*;

use crate::{DirX, DirY, Direction, UsingMonitor};
use crate::{JAKE_WIDTH_ISH, Jake, JakeState};

#[derive(Component, Clone, Copy)]
pub struct JakeIntroArc {
	pub start: Vec3,
	pub end: Vec3,
	pub peak_height: f32,
	pub duration: f32,
	pub elapsed: f32,
}

#[derive(Component, Clone, Copy)]
pub struct JakeSquashNStretch {
	pub duration: f32,
	pub intensity: f32, // how extreme the squash/stretch is
	pub elapsed: f32,
}

pub fn jake_intro(
	mut commands: Commands,
	mut jake: Single<(Entity, &mut Transform, &mut JakeIntroArc)>,
	time: Res<Time>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let (entity, transform, intro_arc) = &mut *jake;

	// like this whole thing is copy-pasted from stack overflow lmaoo
	intro_arc.elapsed += time.delta_secs();
	let t = (intro_arc.elapsed / intro_arc.duration).clamp(0.0, 1.0);

	let mut current = intro_arc.start.lerp(intro_arc.end, t);

	// parabolic arc offset for y
	// formula: 4 * height * t * (1 - t)
	// evaluates to 0 at t=0 and t=1, and 'peak_height' at t=0.5
	// it's just a quadratic lmao
	let arc_offset = 4.0 * intro_arc.peak_height * t * (1.0 - t);

	// combine linear height interpolation with the arc offset
	let base_y = intro_arc.start.y + (intro_arc.end.y - intro_arc.start.y) * t;
	current.y = base_y + arc_offset;

	transform.translation = current;

	const MAGIC_NUMBER: f32 = -12.40; // genuinely no rationale behind this number. pure trial and error.
	transform.rotate(Quat::from_rotation_z(MAGIC_NUMBER * time.delta_secs()));

	// finishes animation
	if t >= 1.0 {
		transform.rotation = Quat::IDENTITY; // Quat::IDENTITY resets to default rotation (using a 0'd array didn't work well lmao)
		commands.entity(*entity).remove::<JakeIntroArc>();
		next_jake_state.set(JakeState::SquashAndStretch);
	}
}

pub fn jake_squash_n_stretch(
	mut commands: Commands,
	mut jake: Single<(Entity, &mut Transform, &mut JakeSquashNStretch, &mut Jake)>,
	time: Res<Time>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let (entity, transform, effect, jake) = &mut *jake;

	// like this whole thing is copy-pasted from stack overflow lmaoo 2 again the sequel :3
	effect.elapsed += time.delta_secs();
	let t = (effect.elapsed / effect.duration).clamp(0.0, 1.0);

	// create a decaying (i guess) oscillation using a sine wave multiplied by an exponential decay
	let decay = (1.0 - t).powi(2); // fades out as it finishes
	let wave = (t * std::f32::consts::TAU * 3.0).sin() * effect.intensity * decay;

	transform.scale.x = jake.scale * (1.0 + wave);
	transform.scale.y = jake.scale * (1.0 - wave);

	// remove the component when the effect finishes
	if t >= 1.0 {
		// reset to default base scale
		transform.scale = Vec3::splat(jake.scale);
		commands.entity(*entity).remove::<JakeSquashNStretch>();
		next_jake_state.set(JakeState::Roaming);
	}
}

pub fn jake_events(
	mut jake: Single<(&Jake, &mut Transform, &GlobalTransform, &mut Direction)>,
	monitor: Res<UsingMonitor>,
	time: Res<Time>,
) {
	// this is for future random events that jAkE can partake in
	match rand::random_range(0..100) {
		_ => {
			let (jake, transform, g_transform, direction) = &mut *jake;
			let monitor = &monitor.0;

			let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;
			let half_monitor_height = (monitor.physical_height as f64 / monitor.scale_factor) / 2.0;

			// right side of monitor
			if half_monitor_width
				<= (g_transform.translation().x + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
			{
				direction.0 = DirX::Left;
			}
			// left side of monitor
			else if -half_monitor_width
				>= (g_transform.translation().x - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
			{
				direction.0 = DirX::Right;
			}
			// top of monitor
			if half_monitor_height
				<= (g_transform.translation().y + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
			{
				direction.1 = DirY::Down;
			}
			// bottom of monitor
			else if -half_monitor_height
				>= (g_transform.translation().y - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
			{
				direction.1 = DirY::Up;
			}

			match direction.0 {
				DirX::Left => transform.rotate(Quat::from_rotation_z(0.05)),
				DirX::Right => transform.rotate(Quat::from_rotation_z(-0.05)),
			}

			const SPEED: f32 = 250.0;
			transform.translation.x += direction.0 as i8 as f32 * SPEED * time.delta_secs();
			transform.translation.y += direction.1 as i8 as f32 * SPEED * time.delta_secs();
		}
	}
}
