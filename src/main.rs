mod trans_window;

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use trans_window::TransparentWindow;

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
		.add_systems(Startup, setup)
		.add_systems(Update, move_purple_square_left_and_right)
		.run();
}

#[allow(dead_code)]
#[derive(Component)]
struct Jake {
	pub eyes: Sprite,
	pub nose: Sprite,
	pub bush: Sprite,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn(Camera2d);

	let jake_eyes = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_eyes.png"));
	let jake_nose = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_nose.png"));
	let jake_bush = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_bush.png"));

	commands
		.spawn((
			Jake {
				eyes: jake_eyes.clone(),
				nose: jake_nose.clone(),
				bush: jake_bush.clone(),
			},
			Transform::default().with_scale(Vec3::new(3.0, 3.0, 0.0)),
			Direction(DirectionX::Right, DirectionY::Up),
			GlobalTransform::default(),
		))
		.with_children(|parent| {
			parent.spawn((jake_eyes, Transform::from_xyz(-2.0, 13.0, 0.0)));
			parent.spawn((jake_nose, Transform::from_xyz(15.0, 4.0, 0.0)));
			parent.spawn((jake_bush, Transform::from_xyz(0.0, -20.0, 0.0)));
		});
}

fn move_purple_square_left_and_right(
	mut jake: Single<(&mut Transform, &GlobalTransform, &mut Direction), With<Jake>>,
	on_monitor_windows: Query<
		(&Window, &bevy::window::OnMonitor),
		With<bevy::window::PrimaryWindow>,
	>,
	monitors: Query<&bevy::window::Monitor>,
	time: Res<Time>,
) {
	let (transform, g_transform, direction) = &mut *jake;

	let Ok((_, on_monitor)) = on_monitor_windows.single()
	else {
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};

	let half_monitor_width = (monitor.physical_width as f64 / monitor.scale_factor) / 2.0;
	let half_monitor_height = (monitor.physical_height as f64 / monitor.scale_factor) / 2.0;
	// right side of monitor
	if half_monitor_width
		<= (g_transform.translation().x + (transform.scale.x + 180.0) / 2.0) as f64
	{
		direction.0 = DirectionX::Left;
	}
	// left side of monitor
	else if -half_monitor_width
		>= (g_transform.translation().x - (transform.scale.x + 180.0) / 2.0) as f64
	{
		direction.0 = DirectionX::Right;
	}
	// top of monitor
	if half_monitor_height
		<= (g_transform.translation().y + (transform.scale.y + 180.0) / 2.0) as f64
	{
		direction.1 = DirectionY::Down;
	}
	// bottom of monitor
	else if -half_monitor_height
		>= (g_transform.translation().y - (transform.scale.y + 180.0) / 2.0) as f64
	{
		direction.1 = DirectionY::Up;
	}

	match direction.0 {
		DirectionX::Left => transform.rotate(Quat::from_rotation_z(0.05)),
		DirectionX::Right => transform.rotate(Quat::from_rotation_z(-0.05)),
	}

	const SPEED: f32 = 250.0;
	transform.translation.x += direction.0 as i8 as f32 * SPEED * time.delta_secs();
	transform.translation.y += direction.1 as i8 as f32 * SPEED * time.delta_secs();
}
