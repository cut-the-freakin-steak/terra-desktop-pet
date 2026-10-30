use bevy::prelude::*;
use bevy::window::{Monitor, OnMonitor, PrimaryWindow};

/// makes the primary window transparent, useful for a desktop pet type app
///
/// IMPORTANT!!! make sure to set the `ExitCondition` for your `WindowPlugin` as
/// `ExitCondition::DontExit` **before** adding this plugin
pub struct TransparentWindow {
	title: String,
	name: Option<String>,
}

#[derive(Resource)]
struct TWindowConfig {
	pub title: String,
	pub name: Option<String>,
}

impl Plugin for TransparentWindow {
	fn build(&self, app: &mut App) {
		assert!(
			app.is_plugin_added::<WindowPlugin>(),
			"the TransparentWindow plugin depends on WindowPlugin. please add WindowPlugin or DefaultPlugins"
		);

		app.insert_resource(ClearColor(Color::NONE))
			.insert_resource(TWindowConfig {
				title: self.title.clone(),
				name: self.name.clone(),
			})
			.init_resource::<ReopenWindow>()
			.add_systems(Startup, window_setup)
			.add_systems(Update, (close_window, reopen_window));
	}
}

#[derive(Resource, Default)]
struct ReopenWindow {
	pub close_requested: bool,
	pub reopen_requested: bool,
	pub monitor: Option<Entity>,
	pub monitor_res: (u32, u32),
	pub monitor_scale: f64,
}

impl TransparentWindow {
	pub fn new(title: String, name: Option<String>) -> Self {
		return TransparentWindow { title, name };
	}
}

fn window_setup(mut window_state: ResMut<ReopenWindow>) {
	window_state.close_requested = true;
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
		return;
	};

	let Ok((_, on_monitor)) = on_monitor_windows.single_mut()
	else {
		return;
	};

	let Ok(monitor) = monitors.get(on_monitor.0)
	else {
		return;
	};

	// let width = monitor.physical_width;
	// let height = monitor.physical_height;

	let width = monitor.physical_width;
	let height = monitor.physical_height;

	commands.entity(window_entity).despawn();

	state.close_requested = false;
	state.reopen_requested = true;
	state.monitor = Some(on_monitor.0);
	state.monitor_res = (width, height);
	state.monitor_scale = monitor.scale_factor;
}

fn reopen_window(
	mut commands: Commands,
	mut state: ResMut<ReopenWindow>,
	windows: Query<(), With<PrimaryWindow>>,
	config: Res<TWindowConfig>,
) {
	#[cfg(any(target_os = "macos", target_os = "linux"))]
	use bevy::window::CompositeAlphaMode;
	use bevy::window::{CursorOptions, WindowLevel, WindowResolution};

	if !state.reopen_requested || !windows.is_empty() {
		return;
	}

	let Some(monitor_entity) = &state.monitor
	else {
		return;
	};

	let window = Window {
		title: config.title.clone(),
		name: config.name.clone(),

		resizable: false,
		focused: false,
		decorations: false,
		transparent: true,

		#[cfg(target_os = "macos")]
		composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
		#[cfg(target_os = "linux")]
		composite_alpha_mode: CompositeAlphaMode::PreMultiplied,

		window_level: WindowLevel::AlwaysOnTop,
		position: WindowPosition::Centered(MonitorSelection::Entity(*monitor_entity)),
		resolution: WindowResolution::from(state.monitor_res)
			.with_scale_factor_override(state.monitor_scale as f32),

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

	info!("successfully made window transparent")
}
