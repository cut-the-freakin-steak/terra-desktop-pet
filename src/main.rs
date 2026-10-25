mod trans_window;

use bevy::color::palettes::basic::PURPLE;
use bevy::prelude::*;
use bevy::window::ExitCondition;

use trans_window::TransparentWindow;

fn main() -> AppExit {
	let transparent_window_plugin =
		TransparentWindow::new("jaketop".to_string(), Some("jaketop".to_string()));

	return App::new()
		.insert_resource(ClearColor(Color::NONE))
		.add_plugins((
			DefaultPlugins.set(WindowPlugin {
				exit_condition: ExitCondition::DontExit,
				..default()
			}),
			transparent_window_plugin,
		))
		.add_systems(Startup, setup)
		.add_systems(Update, move_purple_square_left_and_right)
		.run();
}

#[derive(Component)]
struct PurpleRect;

#[derive(Component)]
struct Direction(f32);

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(Camera2d);
	commands.spawn((
		PurpleRect,
		Mesh2d(meshes.add(Rectangle::default())),
		MeshMaterial2d(materials.add(Color::from(PURPLE))),
		Transform::default().with_scale(Vec3::new(128.0, 128.0, 0.0)),
		Direction(1.0),
		GlobalTransform::default(),
	));
}

fn move_purple_square_left_and_right(
	mut purple_rect: Single<(&mut Transform, &GlobalTransform, &mut Direction), With<PurpleRect>>,
	on_monitor_windows: Query<
		(&Window, &bevy::window::OnMonitor),
		With<bevy::window::PrimaryWindow>,
	>,
	monitors: Query<&bevy::window::Monitor>,
	time: Res<Time>,
) {
	let (transform, g_transform, direction) = &mut *purple_rect;

	let Ok((_, on_monitor)) = on_monitor_windows.single()
	else {
		return;
	};
	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};

	let half_monitor_width = (monitor.physical_width as f64 * monitor.scale_factor) / 2.0;
	if (half_monitor_width <= (g_transform.translation().x + transform.scale.x / 2.0) as f64)
		|| (-half_monitor_width >= (g_transform.translation().x - transform.scale.x / 2.0) as f64)
	{
		direction.0 *= -1.0;
	}

	const SPEED: f32 = 1000.0;
	transform.translation.x += direction.0 * SPEED * time.delta_secs();
}
