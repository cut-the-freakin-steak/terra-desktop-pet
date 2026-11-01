use crate::Jake;

use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow};

pub fn jake_events(
	mut jake: Single<(&Jake, &mut Transform, &GlobalTransform)>,
	_on_monitor_windows: Query<(&Window, &bevy::window::OnMonitor), With<PrimaryWindow>>,
	_monitors: Query<&Monitor>,
	_time: Res<Time>,
) {
	let (_jake, _transform, _g_transform) = &mut *jake;
}
