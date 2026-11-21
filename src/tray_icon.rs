use bevy::prelude::*;
use tray_icon::{
	Icon, TrayIcon, TrayIconBuilder,
	menu::{Menu, MenuEvent, MenuId, MenuItem, Submenu},
};

use crate::{BASE_JAKE_WIDTH_ISH, Jake, JakeWidth, movable_helpers::FixedTransform};

pub struct TrayState {
	_icon: TrayIcon,
	scale_025_id: MenuId,
	scale_050_id: MenuId,
	scale_075_id: MenuId,
	scale_100_id: MenuId,
	scale_125_id: MenuId,
	scale_150_id: MenuId,
	scale_175_id: MenuId,
	scale_200_id: MenuId,
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

	let submenu = Submenu::with_id("scales", "scale jAkE", true);
	let scale_025_item = MenuItem::with_id("0.25x", "scale jAkE 0.25x", true, None);
	let scale_050_item = MenuItem::with_id("0.50x", "scale jAkE 0.50x", true, None);
	let scale_075_item = MenuItem::with_id("0.75x", "scale jAkE 0.75x", true, None);
	let scale_100_item = MenuItem::with_id("1.00x", "scale jAkE 1.00x", true, None);
	let scale_125_item = MenuItem::with_id("1.25x", "scale jAkE 1.25x", true, None);
	let scale_150_item = MenuItem::with_id("1.50x", "scale jAkE 1.50x", true, None);
	let scale_175_item = MenuItem::with_id("1.75x", "scale jAkE 1.75x", true, None);
	let scale_200_item = MenuItem::with_id("2.00x", "scale jAkE 2.00x", true, None);
	let exit_item = MenuItem::with_id("exit", "exit jAkEtop", true, None);

	let _ = submenu.append_items(&[
		&scale_025_item,
		&scale_050_item,
		&scale_075_item,
		&scale_100_item,
		&scale_125_item,
		&scale_150_item,
		&scale_175_item,
		&scale_200_item,
	]);

	// maybe later

	let menu = Menu::with_items(&[&submenu, &exit_item]).expect("failed to create tray menu");

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
		scale_025_id: scale_025_item.id().to_owned(),
		scale_050_id: scale_050_item.id().to_owned(),
		scale_075_id: scale_075_item.id().to_owned(),
		scale_100_id: scale_100_item.id().to_owned(),
		scale_125_id: scale_125_item.id().to_owned(),
		scale_150_id: scale_150_item.id().to_owned(),
		scale_175_id: scale_175_item.id().to_owned(),
		scale_200_id: scale_200_item.id().to_owned(),
		exit_id: exit_item.id().to_owned(),
	});
}

pub fn handle_tray_events(
	tray: NonSend<TrayState>,
	mut app_exit: MessageWriter<AppExit>,
	mut jake: Single<&mut FixedTransform, With<Jake>>,
	mut jake_width_ish: ResMut<JakeWidth>,
) {
	let jake_f_transform = &mut *jake;
	let jake_width_ish = &mut jake_width_ish.0;

	while let Ok(event) = MenuEvent::receiver().try_recv() {
		if event.id == tray.exit_id {
			app_exit.write(AppExit::Success);
		}
		else if event.id == tray.scale_025_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 0.5,
				y: 0.5,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 0.25;
		}
		else if event.id == tray.scale_050_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 1.0,
				y: 1.0,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 0.5;
		}
		else if event.id == tray.scale_075_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 1.5,
				y: 1.5,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 0.75;
		}
		else if event.id == tray.scale_100_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 2.0,
				y: 2.0,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 1.0;
		}
		else if event.id == tray.scale_125_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 2.5,
				y: 2.5,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 1.25;
		}
		else if event.id == tray.scale_150_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 3.0,
				y: 3.0,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 1.5;
		}
		else if event.id == tray.scale_175_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 3.5,
				y: 3.5,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 1.75;
		}
		else if event.id == tray.scale_200_id {
			let prev_z = jake_f_transform.scale().z;
			*jake_f_transform.scale_mut() = Vec3 {
				x: 4.0,
				y: 4.0,
				z: prev_z,
			};
			*jake_width_ish = BASE_JAKE_WIDTH_ISH * 2.0;
		}
	}
}
