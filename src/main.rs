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
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands.spawn(Camera2d);
	commands.spawn((
		Mesh2d(meshes.add(Rectangle::default())),
		MeshMaterial2d(materials.add(Color::from(PURPLE))),
		Transform::default().with_scale(Vec3::splat(128.0)),
	));
}
