use std::ops::Range;

use bevy::prelude::*;

use crate::{
	DirX, DirY, Direction, FixedTransform, JAKE_BASE_SCALE, JakeEyes, JakeWidth, UsingMonitor,
};
use crate::{Jake, JakeNose, JakeState};

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
	mut jake: Single<(Entity, &mut FixedTransform, &mut JakeSquashNStretch), With<Jake>>,
	time: Res<Time<Fixed>>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
	mut jake_starting_scale: Local<Option<f32>>,
) {
	let (entity, f_transform, effect) = &mut *jake;
	let jake_starting_scale = jake_starting_scale.get_or_insert_with(|| f_transform.scale().x);

	// like this whole thing is copy-pasted from stack overflow lmaoo 2 again the sequel :3
	effect.elapsed += time.delta_secs();
	let t = (effect.elapsed / effect.duration).clamp(0.0, 1.0);

	// create a decaying (i guess) oscillation using a sine wave multiplied by an exponential decay
	let decay = (1.0 - t).powi(2); // fades out as it finishes
	let wave = (t * std::f32::consts::TAU * 3.0).sin() * effect.intensity * decay;

	f_transform.scale_mut().x = *jake_starting_scale * (1.0 + wave);
	f_transform.scale_mut().y = *jake_starting_scale * (1.0 - wave);

	// remove the component when the effect finishes
	if t >= 1.0 {
		// reset to default base scale
		let prev_z = f_transform.scale().z;
		*f_transform.scale_mut() = Vec3::new(*jake_starting_scale, *jake_starting_scale, prev_z);
		commands.entity(*entity).remove::<JakeSquashNStretch>();
		next_jake_state.set(JakeState::Roaming);
	}
}

/// shows which border of the screen jAkE last bounced on
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub struct LastScreenBounce(pub Direction);

#[derive(Resource, Clone, Copy, Default, PartialEq, Eq)]
pub enum JakePhysicalEvents {
	#[default]
	None,
	Move {
		tweak: bool,
	},
}

#[derive(Resource, Clone, PartialEq, Eq)]
pub struct PhysicalEventTimer(pub Timer);

#[derive(Resource, Clone, Copy, Default, PartialEq)]
pub enum JakeAltEvents {
	#[default]
	None,
	GlassesSpin,
}

#[derive(Resource, Clone, PartialEq, Eq)]
pub struct AltEventTimer(pub Timer);

#[derive(Resource, Clone, Copy, Default, PartialEq, Eq)]
pub enum JakeSoundEvents {
	None,
	#[default]
	Greetings,
}

#[derive(Resource, Clone, PartialEq, Eq)]
pub struct SoundEventTimer(pub Timer);

