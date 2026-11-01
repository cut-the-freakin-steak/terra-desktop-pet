mod trans_window;

use bevy::prelude::*;
use bevy::window::{ExitCondition, Monitor, OnMonitor, PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;

use trans_window::TransparentWindow;

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
		.add_systems(Update, setup) // technically a setup schedule, but whatever
		.add_systems(Update, polly)
		.add_systems(Update, move_jake)
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

#[derive(Component)]
struct Direction(DirectionX, DirectionY);

#[derive(Clone, Copy)]
enum DirectionX {
	Left = -1,
	Right = 1,
}

#[derive(Clone, Copy)]
enum DirectionY {
	Down = -1,
	Up = 1,
}

#[derive(Resource)]
struct SetupComplete;

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	on_monitor_windows: Query<(&Window, &OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	complete: Option<Res<SetupComplete>>,
) {
	if complete.is_some() {
		return;
	}

	let Ok((_, on_monitor)) = on_monitor_windows.single()
	else {
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};

	commands.spawn(Camera2d);

	let jake_eyes = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_eyes.png"));
	let jake_nose = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_nose.png"));
	let jake_bush = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_bush.png"));
	let jake = Jake {
		eyes: jake_eyes.clone(),
		nose: jake_nose.clone(),
		bush: jake_bush.clone(),
		scale: 2.0,
	};

	commands
		.spawn((
			jake.clone(),
			Transform::default().with_scale(Vec3::new(jake.scale, jake.scale, 0.0)),
			Direction(DirectionX::Right, DirectionY::Up),
			GlobalTransform::default(),
			Visibility::default(),
			JakeIntroArc {
				start: Vec3::new(0.0, 100.0, 0.0),
				end: Vec3::new(-300.0, 30.0, 0.0),
				duration: 2.0,
				height: 150.0,
				elapsed: 0.0,
			},
		))
		.with_children(|parent| {
			parent.spawn((jake_eyes, Transform::from_xyz(-2.0, 13.0, 0.0)));
			parent.spawn((jake_nose, Transform::from_xyz(15.0, 4.0, 0.0)));
			parent.spawn((jake_bush, Transform::from_xyz(0.0, -20.0, 0.0)));
		});

	// this will no longer run
	commands.insert_resource(SetupComplete);
}

fn polly(
	jake_intro_arc: Single<&JakeIntroArc, With<Jake>>,
	mut transform: Single<&mut Transform, With<Jake>>,
) {
	let t = (jake_intro_arc.elapsed / jake_intro_arc.duration).clamp(0.0, 1.0);

	let mut position = jake_intro_arc.start.lerp(jake_intro_arc.end, t);
	position.y += 4.0 * jake_intro_arc.height * t * (1.0 - t);

	transform.translation = position;
}

#[derive(Component, Clone, Copy)]
struct JakeIntroArc {
	start: Vec3,
	end: Vec3,
	duration: f32,
	height: f32,
	elapsed: f32,
}

fn jake_intro(
	mut jake: Single<(&mut Jake, &mut Transform, &GlobalTransform, &mut Direction)>,
	on_monitor_windows: Query<(&Window, &bevy::window::OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	time: Res<Time>,
) {
	let (jake, transform, g_transform, direction) = &mut *jake;

	let Ok((_, on_monitor)) = on_monitor_windows.single()
	else {
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};
}

fn move_jake(
	mut jake: Single<(&Jake, &mut Transform, &GlobalTransform, &mut Direction)>,
	on_monitor_windows: Query<(&Window, &bevy::window::OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	time: Res<Time>,
) {
	// 	let (jake, transform, g_transform, direction) = &mut *jake;
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
