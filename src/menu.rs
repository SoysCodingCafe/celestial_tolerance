// Menu module, for title screen
use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};

use crate::{cutscene::{CutsceneState, CutsceneTracker, SceneName}, helper::GameState, setup::{GameMode, ProgressTracker, SelectedButton, TargetGameState, Volume, ORTHO, REVERT}};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Menu), (
				spawn_menu,
			))
			.add_systems(Update, (
				highlight_selected_button,
				expand_goathead,
				fade_to_black,
			))
		;
	}
}

fn spawn_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut selected_button: ResMut<SelectedButton>,
) {
	selected_button.0 = 0.0;
	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0, 200.0),
		texture: asset_server.load("title.png"),
		sprite: Sprite {
			custom_size: Some(ORTHO),
			..default()
		},
		..default()
		},
	));

	let height = [80.0*REVERT, -30.0*REVERT, -140.0*REVERT, -250.0*REVERT, -360.0*REVERT];
	let text = ["TUTORIAL", "CAMPAIGN", "ENDLESS", "SETTINGS", "QUIT"];
	for i in 0..height.len() {
		let margin = 10.0;
		let size = Vec2::new(ORTHO.x*1.0/4.0, ORTHO.y/10.0);
		commands
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(0.0, height[i], 900.0),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, 0.4),
					custom_size: Some(size),
					..default()
				},
				..default()
			},
			BasicButton{
				screen: 1.0,
				index: i as f32,
			},
		)).with_children(|parent| {
			parent
				.spawn((Text2dBundle {
					text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
						size.x - margin * 2.0,
						size.y - margin * 2.0,
					)},
					transform: Transform::from_xyz(0.0, 0.0,10.0),
					text_anchor: bevy::sprite::Anchor::Center,
					text: Text::from_section(text[i], get_button_text_style(&asset_server))
					.with_justify(JustifyText::Center),
					..default()
				},
			));
			for j in 0..8 {
				let offset = match j {
					0 => (0.0, 1.0),
					1 => (1.0, 0.0),
					2 => (1.0, 1.0),
					3 => (0.0, -1.0),
					4 => (-1.0, 0.0),
					5 => (-1.0, -1.0),
					6 => (-1.0, 1.0),
					_ => (1.0, -1.0),
				};
				parent
					.spawn((Text2dBundle {
						text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
							size.x - margin * 2.0,
							size.y - margin * 2.0,
						)},
						transform: Transform::from_xyz(
							offset.0 * 3.0,
							offset.1 * 3.0,
							9.0,
						),
						text_anchor: bevy::sprite::Anchor::Center,
						text: Text::from_section(text[i], get_button_shadow_text_style(&asset_server))
						.with_justify(JustifyText::Center),
						..default()
					},
				));
			}
		});
	}
}

#[derive(Component)]
pub struct BasicButton{
	pub screen: f32,
	pub index: f32,
}

pub fn get_button_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/rony-siswadi-architect-1-font/smooth.ttf"),
		font_size: 64.0,
		color: Color::CYAN,
		..default()
	}
}

pub fn get_button_shadow_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/rony-siswadi-architect-1-font/smooth.ttf"),
		font_size: 64.0,
		color: Color::rgba(0.0, 0.2, 0.2, 1.0),
		..default()
	}
}

pub const SFX_SCALING: f64 = 1.0;

