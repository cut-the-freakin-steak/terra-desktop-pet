use bevy::color::palettes::basic::PURPLE;
use bevy::prelude::*;
use bevy::window::{
	CursorOptions, Monitor, PrimaryMonitor, PrimaryWindow, WindowLevel, WindowResolution,
};

#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;

fn main() -> AppExit {
	let window = Window {
		resizable: false,
		focused: false,
		decorations: false,
		transparent: true,

		#[cfg(target_os = "macos")]
		composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
		#[cfg(target_os = "linux")]
		composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

		window_level: WindowLevel::AlwaysOnTop,
		resolution: WindowResolution::new(1, 1),
		position: WindowPosition::Automatic,
		..default()
	};

	return App::new()
		.insert_resource(ClearColor(Color::NONE))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(window),
			..default()
		}))
		.add_systems(Startup, setup)
		.run();
}

fn setup(
	mut cursor_options: Single<&mut CursorOptions>,
	mut windows: Query<&mut Window, With<PrimaryWindow>>,
	monitors: Query<&Monitor, With<PrimaryMonitor>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let Ok(mut window) = windows.single_mut()
	else {
		return;
	};

	let Ok(monitor) = monitors.single()
	else {
		return;
	};

	window
		.resolution
		.set_physical_resolution(monitor.physical_width, monitor.physical_height);

	window.position = WindowPosition::At(monitor.physical_position);

	cursor_options.hit_test = false;

	commands.spawn(Camera2d);
	commands.spawn((
		Mesh2d(meshes.add(Rectangle::default())),
		MeshMaterial2d(materials.add(Color::from(PURPLE))),
		Transform::default().with_scale(Vec3::splat(128.0)),
	));
}
