// Debugging module, only used for features that should not get compiled into the final game
use bevy::{prelude::*, app::AppExit};

use crate::{goat::GoatMovement, helper::{cart_to_iso, GameState}, setup::REVERT};

// use bevy_editor_pls::EditorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
		app
			// .add_plugins((
			// 	EditorPlugin::default(),
			// ))
			.add_systems(Update, (
				toggle_resolution,
				switch_states,
				win_level,
				quit_game,
			))
		;
	}
}

fn toggle_resolution(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
	let resolutions = [
		Vec2::new(640.0, 360.0),
		Vec2::new(1280.0, 720.0),
		Vec2::new(1600.0, 900.0),
		Vec2::new(1920.0, 1080.0),
	];
	if keyboard.pressed(KeyCode::ControlRight) {
		if keyboard.just_pressed(KeyCode::KeyF) {
			window.mode = match window.mode {
				bevy::window::WindowMode::Windowed => bevy::window::WindowMode::BorderlessFullscreen,
				_ => bevy::window::WindowMode::Windowed,
			}
		}
		if keyboard.just_pressed(KeyCode::Digit1) {
			let res = resolutions[0];
			window.resolution.set(res.x, res.y);
		}
		if keyboard.just_pressed(KeyCode::Digit2) {
			let res = resolutions[1];
			window.resolution.set(res.x, res.y);
		}
		if keyboard.just_pressed(KeyCode::Digit3) {
			let res = resolutions[2];
			window.resolution.set(res.x, res.y);
		}
		if keyboard.just_pressed(KeyCode::Digit4) {
			let res = resolutions[3];
			window.resolution.set(res.x, res.y);
		}
	}
}

fn switch_states(
	mut next_state: ResMut<NextState<GameState>>,
	keyboard: Res<ButtonInput<KeyCode>>,
) {
	if keyboard.pressed(KeyCode::ControlLeft) {
		let key_states =[
			(KeyCode::Digit1, GameState::Config),
			(KeyCode::Digit2, GameState::Menu),
			(KeyCode::Digit3, GameState::Cutscene),
			(KeyCode::Digit4, GameState::Game),
		];
		for (key, state) in key_states.iter() {
			if keyboard.just_pressed(*key) {
				next_state.set(*state);
			}
		}
	}
}

fn win_level(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	if keyboard.pressed(KeyCode::KeyO) {
		let iso_loc = cart_to_iso(Vec2::new(750.0, -50.0)*REVERT);
		commands.spawn((SpriteBundle {
			transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 600.0),
			texture: asset_server.load("goat.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(64.0, 64.0)),
				..default()
			},
			..default()
			},
			GoatMovement {
				cart_transform: Vec2::new(750.0, -50.0)*REVERT,
				velocity: Vec2::new(1.0, 0.0),
				speed: 90.0,
				boost_timer: Timer::from_seconds(3.0, TimerMode::Once),
				move_timer: Timer::from_seconds(3.0, TimerMode::Once),
				idle_timer: Timer::from_seconds(1.0, TimerMode::Once),
				iso_mode: true,
				grabbed: false,
				safe: false,
				furthest_tile: 0,
			}
		));
	}
}

fn quit_game(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut ev_w_exit: EventWriter<AppExit>,
) {
	if keyboard.just_pressed(KeyCode::Escape) {
		ev_w_exit.send(AppExit);
	}
}