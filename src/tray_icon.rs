use bevy::prelude::*;
use tray_icon::{
	Icon, TrayIcon, TrayIconBuilder,
	menu::{Menu, MenuEvent, MenuId, MenuItem},
};

pub struct TrayState {
	_icon: TrayIcon,
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

	let exit_item = MenuItem::with_id("exit", "Exit jAkEtop", true, None);

	let menu = Menu::with_items(&[&exit_item]).expect("failed to create tray menu");

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
		exit_id: exit_item.id().to_owned(),
	});
}

pub fn handle_tray_events(tray: NonSend<TrayState>, mut app_exit: MessageWriter<AppExit>) {
	while let Ok(event) = MenuEvent::receiver().try_recv() {
		if event.id == tray.exit_id {
			app_exit.write(AppExit::Success);
		}
	}
}
