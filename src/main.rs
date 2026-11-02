mod jake_events;
mod trans_window;

use bevy::prelude::*;
use bevy::window::{ExitCondition, Monitor, OnMonitor, PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;

use trans_window::TransparentWindow;

use jake_events::jake_events;

static JAKE_WIDTH_ISH: f32 = 65.0;

fn main() -> AppExit {
	let transparent_window_plugin =
		TransparentWindow::new("jaketop".to_string(), Some("jaketop".to_string()));

	return App::new()
		.insert_resource(ClearColor(Color::NONE))
		.add_plugins((
			EmbeddedAssetPlugin::default(),
			DefaultPlugins
				.set(WindowPlugin {
					exit_condition: ExitCondition::DontExit,
					..default()
				})
				.set(ImagePlugin::default_nearest()),
			transparent_window_plugin,
		))
		.add_systems(Update, setup) // technically a Setup schedule, but whatever
		.add_systems(FixedUpdate, jake_intro)
		.add_systems(FixedUpdate, jake_squash_n_stretch)
		.add_systems(FixedUpdate, jake_events)
		.run();
}

#[allow(dead_code)]
#[derive(Component, Clone)]
struct Jake {
	pub eyes: Sprite,
	pub nose: Sprite,
	pub bush: Sprite,
	pub scale: f32,
}

#[derive(Resource)]
struct SetupComplete;

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut on_monitor_windows: Query<(&mut Window, &OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	complete: Option<Res<SetupComplete>>,
) {
	if complete.is_some() {
		return;
	}

	let Ok((mut window, on_monitor)) = on_monitor_windows.single_mut()
	else {
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};

	window.present_mode = bevy::window::PresentMode::AutoVsync;

	let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;

	let jake_eyes = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_eyes.png"));
	let jake_nose = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_nose.png"));
	let jake_bush = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_bush.png"));
	let jake = Jake {
		eyes: jake_eyes.clone(),
		nose: jake_nose.clone(),
		bush: jake_bush.clone(),
		scale: 2.0,
	};

	let jake_starting_pos = (-half_monitor_width - JAKE_WIDTH_ISH as f64 - 25.0) as f32; // cookin up sum bs

	commands.spawn(Camera2d);
	commands
		.spawn((
			jake.clone(),
			Transform::from_xyz(jake_starting_pos, 0.0, 0.0)
				.with_scale(Vec3::new(jake.scale, jake.scale, 0.0)),
			GlobalTransform::default(),
			Visibility::default(),
			bevy::sprite::Anchor::CENTER,
			JakeIntroArc {
				start: Vec3::new(jake_starting_pos, 0.0, 0.0),
				end: Vec3::new(
					jake_starting_pos + 150.0 * monitor.scale_factor as f32,
					0.0,
					0.0,
				),
				duration: 1.5,
				peak_height: 250.0,
				elapsed: 0.0,
			},
			JakeSquashNStretch {
				duration: 1.0,
				intensity: 1.0,
				elapsed: 0.0,
			},
		))
		.with_children(|parent| {
			parent.spawn((jake_eyes, Transform::from_xyz(-2.0, 13.0, 0.0)));
			parent.spawn((jake_nose, Transform::from_xyz(15.0, 4.0, 0.0)));
			parent.spawn((jake_bush, Transform::from_xyz(0.0, -20.0, 0.0)));
		});

	commands.insert_resource(SetupComplete);
}

#[derive(Component, Clone, Copy)]
pub struct JakeIntroArc {
	start: Vec3,
	end: Vec3,
	peak_height: f32,
	duration: f32,
	elapsed: f32,
}

#[derive(Resource)]
struct JakeIntroComplete;

fn jake_intro(
	mut commands: Commands,
	mut jake: Single<(Entity, &mut Transform, &mut JakeIntroArc)>,
	time: Res<Time>,
	complete: Option<Res<JakeIntroComplete>>,
) {
	if complete.is_some() {
		return;
	}

	let (entity, transform, intro_arc) = &mut *jake;

	intro_arc.elapsed += time.delta_secs();
	let duration = (intro_arc.elapsed / intro_arc.duration).clamp(0.0, 1.0);

	let mut current = intro_arc.start.lerp(intro_arc.end, duration);

	// parabolic arc offset for y
	// formula: 4 * height * duration * (1 - duration)
	// evaluates to 0 at duration=0 and duration=1, and 'peak_height' at duration=0.5
	// it's just a quadratic lmao
	let arc_offset = 4.0 * intro_arc.peak_height * duration * (1.0 - duration);

	// combine linear height interpolation with the arc offset
	let base_y = intro_arc.start.y + (intro_arc.end.y - intro_arc.start.y) * duration;
	current.y = base_y + arc_offset;

	transform.translation = current;

	const MAGIC_NUMBER: f32 = -12.40; // genuinely no rationale behind this number. pure trial and error.
	transform.rotate(Quat::from_rotation_z(MAGIC_NUMBER * time.delta_secs()));

	// finishes animation
	if duration >= 1.0 {
		transform.rotation = Quat::from_array([0.0; 4]);
		commands.entity(*entity).remove::<JakeIntroArc>();
		commands.insert_resource(JakeIntroComplete);
	}
}

#[derive(Component, Clone, Copy)]
struct JakeSquashNStretch {
	pub duration: f32,
	pub intensity: f32, // how extreme the squash/stretch is
	pub elapsed: f32,
}

#[derive(Resource)]
pub struct JakeSquashNStretchCompleted;

fn jake_squash_n_stretch(
	mut commands: Commands,
	mut jake: Single<(Entity, &mut Transform, &mut JakeSquashNStretch, &mut Jake)>,
	time: Res<Time>,
	jake_intro_complete: Option<Res<JakeIntroComplete>>,
) {
	if jake_intro_complete.is_none() {
		return;
	}

	let (entity, transform, effect, jake) = &mut *jake;

	effect.elapsed += time.delta_secs();
	let t = (effect.elapsed / effect.duration).clamp(0.0, 1.0);

	// create a decaying oscillation using a sine wave multiplied by an exponential decay
	let decay = (1.0 - t).powi(2); // fades out as it finishes
	let wave = (t * std::f32::consts::TAU * 3.0).sin() * effect.intensity * decay;

	// squash on X/Z (wider) when stretching on Y (taller), and vice versa to preserve volume
	let scale_factor = 2.0; // Your base scale (e.g., jake.scale)
	// transform.scale.x = scale_factor * (1.0 - wave);
	transform.scale.y = scale_factor * (1.0 - wave);

	// remove the component when the effect finishes
	if t >= 1.0 {
		// reset to default base scale
		transform.scale = Vec3::splat(scale_factor);
		commands.entity(*entity).remove::<JakeSquashNStretch>();
		commands.insert_resource(JakeSquashNStretchCompleted);
	}
}

fn jake_events_prev(//
	// mut jake: Single<(&Jake, &mut Transform, &GlobalTransform)>,
	// on_monitor_windows: Query<(&Window, &bevy::window::OnMonitor), With<PrimaryWindow>>,
	// monitors: Query<&Monitor>,
	// time: Res<Time>,
) {
	// 	let (jake, transform, g_transform) = &mut *jake;
	//
	// 	let Ok((_, on_monitor)) = on_monitor_windows.single()
	// 	else {
	// 		return;
	// 	};
	//
	// 	let Ok(monitor) = monitors.get(on_monitor.0)
	// 	else {
	// 		return;
	// 	};
	//
	// 	let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;
	// 	let half_monitor_height = (monitor.physical_height as f64 / monitor.scale_factor) / 2.0;
	// 	// right side of monitor
	// 	if half_monitor_width
	// 		<= (g_transform.translation().x + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
	// 	{
	// 		direction.0 = DirectionX::Left;
	// 	}
	// 	// left side of monitor
	// 	else if -half_monitor_width
	// 		>= (g_transform.translation().x - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
	// 	{
	// 		direction.0 = DirectionX::Right;
	// 	}
	// 	// top of monitor
	// 	if half_monitor_height
	// 		<= (g_transform.translation().y + (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
	// 	{
	// 		direction.1 = DirectionY::Down;
	// 	}
	// 	// bottom of monitor
	// 	else if -half_monitor_height
	// 		>= (g_transform.translation().y - (JAKE_WIDTH_ISH * jake.scale) / 2.0) as f64
	// 	{
	// 		direction.1 = DirectionY::Up;
	// 	}
	//
	// 	match direction.0 {
	// 		DirectionX::Left => transform.rotate(Quat::from_rotation_z(0.05)),
	// 		DirectionX::Right => transform.rotate(Quat::from_rotation_z(-0.05)),
	// 	}
	//
	// 	const SPEED: f32 = 250.0;
	// 	transform.translation.x += direction.0 as i8 as f32 * SPEED * time.delta_secs();
	// 	transform.translation.y += direction.1 as i8 as f32 * SPEED * time.delta_secs();
}