fn highlight_selected_button(
	mut commands: Commands,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut selected_button: ResMut<SelectedButton>,
	mut button_query: Query<(&mut Sprite, &BasicButton)>,
	current_state: Res<State<GameState>>,
	asset_server: Res<AssetServer>,
	mut volume: ResMut<Volume>,
	mut windows: Query<&mut Window, With<PrimaryWindow>>,
	mut next_state: ResMut<NextState<GameState>>,
	mut target_state: ResMut<TargetGameState>,
	mut ev_w_exit: EventWriter<AppExit>,
	mut progress_tracker: ResMut<ProgressTracker>,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	audio: Res<Audio>,
) {
	let screen = match *current_state.get() {
		GameState::Config => 0.0,
		GameState::Menu => 1.0,
		_ => -1.0,
	};
	let num_buttons = match screen as usize {
		0 => 4.0,
		1 => 5.0,
		_ => 0.0,
	};
	if keyboard.just_pressed(KeyCode::KeyW) {
		selected_button.0 = (selected_button.0 - 1.0) % num_buttons;
		if selected_button.0 == -1.0 {
			selected_button.0 = num_buttons - 1.0;
		}
	}
	if keyboard.just_pressed(KeyCode::KeyS) {
		selected_button.0 = (selected_button.0 + 1.0) % num_buttons;
	}

	if screen == 0.0 {
		let vol_down = keyboard.just_pressed(KeyCode::KeyA);
		let vol_up = keyboard.just_pressed(KeyCode::KeyD);
		if vol_up {
			match selected_button.0 as usize {
				0 => volume.bgm = (volume.bgm + 0.1).clamp(0.0, 1.0),
				1 => {
					volume.sfx = (volume.sfx + 0.1).clamp(0.0, 1.0);
					let rnd = rand::random::<f32>();
					let path = if rnd < 0.25 {
						"sfx/bell.ogg"
					} else if rnd < 0.5 {
						"sfx/bell_long.ogg"
					} else if rnd < 0.75 {
						"sfx/goat_short_far.ogg"
					} else {
						"sfx/goat_short.ogg"
					};
					audio.play(asset_server.load(path)).with_volume((volume.sfx*SFX_SCALING).powf(2.0));
				},
				_ => (),
			};
		}
		if vol_down {
			match selected_button.0 as usize {
				0 => volume.bgm = (volume.bgm - 0.1).clamp(0.0, 1.0),
				1 => {
					volume.sfx = (volume.sfx - 0.1).clamp(0.0, 1.0);
					let rnd = rand::random::<f32>();
					let path = if rnd < 0.25 {
						"sfx/bell.ogg"
					} else if rnd < 0.5 {
						"sfx/bell_long.ogg"
					} else if rnd < 0.75 {
						"sfx/goat_short_far.ogg"
					} else {
						"sfx/goat_short.ogg"
					};
					audio.play(asset_server.load(path)).with_volume((volume.sfx*SFX_SCALING).powf(2.0));
				},
				_ => (),
			};
		}

		if keyboard.just_pressed(KeyCode::Space) {
			match selected_button.0 as usize {
				2 => {
					for mut window in windows.iter_mut() {
						window.mode = match window.mode {
							bevy::window::WindowMode::Windowed => bevy::window::WindowMode::BorderlessFullscreen,
							_ => bevy::window::WindowMode::Windowed,
						}
					}
				},
				3 => {
					next_state.set(GameState::Transition);
					target_state.state = GameState::Menu;
					commands.spawn((SpriteBundle {
						transform: Transform::from_xyz(0.0, 0.0, 950.0),
						sprite: Sprite {
							color: Color::rgba(0.0, 0.0, 0.0, 0.0),
							custom_size: Some(ORTHO),
							..default()
						},
						..default()
						},
						ScreenFade{
							up: true,
							timer: Timer::from_seconds(0.5, TimerMode::Once)
						},
					));
				},
				_ => (),
			}
		}
	} else if screen == 1.0 {
		if keyboard.just_pressed(KeyCode::Space) {
			match selected_button.0 as usize {
				0 => {
					progress_tracker.mode = GameMode::Tutorial;
					target_state.state = GameState::Cutscene;
					cutscene_tracker.current_scene = SceneName::TutorialIntro;
					cutscene_tracker.cutscene_state = CutsceneState::Initialize;
					next_state.set(GameState::Transition);
					commands.spawn((SpriteBundle {
						transform: Transform::from_xyz(0.0, 0.0, 950.0).with_scale(Vec3::new(0.0, 0.0, 1.0)),
						texture: asset_server.load("goathead.png"),
						sprite: Sprite {
							custom_size: Some(Vec2::new(64.0, 64.0)),
							..default()
						},
						..default()
						},
						GoatHead{
							up: true,
							timer: Timer::from_seconds(1.5, TimerMode::Once)
						},
					));
				},
				1 => {
					match progress_tracker.max_campaign {
						0 => {
							progress_tracker.mode = GameMode::Campaign(0);
							cutscene_tracker.current_scene = SceneName::CampaignIntro;
						},
						1 => {
							progress_tracker.mode = GameMode::Campaign(1);
							cutscene_tracker.current_scene = SceneName::CampaignDay(0)
						},
						2 => {
							progress_tracker.mode = GameMode::Campaign(2);
							cutscene_tracker.current_scene = SceneName::CampaignDay(1)
						},
						_ => {
							progress_tracker.mode = GameMode::Campaign(3);
							cutscene_tracker.current_scene = SceneName::CampaignDay(2)
						},
					};
					target_state.state = GameState::Cutscene;
					cutscene_tracker.cutscene_state = CutsceneState::Initialize;
					next_state.set(GameState::Transition);
					commands.spawn((SpriteBundle {
						transform: Transform::from_xyz(0.0, 0.0, 950.0).with_scale(Vec3::new(0.0, 0.0, 1.0)),
						texture: asset_server.load("goathead.png"),
						sprite: Sprite {
							custom_size: Some(Vec2::new(64.0, 64.0)),
							..default()
						},
						..default()
						},
						GoatHead{
							up: true,
							timer: Timer::from_seconds(1.5, TimerMode::Once)
						},
					));
				},
				2 => {
					progress_tracker.mode = GameMode::Endless(0);
					target_state.state = GameState::Game;
					next_state.set(GameState::Transition);
					commands.spawn((SpriteBundle {
						transform: Transform::from_xyz(0.0, 0.0, 950.0).with_scale(Vec3::new(0.0, 0.0, 1.0)),
						texture: asset_server.load("goathead.png"),
						sprite: Sprite {
							custom_size: Some(Vec2::new(64.0, 64.0)),
							..default()
						},
						..default()
						},
						GoatHead{
							up: true,
							timer: Timer::from_seconds(1.5, TimerMode::Once)
						},
					));
				}
				3 => {
					target_state.state = GameState::Config;
					next_state.set(GameState::Transition);
					commands.spawn((SpriteBundle {
						transform: Transform::from_xyz(0.0, 0.0, 950.0),
						sprite: Sprite {
							color: Color::rgba(0.0, 0.0, 0.0, 0.0),
							custom_size: Some(ORTHO),
							..default()
						},
						..default()
						},
						ScreenFade{
							up: true,
							timer: Timer::from_seconds(0.5, TimerMode::Once)
						},
					));
				}
				4 => {ev_w_exit.send(AppExit);}, // Quit
				_ => (),
			}
		}
	}

	for (mut sprite, button) in button_query.iter_mut() {
		if button.screen == screen {
			if button.index == selected_button.0 {
				sprite.color = Color::WHITE;
			} else {
				sprite.color = Color::GRAY;
			}
		}
	}
}

