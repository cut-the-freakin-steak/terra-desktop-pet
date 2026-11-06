mod jake_events;
mod movable_helpers;
mod trans_window;

use bevy::prelude::*;
use bevy::window::{ExitCondition, Monitor, OnMonitor, PrimaryWindow};
use bevy_embedded_assets::EmbeddedAssetPlugin;

use trans_window::TransparentWindow;

use jake_events::*;
use movable_helpers::*;

const JAKE_WIDTH_ISH: f32 = 65.0;

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
}

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
			PositionPlugin,
		))
		.init_state::<AppState>()
		.init_state::<JakeState>()
		.add_systems(Update, setup_game.run_if(in_state(AppState::Setup))) // technically a Setup schedule, but whatever
		.add_systems(
			FixedUpdate,
			(
				jake_intro.run_if(in_state(JakeState::Intro)),
				jake_squash_n_stretch.run_if(in_state(JakeState::SquashAndStretch)),
				jake_events.run_if(in_state(JakeState::Roaming)),
			)
				.run_if(in_state(AppState::Elsewhere)),
		)
		.run();
}

#[derive(Resource)]
pub struct UsingMonitor(Monitor);

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct Jake {
	pub eyes: Sprite,
	pub nose: Sprite,
	pub bush: Sprite,
	pub scale: f32,
}

fn setup_game(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut on_monitor_windows: Query<(&mut Window, &OnMonitor), With<PrimaryWindow>>,
	monitors: Query<&Monitor>,
	mut next_app_state: ResMut<NextState<AppState>>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
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

	let jake_eyes = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_eyes.png"));
	let jake_nose = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_nose.png"));
	let jake_bush = Sprite::from_image(asset_server.load("embedded://sprites/jAkE_bush.png"));
	let jake = Jake {
		eyes: jake_eyes.clone(),
		nose: jake_nose.clone(),
		bush: jake_bush.clone(),
		scale: 2.0,
	};

	// cookin up sum bs
	let jake_starting_pos_x = -half_monitor_width as f32 - JAKE_WIDTH_ISH - 25.0;

	commands.spawn(Camera2d);
	commands
		.spawn((
			jake.clone(),
			JakeIntroArc {
				start: Vec3::new(jake_starting_pos_x, 0.0, 0.0),
				end: Vec3::new(jake_starting_pos_x + 150.0 * jake.scale, 0.0, 0.0),
				duration: 1.5,
				peak_height: 250.0,
				elapsed: 0.0,
			},
			JakeSquashNStretch {
				duration: 1.0,
				intensity: rand::random_range(0.1..=20.0),
				elapsed: 0.0,
			},
			Position::new(Vec3::new(jake_starting_pos_x, 0.0, 0.0)),
			Transform::from_xyz(jake_starting_pos_x, 0.0, 0.0)
				.with_scale(Vec3::new(jake.scale, jake.scale, 0.0)),
			GlobalTransform::default(),
			Visibility::default(),
			Direction(DirX::Right, DirY::Up),
		))
		.with_children(|parent| {
			parent.spawn((jake_eyes, Transform::from_xyz(-2.0, 13.0, 0.0)));
			parent.spawn((jake_nose, Transform::from_xyz(15.0, 4.0, 0.0)));
			parent.spawn((jake_bush, Transform::from_xyz(0.0, -20.0, 0.0)));
		});

	commands.insert_resource(UsingMonitor(monitor.clone()));

	next_app_state.set(AppState::Elsewhere);
	next_jake_state.set(JakeState::Intro);
}
