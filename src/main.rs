use bevy::color::palettes::basic::PURPLE;
use bevy::prelude::*;
use bevy::window::{
	CursorOptions, ExitCondition, Monitor, OnMonitor, PrimaryMonitor, PrimaryWindow, WindowLevel,
	WindowResolution,
};

#[cfg(any(target_os = "macos", target_os = "linux"))]
use bevy::window::CompositeAlphaMode;

fn main() -> AppExit {
	let window = Window {
		title: "desktop-pet".to_string(),
		name: Some("desktop-pet".to_string()),

		resizable: false,
		focused: false,
		decorations: false,
		transparent: true,

		#[cfg(target_os = "macos")]
		composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
		#[cfg(target_os = "linux")]
		composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

		window_level: WindowLevel::AlwaysOnTop,
		position: WindowPosition::Automatic,
		..default()
	};

	return App::new()
		.insert_resource(ClearColor(Color::NONE))
		.init_resource::<ReopenWindow>()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(window),
			exit_condition: ExitCondition::DontExit,
			..default()
		}))
		.add_systems(Startup, setup)
		.add_systems(Update, (close_window, reopen_window))
		// .add_systems(Update, setup_window)
		.run();
}

fn setup(
	mut cursor_options: Single<&mut CursorOptions>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut window_state: ResMut<ReopenWindow>,
) {
	cursor_options.hit_test = false;

	commands.spawn(Camera2d);
	commands.spawn((
		Mesh2d(meshes.add(Rectangle::default())),
		MeshMaterial2d(materials.add(Color::from(PURPLE))),
		Transform::default().with_scale(Vec3::splat(128.0)),
	));

	window_state.close_requested = true;
}

#[derive(Resource, Default)]
struct ReopenWindow {
	pub close_requested: bool,
	pub reopen_requested: bool,
	pub monitor_res: (u32, u32),
}

fn close_window(
	mut commands: Commands,
	mut state: ResMut<ReopenWindow>,
	mut on_monitor_windows: Query<(&mut Window, &OnMonitor), With<PrimaryWindow>>,
	windows: Query<Entity, With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
) {
	if !state.close_requested {
		return;
	}

	let Ok(window_entity) = windows.single()
	else {
		info!("why");
		return;
	};

	let Ok((_, on_monitor)) = on_monitor_windows.single_mut()
	else {
		info!("FREAK!");
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		info!("FREAK! 2!");
		return;
	};

	let width = monitor.physical_width;
	let height = monitor.physical_height;

	commands.entity(window_entity).despawn();
	state.close_requested = false;
	state.reopen_requested = true;
	state.monitor_res = (width, height);
	info!("close success!")
}

fn reopen_window(
	mut commands: Commands,
	mut state: ResMut<ReopenWindow>,
	windows: Query<(), With<PrimaryWindow>>,
) {
	if !state.reopen_requested || !windows.is_empty() {
		return;
	}

	let window = Window {
		title: "desktop-pet".to_string(),
		name: Some("desktop-pet".to_string()),

		resizable: false,
		focused: false,
		decorations: false,
		transparent: true,

		#[cfg(target_os = "macos")]
		composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
		#[cfg(target_os = "linux")]
		composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

		window_level: WindowLevel::AlwaysOnTop,
		resolution: WindowResolution::from(state.monitor_res),
		..default()
	};

	commands.spawn((
		window,
		PrimaryWindow,
		CursorOptions {
			hit_test: false,
			..default()
		},
	));

	state.reopen_requested = false;
	info!("success!!")
}