pub fn jake_events(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	time: Res<Time<Fixed>>,
	monitor: Res<UsingMonitor>,
	mut last_bounce: ResMut<LastScreenBounce>,
	mut jake: Single<(&GlobalTransform, &mut Direction), With<Jake>>,
	jake_width_ish: Res<JakeWidth>,
	mut jake_physical_events: ResMut<JakePhysicalEvents>,
	mut physical_event_timer: ResMut<PhysicalEventTimer>,
	mut jake_sound_events: ResMut<JakeSoundEvents>,
	mut sound_event_timer: ResMut<SoundEventTimer>,
	mut jake_alt_events: ResMut<JakeAltEvents>,
	mut alt_event_timer: ResMut<AltEventTimer>,
) {
	let (g_transform, direction) = &mut *jake;
	let jake_width_ish = jake_width_ish.0;
	let g_position = g_transform.translation();
	let physical_event_timer = &mut physical_event_timer.0;
	let sound_event_timer = &mut sound_event_timer.0;
	let alt_event_timer = &mut alt_event_timer.0;

	// bounds checks
	let monitor = &monitor.0;
	let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;
	let half_monitor_height = (monitor.physical_height as f64 / monitor.scale_factor) / 2.0;

	// right side of monitor
	if half_monitor_width <= (g_position.x + (jake_width_ish * JAKE_BASE_SCALE) / 2.0) as f64 {
		direction.0 = DirX::Left;
		last_bounce.0.0 = DirX::Right;
	}
	// left side of monitor
	else if -half_monitor_width
		>= (g_position.x - (jake_width_ish * JAKE_BASE_SCALE) / 2.0) as f64
	{
		direction.0 = DirX::Right;
		last_bounce.0.0 = DirX::Left;
	}
	else {
		last_bounce.0.0 = DirX::None;
	}
	// top of monitor
	if half_monitor_height <= (g_position.y + (jake_width_ish * JAKE_BASE_SCALE) / 2.0) as f64 {
		direction.1 = DirY::Down;
		last_bounce.0.1 = DirY::Up;
	}
	// bottom of monitor
	else if -half_monitor_height
		>= (g_position.y - (jake_width_ish * JAKE_BASE_SCALE) / 2.0) as f64
	{
		direction.1 = DirY::Up;
		last_bounce.0.1 = DirY::Down;
	}
	else {
		last_bounce.0.1 = DirY::None;
	}

	// makes physical event stuff run every 2 seconds
	let mut do_physical_event = true;
	if !physical_event_timer.tick(time.delta()).just_finished() {
		do_physical_event = false;
	}

	// random physical (things on screen) events that jAkE can partake in
	if do_physical_event {
		match rand::random_range(0.0..1.0) {
			0.0..0.05 => *jake_physical_events = JakePhysicalEvents::Move { tweak: true },
			0.05..0.66 => {
				*jake_physical_events = JakePhysicalEvents::Move { tweak: false };
			}
			_ => *jake_physical_events = JakePhysicalEvents::None,
		};
	}

	// makes sound event stuff run every 10 seconds
	let mut do_sound_event = true;
	if !sound_event_timer.tick(time.delta()).just_finished() {
		do_sound_event = false;
	}

	// random sound (thing that hear in ear) events that jAkE can partake in
	if do_sound_event {
		match rand::random_range(0.0..1.0) {
			0.0..0.10 => *jake_sound_events = JakeSoundEvents::Greetings,
			_ => *jake_sound_events = JakeSoundEvents::None,
		};
	}

	match *jake_sound_events {
		JakeSoundEvents::None => {}
		JakeSoundEvents::Greetings => {
			commands.spawn((
				AudioPlayer::new(asset_server.load("embedded://audio/jAkE_name.ogg")),
				PlaybackSettings::DESPAWN,
			));

			*jake_sound_events = JakeSoundEvents::None;
		}
	}

	// makes alt event stuff run every 15 seconds
	let mut do_alt_event = true;
	if !alt_event_timer.tick(time.delta()).just_finished() {
		do_alt_event = false;
	}

	if do_alt_event {
		match rand::random_range(0.0..1.0) {
			0.0..0.70 => *jake_alt_events = JakeAltEvents::GlassesSpin,
			_ => *jake_alt_events = JakeAltEvents::None,
		}
	}
}

