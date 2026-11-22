#![feature(str_split_remainder)]

mod jake_events;
mod movable_helpers;
mod trans_window;
mod tray_icon;

use std::io::{BufRead, Write};

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::{ExitCondition, Monitor, OnMonitor, PrimaryMonitor, PrimaryWindow};
use bevy::winit::WinitWindows;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use trans_window::TransparentWindow;

use jake_events::*;
use movable_helpers::*;
use tray_icon::*;

use crate::trans_window::WindowRecreated;

const JAKE_BASE_SCALE: f32 = 2.0;
const JAKE_BASE_WIDTH_ISH: f32 = 65.0;

pub static APP_DIR: std::sync::LazyLock<std::path::PathBuf> = std::sync::LazyLock::new(|| {
	let base_dir = dirs::data_dir().expect("couldn't get user data dir");
	const APP_NAME: &str = "jAkEtop";

	#[cfg(not(target_os = "macos"))]
	let app_dir = base_dir.join(APP_NAME);

	#[cfg(target_os = "macos")]
	let app_dir = base_dir.join(format!("com.steak.{APP_NAME}"));

	std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");

	app_dir
});

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
	#[default]
	Setup,
	Elsewhere,
}

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JakeState {
	#[default]
	NotStarted,
	Intro,
	SquashAndStretch,
	Roaming,
	ChasingMouse {
		toggled: bool,
	},
}

fn main() -> AppExit {
	let transparent_window_plugin =
		TransparentWindow::new("jAkEtop".to_string(), Some("jAkEtop".to_string()));

	return App::new()
		.insert_resource(ClearColor(Color::NONE))
		.init_non_send::<WinitWindows>()
		.init_resource::<JakePhysicalEvents>()
		.init_resource::<JakeSoundEvents>()
		.init_resource::<JakeAltEvents>()
		.init_resource::<GlobalCursorPos>()
		.insert_resource(JakeChaseSpeed(0.05))
		.insert_resource(LastScreenBounce(Direction::default()))
		.insert_resource(PhysicalEventTimer(Timer::from_seconds(
			2.0,
			TimerMode::Repeating,
		)))
		.insert_resource(SoundEventTimer(Timer::from_seconds(
			10.0,
			TimerMode::Repeating,
		)))
		.insert_resource(AltEventTimer(Timer::from_seconds(
			15.0,
			TimerMode::Repeating,
		)))
		.insert_resource(JakeWidth(JAKE_BASE_WIDTH_ISH))
		.add_plugins((
			EmbeddedAssetPlugin::default(),
			DefaultPlugins
				.set(WindowPlugin {
					exit_condition: ExitCondition::DontExit,
					..default()
				})
				.set(ImagePlugin::default_nearest()),
			transparent_window_plugin,
			PositionPlugin,
		))
		.init_state::<AppState>()
		.init_state::<JakeState>()
		.add_systems(
			Update,
			(
				setup_game.run_if(in_state(AppState::Setup)),
				setup_jake_stuff.run_if(in_state(AppState::Setup)),
			)
				.chain(),
		) // technically a Setup schedule, but whatever
		.add_systems(Startup, setup_tray)
		.add_systems(Update, set_window_icon)
		// intro + events
		.add_systems(
			FixedUpdate,
			(
				track_global_cursor,
				jake_intro.run_if(in_state(JakeState::Intro)),
				jake_squash_n_stretch.run_if(in_state(JakeState::SquashAndStretch)),
				jake_events.run_if(in_state(JakeState::Roaming)),
				jake_move.run_if(in_state(JakeState::Roaming)),
				jake_chase_mouse.run_if(is_chasing_mouse),
			)
				.run_if(in_state(AppState::Elsewhere)),
		)
		// alt events
		.add_systems(
			FixedUpdate,
			jake_eyes_spin.run_if(in_state(JakeState::Roaming)),
		)
		.add_systems(Update, handle_tray_events)
		.run();
}

fn is_chasing_mouse(state: Res<State<JakeState>>) -> bool {
	return matches!(state.get(), JakeState::ChasingMouse { .. });
}

#[derive(Resource)]
pub struct UsingMonitor(Monitor);

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct Jake;

#[derive(Resource, Clone, Copy, PartialEq)]
pub struct JakeWidth(pub f32);

