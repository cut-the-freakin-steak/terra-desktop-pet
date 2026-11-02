use crate::{Jake, JakeIntroComplete, JakeSquashNStretchCompleted};

use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryWindow};

pub fn jake_events(
	mut jake: Single<(&Jake, &mut Transform, &GlobalTransform)>,
	_on_monitor_windows: Query<(&Window, &bevy::window::OnMonitor), With<PrimaryWindow>>,
	_monitors: Query<&Monitor>,
	time: Res<Time>,
	jake_squash_complete: Option<Res<JakeSquashNStretchCompleted>>,
) {
	if jake_squash_complete.is_none() {
		return;
	}

	let (jake, transform, g_transform) = &mut *jake;
	transform.translation.x += 1.0
}
