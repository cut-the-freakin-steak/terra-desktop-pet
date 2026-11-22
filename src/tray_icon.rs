use std::io::Write;

use bevy::{audio::Volume, prelude::*};
use tray_icon::{
	Icon, TrayIcon, TrayIconBuilder,
	menu::{Menu, MenuEvent, MenuId, MenuItem, Submenu},
};

use crate::{
	APP_DIR, JAKE_BASE_WIDTH_ISH, Jake, JakeState, JakeWidth, movable_helpers::FixedTransform,
};

pub struct TrayState {
	_icon: TrayIcon,
	scale_ids: [MenuId; 8],
	volume_ids: [MenuId; 11],
	jake_mode_ids: [MenuId; 2],
	exit_id: MenuId,
}

pub fn setup_tray(world: &mut World) {
	let png = include_bytes!("../assets/jAkE_app_icon.png");
	let image = image::load_from_memory(png)
		.expect("failed to decode tray icon")
		.into_rgba8();

	let (width, height) = image.dimensions();

	let icon =
		Icon::from_rgba(image.into_raw(), width, height).expect("failed to create tray icon");

	let jake_modes_submenu = Submenu::with_id("jAkE_modes", "jAkE modes", true);
	let roaming_item = MenuItem::with_id("roaming", "roaming", true, None);
	let chasing_item = MenuItem::with_id("chasing", "chasing", true, None);

	let _ = jake_modes_submenu.append_items(&[&roaming_item, &chasing_item]);

	let scaling_submenu = Submenu::with_id("scales", "scale jAkE", true);
	let scale_025_item = MenuItem::with_id("0.25x", "scale jAkE 0.25x", true, None);
	let scale_050_item = MenuItem::with_id("0.50x", "scale jAkE 0.50x", true, None);
	let scale_075_item = MenuItem::with_id("0.75x", "scale jAkE 0.75x", true, None);
	let scale_100_item = MenuItem::with_id("1.00x", "scale jAkE 1.00x", true, None);
	let scale_125_item = MenuItem::with_id("1.25x", "scale jAkE 1.25x", true, None);
	let scale_150_item = MenuItem::with_id("1.50x", "scale jAkE 1.50x", true, None);
	let scale_175_item = MenuItem::with_id("1.75x", "scale jAkE 1.75x", true, None);
	let scale_200_item = MenuItem::with_id("2.00x", "scale jAkE 2.00x", true, None);

	let _ = scaling_submenu.append_items(&[
		&scale_025_item,
		&scale_050_item,
		&scale_075_item,
		&scale_100_item,
		&scale_125_item,
		&scale_150_item,
		&scale_175_item,
		&scale_200_item,
	]);

	let volume_submenu = Submenu::with_id("volumes", "change volume", true);
	let volume_mute_item = MenuItem::with_id("mute", "mute", true, None);
	let volume_001_item = MenuItem::with_id("1%", "1% volume lmao", true, None);
	let volume_025_item = MenuItem::with_id("25%", "25% volume", true, None);
	let volume_050_item = MenuItem::with_id("50%", "50% volume", true, None);
	let volume_075_item = MenuItem::with_id("75%", "75% volume", true, None);
	let volume_100_item = MenuItem::with_id("100%", "100% volume", true, None);
	let volume_125_item = MenuItem::with_id("125%", "125% volume", true, None);
	let volume_150_item = MenuItem::with_id("150%", "150% volume", true, None);
	let volume_175_item = MenuItem::with_id("175%", "175% volume", true, None);
	let volume_200_item = MenuItem::with_id("200%", "200% volume", true, None);
	let volume_300_item = MenuItem::with_id("300%", "300% volume :p", true, None);

	let _ = volume_submenu.append_items(&[
		&volume_mute_item,
		&volume_001_item,
		&volume_025_item,
		&volume_050_item,
		&volume_075_item,
		&volume_100_item,
		&volume_125_item,
		&volume_150_item,
		&volume_175_item,
		&volume_200_item,
		&volume_300_item,
	]);

	let exit_item = MenuItem::with_id("exit", "exit jAkEtop", true, None);

	let menu = Menu::with_items(&[
		&scaling_submenu,
		&volume_submenu,
		&jake_modes_submenu,
		&exit_item,
	])
	.expect("failed to create tray menu");

	let tray = TrayIconBuilder::new()
		.with_icon(icon)
		.with_tooltip("jAkEtop")
		.with_menu(Box::new(menu))
		.with_menu_on_left_click(true)
		.with_menu_on_right_click(true)
		.build()
		.expect("failed to create tray icon");

	world.insert_non_send(TrayState {
		_icon: tray,
		scale_ids: [
			scale_025_item.id().to_owned(),
			scale_050_item.id().to_owned(),
			scale_075_item.id().to_owned(),
			scale_100_item.id().to_owned(),
			scale_125_item.id().to_owned(),
			scale_150_item.id().to_owned(),
			scale_175_item.id().to_owned(),
			scale_200_item.id().to_owned(),
		],
		volume_ids: [
			volume_mute_item.id().to_owned(),
			volume_001_item.id().to_owned(),
			volume_025_item.id().to_owned(),
			volume_050_item.id().to_owned(),
			volume_075_item.id().to_owned(),
			volume_100_item.id().to_owned(),
			volume_125_item.id().to_owned(),
			volume_150_item.id().to_owned(),
			volume_175_item.id().to_owned(),
			volume_200_item.id().to_owned(),
			volume_300_item.id().to_owned(),
		],
		jake_mode_ids: [roaming_item.id().to_owned(), chasing_item.id().to_owned()],
		exit_id: exit_item.id().to_owned(),
	});
}