#[derive(Component)]
pub struct JakeEyes;

#[derive(Component)]
pub struct JakeNose;

#[derive(Component)]
pub struct JakeBush;

fn setup_game(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut on_monitor_windows: Query<(&mut Window, &OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	jake_width_ish: Res<JakeWidth>,
) {
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
	let jake_width_ish = jake_width_ish.0;

	let jake_eyes_sprite =
		Sprite::from_image(asset_server.load("embedded://sprites/jAkE_eyes.png"));
	let jake_nose_sprite =
		Sprite::from_image(asset_server.load("embedded://sprites/jAkE_nose.png"));
	let jake_bush_sprite =
		Sprite::from_image(asset_server.load("embedded://sprites/jAkE_bush.png"));

	// cookin up sum bs
	let jake_starting_pos_x = -half_monitor_width as f32 - jake_width_ish - 25.0;
	let magic_number = if rand::random_bool(0.90) {
		-12.40 // genuinely no rationale behind this number. pure trial and error.
	}
	else {
		-112.40 // lmao.
	};

	commands.spawn(Camera2d);
	commands
		.spawn((
			Jake,
			JakeIntroArc {
				start: Vec3::new(jake_starting_pos_x, 0.0, 0.0),
				end: Vec3::new(jake_starting_pos_x + 150.0 * JAKE_BASE_SCALE, 0.0, 0.0),
				duration: 1.5,
				peak_height: 250.0,
				elapsed: 0.0,
				magic_rotation_number: magic_number,
			},
			JakeSquashNStretch {
				duration: 1.0,
				intensity: rand::random_range(0.1..=20.0),
				elapsed: 0.0,
			},
			Transform::from_xyz(jake_starting_pos_x, 0.0, 0.0)
				.with_scale(Vec3::new(JAKE_BASE_SCALE, JAKE_BASE_SCALE, 0.0))
				.into_fixed_bundle(),
			GlobalTransform::default(),
			Visibility::default(),
			Direction::new(DirX::Right, DirY::Up),
		))
		.with_children(|parent| {
			parent.spawn((
				JakeNose,
				jake_nose_sprite,
				Transform::from_xyz(-25.0, 4.0, 1.0).into_fixed_bundle(),
				Anchor::CENTER_LEFT,
			));
			parent.spawn((
				JakeEyes,
				jake_eyes_sprite,
				Transform::from_xyz(-2.0, 13.0, 0.0).into_fixed_bundle(),
			));
			parent.spawn((
				JakeBush,
				jake_bush_sprite,
				Transform::from_xyz(0.0, -20.0, 0.0).into_fixed_bundle(),
			));
		});

	commands.insert_resource(UsingMonitor(monitor.clone()));
}

// this function is such a mess, but i love the .jake file extension,,,
// note to self: if any user dares touch this file, kill them with your own hands
fn setup_jake_stuff(
	mut jake: Single<&mut FixedTransform, With<Jake>>,
	mut global_volume: ResMut<GlobalVolume>,
	mut next_app_state: ResMut<NextState<AppState>>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let f_transform = &mut *jake;

	if !APP_DIR.join("jAkE_data_stuff.jake").exists() {
		let mut data_file = std::fs::OpenOptions::new()
			.read(true)
			.write(true)
			.append(false)
			.create(true)
			.open(APP_DIR.join("jAkE_data_stuff.jake"))
			.expect(&format!(
				"could not open file: {:?}",
				APP_DIR.join("jAkE_data_stuff.jake")
			));

		let jake_scale = f_transform.scale().x;
		let app_volume = global_volume.volume.to_linear();
		let _ = data_file.write_all(
			format!(
				concat!(
					"DO NOT MANUALLY EDIT THIS FILE UNLESS YOU KNOW WHAT YOU'RE DOING, OR I WILL KILL YOU.\n",
					"jake_scale = {}\n",
					"app_volume = {}"
				),
				jake_scale, app_volume
			)
			.as_bytes(),
		);
	}

	let data_file = std::fs::OpenOptions::new()
		.read(true)
		.write(false)
		.append(false)
		.create(false)
		.open(APP_DIR.join("jAkE_data_stuff.jake"))
		.expect("couldn't open jake data file on startup");

	let reader = std::io::BufReader::new(data_file);
	let mut line_reader = reader.lines();
	// first line, skip 'cause it's a header
	_ = line_reader.next();

	let line = line_reader.next().unwrap().unwrap();
	info!("line: {line}");
	let mut split_line = line.split("=");
	split_line.next();
	let jake_scale = split_line
		.remainder()
		.expect("should be something after the = sign")
		.trim()
		.parse::<f32>()
		.expect("couldn't make string into a scale f32");

	let prev_scale_z = f_transform.scale_mut().z;
	*f_transform.scale_mut() = Vec3 {
		x: jake_scale,
		y: jake_scale,
		z: prev_scale_z,
	};

	let line = line_reader.next().unwrap().unwrap();
	info!("line: {line}");
	let mut split_line = line.split("=");
	split_line.next();
	let file_volume = split_line
		.remainder()
		.expect("should be something after the = sign")
		.trim()
		.parse::<f32>()
		.expect("couldn't make string into a volume f32");

	global_volume.volume = bevy::audio::Volume::Linear(file_volume);

	next_app_state.set(AppState::Elsewhere);
	next_jake_state.set(JakeState::Intro);
}

#[derive(Resource)]
struct WindowIconSet;

// probably entirely unnecessary because its a borderless frameless window, but whatever lmao
fn set_window_icon(
	mut commands: Commands,
	windows: NonSend<WinitWindows>,
	window_recreated: Option<Res<WindowRecreated>>,
	icon_set: Option<Res<WindowIconSet>>,
) {
	if icon_set.is_some() || window_recreated.is_none() {
		return;
	}

	let (icon_rgba, icon_width, icon_height) = {
		let png = include_bytes!("../assets/jAkE_app_icon.png");
		let image = image::load_from_memory(png)
			.expect("failed to open icon path")
			.into_rgba8();

		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

	for window in windows.windows.values() {
		window.set_window_icon(Some(icon.clone()));
	}

	commands.insert_resource(WindowIconSet);
}

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct GlobalCursorPos(pub Vec2);

/// stupid goddamn click-through window with its inability to track mouse cursor position,,,
/// much of this code i don't understand because 1. im tired and 2. its copied from various forums lmao
fn track_global_cursor(
	window: Single<&Window, With<PrimaryWindow>>,
	monitors: Query<(&Monitor, Has<PrimaryMonitor>)>,
	mut global_cursor: ResMut<GlobalCursorPos>,
) {
	use mouse_coords::Point;

	if let Ok(Point { x, y }) = mouse_coords::get_position() {
		let raw = Vec2::new(x as f32, y as f32);
		let window_phys_size = window.physical_size().as_vec2();

		// identify main monitor where the PrimaryWindow lives
		let main_monitor = monitors
			.iter()
			.find(|(_, is_primary)| *is_primary)
			.or_else(|| {
				let win_aspect = window_phys_size.x / window_phys_size.y;
				monitors.iter().find(|(m, _)| {
					let mon_aspect = m.physical_width as f32 / m.physical_height as f32;
					(win_aspect - mon_aspect).abs() < 0.01
				})
			})
			.or_else(|| monitors.iter().next())
			.map(|(m, _)| m);

		if let Some(monitor) = main_monitor {
			let monitor_phys_size = Vec2::new(
				monitor.physical_width as f32,
				monitor.physical_height as f32,
			);

			let true_logical = (monitor_phys_size * monitor_phys_size) / window_phys_size;
			let reported_logical = monitor_phys_size / monitor.scale_factor as f32;
			let scale_down = reported_logical / true_logical;

			// subtract main monitor top-left origin in logical desktop space
			let actual_scale = monitor_phys_size / true_logical;
			let mon_origin_logical = monitor.physical_position.as_vec2() / actual_scale;
			let local_raw = raw - mon_origin_logical;

			// stop updating GlobalCursorPos once the cursor leaves main monitor's bounds
			let is_inside_main_monitor = local_raw.x >= 0.0
				&& local_raw.x <= true_logical.x
				&& local_raw.y >= 0.0
				&& local_raw.y <= true_logical.y;

			if is_inside_main_monitor {
				global_cursor.0 = Vec2::new(
					(local_raw.x - (true_logical.x * 0.5)) * scale_down.x,
					((true_logical.y * 0.5) - local_raw.y) * scale_down.y,
				);
			}
		}
	}
}