#[derive(Component)]
pub struct GoatHead{
	pub up: bool,
	pub timer: Timer,
}

#[derive(Component)]
pub struct ScreenFade{
	pub up: bool,
	pub timer: Timer,
}

fn expand_goathead(
	time: Res<Time>,
	mut commands: Commands,
	mut goathead_query: Query<(Entity, &mut Transform, &mut GoatHead)>,
	mut next_state: ResMut<NextState<GameState>>,
	target_state: Res<TargetGameState>,
) {
	let max_size = ORTHO.x/4.0;
	let curve = 3.0;
	for (entity, mut transform, mut goathead) in goathead_query.iter_mut() {
		goathead.timer.tick(time.delta());
		if goathead.up {
			transform.scale.x = goathead.timer.fraction().powf(curve) * max_size;
			transform.scale.y = goathead.timer.fraction().powf(curve) * max_size;
			if goathead.timer.finished() {
				next_state.set(target_state.state);
				goathead.up = false;
				goathead.timer.reset();
			}
		} else {
			transform.scale.x = (1.0 - goathead.timer.fraction()).powf(curve) * max_size;
			transform.scale.y = (1.0 - goathead.timer.fraction()).powf(curve) * max_size;
			if goathead.timer.finished() {
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}

fn fade_to_black(
	time: Res<Time>,
	mut commands: Commands,
	mut screen_fade_query: Query<(Entity, &mut Sprite, &mut ScreenFade)>,
	mut next_state: ResMut<NextState<GameState>>,
	target_state: Res<TargetGameState>,
) {
	let curve = 0.5;
	for (entity, mut sprite, mut screen_fade) in screen_fade_query.iter_mut() {
		screen_fade.timer.tick(time.delta());
		if screen_fade.up {
			sprite.color.set_a(screen_fade.timer.fraction().powf(curve));
			if screen_fade.timer.finished() {
				next_state.set(target_state.state);
				screen_fade.up = false;
				screen_fade.timer.reset();
			}
		} else {
			sprite.color.set_a((1.0 - screen_fade.timer.fraction()).powf(curve));
			if screen_fade.timer.finished() {
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}