pub fn handle_tray_events(
	tray: NonSend<TrayState>,
	mut app_exit: MessageWriter<AppExit>,
	mut global_volume: ResMut<GlobalVolume>,
	mut jake: Single<&mut FixedTransform, With<Jake>>,
	mut jake_width_ish: ResMut<JakeWidth>,
	mut next_jake_state: ResMut<NextState<JakeState>>,
) {
	let jake_f_transform = &mut *jake;
	let jake_width_ish = &mut jake_width_ish.0;

	while let Ok(event) = MenuEvent::receiver().try_recv() {
		// APP EXIT
		if event.id == tray.exit_id {
			let mut data_file = std::fs::OpenOptions::new()
				.read(false)
				.write(true)
				.append(false)
				.create(true)
				.open(APP_DIR.join("jAkE_data_stuff.jake"))
				.expect(&format!(
					"could not open file: {:?}",
					APP_DIR.join("jAkE_data_stuff.jake")
				));

			let jake_scale = jake_f_transform.scale().x;
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

			app_exit.write(AppExit::Success);
		}
		// SCALING
		// scale 0.25x
		else if event.id == tray.scale_ids.get(0).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 0.5,
				y: 0.5,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 0.25;
		}
		// scale 0.50x
		else if event.id == tray.scale_ids.get(1).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 1.0,
				y: 1.0,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 0.5;
		}
		// scale 0.75x
		else if event.id == tray.scale_ids.get(2).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 1.5,
				y: 1.5,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 0.75;
		}
		// scale 1.00x
		else if event.id == tray.scale_ids.get(3).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 2.0,
				y: 2.0,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 1.0;
		}
		// scale 1.25x
		else if event.id == tray.scale_ids.get(4).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 2.5,
				y: 2.5,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 1.25;
		}
		// scale 1.50x
		else if event.id == tray.scale_ids.get(5).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 3.0,
				y: 3.0,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 1.5;
		}
		// scale 1.75x
		else if event.id == tray.scale_ids.get(6).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 3.5,
				y: 3.5,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 1.75;
		}
		// scale 2.00x
		else if event.id == tray.scale_ids.get(7).unwrap() {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 4.0,
				y: 4.0,
				z: prev_z,
			};
			*jake_width_ish = JAKE_BASE_WIDTH_ISH * 2.0;
		}
		// VOLUME CONTROL
		// mute
		else if event.id == tray.volume_ids.get(0).unwrap() {
			global_volume.volume = Volume::Linear(0.00);
		}
		// 1% volume lol
		else if event.id == tray.volume_ids.get(1).unwrap() {
			global_volume.volume = Volume::Linear(0.01);
		}
		// 25% volume
		else if event.id == tray.volume_ids.get(2).unwrap() {
			global_volume.volume = Volume::Linear(0.25);
		}
		// 50% volume
		else if event.id == tray.volume_ids.get(3).unwrap() {
			global_volume.volume = Volume::Linear(0.50);
		}
		// 75% volume
		else if event.id == tray.volume_ids.get(4).unwrap() {
			global_volume.volume = Volume::Linear(0.75);
		}
		// 100% volume
		else if event.id == tray.volume_ids.get(5).unwrap() {
			global_volume.volume = Volume::Linear(1.00);
		}
		// 125% volume
		else if event.id == tray.volume_ids.get(6).unwrap() {
			global_volume.volume = Volume::Linear(1.25);
		}
		// 150% volume
		else if event.id == tray.volume_ids.get(7).unwrap() {
			global_volume.volume = Volume::Linear(1.50);
		}
		// 175% volume
		else if event.id == tray.volume_ids.get(8).unwrap() {
			global_volume.volume = Volume::Linear(1.75);
		}
		// 200% volume
		else if event.id == tray.volume_ids.get(9).unwrap() {
			global_volume.volume = Volume::Linear(2.00);
		}
		// 300% volume :p
		else if event.id == tray.volume_ids.get(10).unwrap() {
			global_volume.volume = Volume::Linear(3.00);
		}
		// JAKE MODES
		// jake roaming mode
		else if event.id == tray.jake_mode_ids.get(0).unwrap() {
			next_jake_state.set(JakeState::Roaming);
		}
		// jake chasing mode
		else if event.id == tray.jake_mode_ids.get(1).unwrap() {
			next_jake_state.set(JakeState::ChasingMouse { toggled: true });
		}
	}
}
