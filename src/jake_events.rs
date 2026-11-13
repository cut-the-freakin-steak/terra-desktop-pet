use std::ops::Range;

use bevy::prelude::*;

use crate::{DirX, DirY, Direction, FixedTransform, UsingMonitor};
use crate::{JAKE_WIDTH_ISH, Jake, JakeState};

#[derive(Component, Clone, Copy)]
pub struct JakeIntroArc {
	pub start: Vec3,
	pub end: Vec3,
	pub peak_height: f32,
	pub duration: f32,
	pub elapsed: f32,
	pub magic_rotation_number: f32,
}

#[derive(Component, Clone, Copy)]
pub struct JakeSquashNStretch {
	pub duration: f32,
	pub intensity: f32, // how extreme the squash/stretch is
	pub elapsed: f32,
}

pub fn jake_intro(
	mut commands: Commands,
	mut jake: Single<(Entity, &mut FixedTransform, &mut JakeIntroArc)>,
	time: Res<Time<Fixed>>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let (entity, f_transform, intro_arc) = &mut *jake;
	let translation = f_transform.translation_mut();

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

	*translation = current;

	f_transform.rotate(Quat::from_rotation_z(
		intro_arc.magic_rotation_number * time.delta_secs(),
	));

	// finishes animation
	if t >= 1.0 {
		// Quat::IDENTITY resets to default rotation (using a 0'd array didn't work well lmao)
		*f_transform.rotation_mut() = Quat::IDENTITY;
		commands.entity(*entity).remove::<JakeIntroArc>();
		next_jake_state.set(JakeState::SquashAndStretch);
	}
}

pub fn jake_squash_n_stretch(
	mut commands: Commands,
	mut jake: Single<(
		Entity,
		&mut FixedTransform,
		&mut JakeSquashNStretch,
		&mut Jake,
	)>,
	time: Res<Time<Fixed>>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let (entity, transform, effect, jake) = &mut *jake;

	// like this whole thing is copy-pasted from stack overflow lmaoo 2 again the sequel :3
	effect.elapsed += time.delta_secs();
	let t = (effect.elapsed / effect.duration).clamp(0.0, 1.0);

	// create a decaying (i guess) oscillation using a sine wave multiplied by an exponential decay
	let decay = (1.0 - t).powi(2); // fades out as it finishes
	let wave = (t * std::f32::consts::TAU * 3.0).sin() * effect.intensity * decay;

	transform.scale_mut().x = jake.scale * (1.0 + wave);
	transform.scale_mut().y = jake.scale * (1.0 - wave);

	// remove the component when the effect finishes
	if t >= 1.0 {
		// reset to default base scale
		*transform.scale_mut() = Vec3::splat(jake.scale);
		commands.entity(*entity).remove::<JakeSquashNStretch>();
		next_jake_state.set(JakeState::Roaming);
	}
}

/// shows which border of the screen jAkE last bounced on
#[derive(Resource, Clone, Copy)]
pub struct LastScreenBounce(pub Direction);

#[derive(Resource, Clone, Copy, Default)]
pub enum JakeEvents {
	None,
	#[default]
	Move,
	Vibrate,
}

pub fn jake_events(
	mut jake: Single<(
		&mut Jake,
		&mut FixedTransform,
		&GlobalTransform,
		&mut Direction,
	)>,
	monitor: Res<UsingMonitor>,
	time: Res<Time<Fixed>>,
	mut event_timer: Local<Option<Timer>>,
	mut jake_events: ResMut<JakeEvents>,
	mut last_bounce: ResMut<LastScreenBounce>,
	mut jake_speed: Local<f32>,
) {
	let event_timer =
		event_timer.get_or_insert_with(|| Timer::from_seconds(1.0, TimerMode::Repeating));

	let (jake, f_transform, g_transform, direction) = &mut *jake;
	let g_position = g_transform.translation();

	// bounds checks
	let monitor = &monitor.0;
	let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;
	let half_monitor_height = (monitor.physical_height as f64 / monitor.scale_factor) / 2.0;

	// right side of monitor
	if half_monitor_width <= (g_position.x + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64 {
		direction.0 = DirX::Left;
		last_bounce.0.0 = DirX::Right;
	}
	// left side of monitor
	else if -half_monitor_width >= (g_position.x - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64 {
		direction.0 = DirX::Right;
		last_bounce.0.0 = DirX::Left;
	}
	else {
		last_bounce.0.0 = DirX::None;
	}
	// top of monitor
	if half_monitor_height <= (g_position.y + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64 {
		direction.1 = DirY::Down;
		last_bounce.0.1 = DirY::Up;
	}
	// bottom of monitor
	else if -half_monitor_height >= (g_position.y - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64 {
		direction.1 = DirY::Up;
		last_bounce.0.1 = DirY::Down;
	}
	else {
		last_bounce.0.1 = DirY::None;
	}

	// makes event stuff run every second
	let mut do_event = true;
	if !event_timer.tick(time.delta()).just_finished() {
		do_event = false;
	}

	// random events that jAkE can partake in
	if do_event {
		match rand::random_range(0.0..1.0) {
			0.0..0.33 => {
				jake_speed = *jake_events = JakeEvents::Move;
			}
			0.33..0.66 => *jake_events = JakeEvents::Vibrate,
			_ => *jake_events = JakeEvents::None,
		}
	}

	match *jake_events {
		JakeEvents::None => {}
		JakeEvents::Move => jake_move(
			jake,
			f_transform,
			direction,
			&time,
			&last_bounce,
			event_timer,
			1100.0..3100.0,
		),
		JakeEvents::Vibrate => jake_vibrate(
			jake,
			f_transform,
			direction,
			&time,
			&last_bounce,
			event_timer,
		),
	};
}

fn jake_move(
	jake: &mut Jake,
	f_transform: &mut FixedTransform,
	direction: &mut Direction,
	time: &Res<Time<Fixed>>,
	last_bounce: &ResMut<LastScreenBounce>,
	event_timer: &Timer,
	speed_range: Range<f32>,
) {
	if event_timer.just_finished() {}
	let mut desired_dir_x = if rand::random_bool(0.5) {
		DirX::Left
	}
	else {
		DirX::Right
	};

	let mut desired_dir_y = if rand::random_bool(0.5) {
		DirY::Up
	}
	else {
		DirY::Down
	};

	if last_bounce.0.0 == desired_dir_x {
		if rand::random_bool(0.25) {
			desired_dir_x = desired_dir_x.opposite();
		}
		else {
			desired_dir_x = DirX::None;
		}
	}

	if last_bounce.0.1 == desired_dir_y {
		if rand::random_bool(0.25) {
			desired_dir_y = desired_dir_y.opposite();
		}
		else {
			desired_dir_y = DirY::None;
		}
	}

	let dir = Direction::new(desired_dir_x, desired_dir_y);
	let speed_x = rand::random_range(speed_range.clone());
	let speed_y = rand::random_range(speed_range);
	let difference = Vec3::new(
		dir.0 as i8 as f32 * speed_x,
		dir.1 as i8 as f32 * speed_y,
		0.0,
	);
	*f_transform.translation_mut() += difference * time.delta_secs();
}

fn jake_vibrate(
	jake: &mut Jake,
	f_transform: &mut FixedTransform,
	direction: &mut Direction,
	time: &Res<Time<Fixed>>,
	last_bounce: &ResMut<LastScreenBounce>,
	event_timer: &Timer,
) {
	// just a liiitle movement is fine :3
	jake_move(
		jake,
		f_transform,
		direction,
		time,
		last_bounce,
		event_timer,
		100.0..300.0,
	)
}