pub fn jake_move(
	mut jake: Single<(Entity, &mut FixedTransform), With<Jake>>,
	jake_physical_events: Res<JakePhysicalEvents>,
	children_query: Query<&Children>,
	mut sprites_and_transforms: Query<
		(&mut Sprite, &mut FixedTransform, Option<&JakeNose>),
		Without<Jake>,
	>,
	time: Res<Time<Fixed>>,
	last_bounce: ResMut<LastScreenBounce>,
	mut physical_event_timer: ResMut<PhysicalEventTimer>,
	mut jake_speed_xy: Local<(f32, f32)>,
	mut jake_desired_dir: Local<Direction>,
) {
	let physical_event_timer = &mut physical_event_timer.0;
	let (jake_entity, f_transform) = &mut *jake;

	let tweakbob: bool;
	let speed_range: Range<f32>;
	match *jake_physical_events {
		JakePhysicalEvents::Move { tweak } => {
			tweakbob = tweak;
			if tweak {
				speed_range = 200.0..400.0;
			}
			else {
				speed_range = 50.0..222.0;
			}
		}
		_ => return,
	};

	if physical_event_timer.just_finished() && !tweakbob {
		*jake_speed_xy = (
			rand::random_range(speed_range.clone()),
			rand::random_range(speed_range.clone()),
		);

		let desired_dir_x = if rand::random_bool(0.5) {
			DirX::Left
		}
		else {
			DirX::Right
		};

		let desired_dir_y = if rand::random_bool(0.5) {
			DirY::Up
		}
		else {
			DirY::Down
		};

		*jake_desired_dir = Direction::new(desired_dir_x, desired_dir_y);
	}
	// just do what we did above but without the timer's approval
	else if tweakbob {
		*jake_speed_xy = (
			rand::random_range(speed_range.clone()),
			rand::random_range(speed_range.clone()),
		);

		let desired_dir_x = if rand::random_bool(0.5) {
			DirX::Left
		}
		else {
			DirX::Right
		};

		let desired_dir_y = if rand::random_bool(0.5) {
			DirY::Up
		}
		else {
			DirY::Down
		};

		*jake_desired_dir = Direction::new(desired_dir_x, desired_dir_y);
	}

	if last_bounce.0.0 == jake_desired_dir.0 {
		jake_desired_dir.0 = jake_desired_dir.0.opposite();
	}

	if last_bounce.0.1 == jake_desired_dir.1 {
		jake_desired_dir.1 = jake_desired_dir.1.opposite();
	}

	// flipping logic, cause just doing flip_x on the sprites dont work
	if jake_desired_dir.0 == DirX::Left {
		if let Ok(jake_children) = children_query.get(*jake_entity) {
			for child in jake_children.iter() {
				let Ok((_sprite, mut transform, nose)) = sprites_and_transforms.get_mut(child)
				else {
					continue;
				};

				transform.scale_mut().x = -1.0;
				if nose.is_some() {
					// god damn it i hate that the nose' center in the sprite is actually the center
					// and not the base of the nose ughh
					*transform.translation_mut() = Vec3::new(21.0, 4.0, 0.0);
				}
			}
		}
	}
	else {
		if let Ok(jake_children) = children_query.get(*jake_entity) {
			for child in jake_children.iter() {
				let Ok((_sprite, mut transform, nose)) = sprites_and_transforms.get_mut(child)
				else {
					continue;
				};

				transform.scale_mut().x = 1.0;
				if nose.is_some() {
					// put it back
					*transform.translation_mut() = Vec3::new(-25.0, 4.0, 0.0);
				}
			}
		}
	}

	// move that boy
	let dir = Direction::new(jake_desired_dir.0, jake_desired_dir.1);
	let difference = Vec3::new(
		dir.0 as i8 as f32 * jake_speed_xy.0,
		dir.1 as i8 as f32 * jake_speed_xy.1,
		0.0,
	);
	*f_transform.translation_mut() += difference * time.delta_secs();
}

pub fn jake_eyes_spin(
	mut jake_alt_events: ResMut<JakeAltEvents>,
	mut f_transform: Single<&mut FixedTransform, With<JakeEyes>>,
	mut spin_timer_opt: Local<Option<Timer>>,
	time: Res<Time<Fixed>>,
) {
	use JakeAltEvents as AltPhysEvents;
	if *jake_alt_events != AltPhysEvents::GlassesSpin {
		return;
	}

	let spin_timer = spin_timer_opt
		.get_or_insert_with(|| Timer::from_seconds(std::f32::consts::PI, TimerMode::Once));

	if spin_timer.tick(time.delta()).just_finished() {
		*f_transform.rotation_mut() = Quat::IDENTITY;
		spin_timer.reset();
		*jake_alt_events = JakeAltEvents::None;
	}
	else {
		f_transform.rotate(Quat::from_rotation_z(-10.40 * time.delta_secs()));
	}
}
