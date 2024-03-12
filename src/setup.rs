// Setup module, used for initial game setup and initialising resources
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};

use crate::{helper::{cart_to_iso, despawn_entities_without, iso_to_cart, GameState}, menu::{get_button_shadow_text_style, get_button_text_style, BasicButton, GoatHead}, post_proc::PostProcessSettings};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_event::<BellEvent>()
			.insert_resource(SelectedButton(0.0))
			.insert_resource(ProgressTracker{
				mode:GameMode::Tutorial,
				max_campaign: 0,
				win_timer: Timer::from_seconds(2.0, TimerMode::Once),
				lose_timer: Timer::from_seconds(2.0, TimerMode::Once),
			})
			.insert_resource(Volume{
				track: 0,
				bgm: 0.4,
				sfx: 0.4,
			})
			.insert_resource(TargetGameState{
				state: GameState::Menu,
			})
			.insert_state(GameState::default())
			.add_systems(Startup, (
				setup,
			))
			.add_systems(OnExit(GameState::PreConfig), (
				start_music,
			))
			.add_systems(OnEnter(GameState::Config), (
				spawn_config,
			))
			.add_systems(Update, (
				adjust_volume.run_if(not(in_state(GameState::PreConfig))),
				adjust_volume_sliders.run_if(in_state(GameState::Config)),
			))
			.add_systems(OnExit(GameState::Transition), (
				despawn_entities_without::<GoatHead>,	
			))
		;
	}
}

#[derive(Event)]
pub struct BellEvent{
	pub location: Vec2,
	pub selected_bell: usize,
}

#[derive(PartialEq)]
pub enum GameMode {
	Tutorial,
	Campaign(usize),
	Endless(usize),
}

#[derive(Resource)]
pub struct ProgressTracker{
	pub mode: GameMode,
	pub max_campaign: usize,
	pub win_timer: Timer,
	pub lose_timer: Timer,
}

#[derive(Resource)]
pub struct SelectedButton(pub f32);

#[derive(Resource)]
pub struct TargetGameState{
	pub state: GameState,
}

#[derive(Resource)]
pub struct Volume{
	pub track: usize,
	pub bgm: f64,
	pub sfx: f64,
}

#[derive(Component)]
struct Persistent;

pub const ORTHO: Vec2 = Vec2::new(1920.0, 1080.0);
pub const REVERT: f32 = 1.2;

fn setup(
	mut commands: Commands,
) {
	// Spawn camera
	commands.spawn((Camera2dBundle{
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::Fixed{width: ORTHO.x, height: ORTHO.y},
			..default()
		},
		..default()
		},
		PostProcessSettings {
			time: 0.0,
			start_time: -100.0,
			end_time: 0.0,
			location: Vec2::ZERO,
		},
	));

	// let scale = 10.0;
	// for i in -79..80 {
	// 	for j in -44..45 {
	// 		let mut coords = Vec2::new(i as f32 * scale, j as f32 * scale);
	// 		// coords = cart_to_iso(coords);
	// 		// coords = iso_to_cart(coords);
	// 		commands.spawn((SpriteBundle {
	// 			transform: Transform::from_xyz(coords.x, coords.y, 250.0),
	// 			sprite: Sprite {
	// 				color: Color::rgb((i as f32 + 79.0)/159.0, 0.0, (j as f32 + 44.0)/89.0),
	// 				custom_size: Some(Vec2::new(2.0, 2.0)),
	// 				..default()
	// 			},
	// 			..default()
	// 			},
	// 		));
	// 	}
	// }

	// for i in -79..80 {
	// 	for j in -44..45 {
	// 		let mut coords = Vec2::new(i as f32 * scale, j as f32 * scale);
	// 		coords = iso_to_cart(coords);
	// 		commands.spawn((SpriteBundle {
	// 			transform: Transform::from_xyz(coords.x, coords.y, 250.0),
	// 			sprite: Sprite {
	// 				custom_size: Some(Vec2::new(2.0, 2.0)),
	// 				..default()
	// 			},
	// 			..default()
	// 			},
	// 		));
	// 	}
	// }

	// for i in -99..100 {
	// 	for j in -44..45 {
	// 		let mut coords = Vec2::new(i as f32 * 10.0, j as f32 * 30.0);
	// 		coords = cart_to_iso(coords);
	// 		commands.spawn((SpriteBundle {
	// 			transform: Transform::from_xyz(coords.x, coords.y, 250.0),
	// 			sprite: Sprite {
	// 				custom_size: Some(Vec2::new(2.0, 2.0)),
	// 				..default()
	// 			},
	// 			..default()
	// 			},
	// 		));
	// 	}
	// }
}

#[derive(Resource)]
pub struct BGMHandles{
	title: Handle<AudioInstance>,
	pure: Handle<AudioInstance>,
	corrupt: Handle<AudioInstance>,
}

fn start_music(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	volume: Res<Volume>,
) {
	let title = audio.play(asset_server.load("bgm/title.ogg")).looped().with_volume((volume.bgm*2.0).powf(2.0)).loop_from(4.0).handle();
	let pure = audio.play(asset_server.load("bgm/title_pure.ogg")).looped().with_volume((volume.bgm*2.0).powf(2.0)).loop_from(4.0).handle();
	let corrupt = audio.play(asset_server.load("bgm/title_corrupt.ogg")).looped().with_volume(0.0).loop_from(4.0).handle();
	commands.insert_resource(BGMHandles{title:title, pure:pure, corrupt:corrupt});
}

fn spawn_config(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut selected_button: ResMut<SelectedButton>,
) {
	selected_button.0 = 0.0;

	// Spawn splash screen
	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0, 200.0),
		texture: asset_server.load("config.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(ORTHO.x, ORTHO.y)),
			..default()
		},
		..default()
		},
	));



	let loc = [
		Vec2::new(0.0, 100.0*REVERT),
		Vec2::new(0.0, -50.0*REVERT),
		Vec2::new(0.0, -200.0*REVERT),
		Vec2::new(0.0, -350.0*REVERT),
	];
	let text = ["", "", "Toggle", "BEGIN"];
	for i in 0..loc.len() {
		let margin = 10.0;
		let size = Vec2::new(ORTHO.x*1.0/4.0, ORTHO.y/10.0);
		commands
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(loc[i].x, loc[i].y, 900.0),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, 0.4),
					custom_size: Some(size),
					..default()
				},
				..default()
			},
			BasicButton{
				screen: 0.0,
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

	let text = ["BGM Volume", "SFX Volume", "Fullscreen"];
	let loc = [
		Vec2::new(-ORTHO.x*1.0/8.0 - 30.0, 100.0*REVERT),
		Vec2::new(-ORTHO.x*1.0/8.0 - 30.0, -50.0*REVERT),
		Vec2::new(-ORTHO.x*1.0/8.0 - 30.0, -200.0*REVERT),
	];
	for i in 0..3 {
		let margin = 10.0;
		let size = Vec2::new(ORTHO.x*1.0/4.0, ORTHO.y/10.0);
		commands
			.spawn((Text2dBundle {
				text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
					size.x - margin * 2.0,
					size.y - margin * 2.0,
				)},
				transform: Transform::from_xyz(loc[i].x, loc[i].y,910.0),
				text_anchor: bevy::sprite::Anchor::CenterRight,
				text: Text::from_section(text[i], get_button_text_style(&asset_server))
				.with_justify(JustifyText::Right),
				..default()
			},
		));
	}

	let loc = [
		Vec2::new(0.0, 100.0*REVERT),
		Vec2::new(0.0, -50.0*REVERT),
	];
	for i in 0..2 {
		let size = Vec2::new(460.0, 90.0);
		commands
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(loc[i].x, loc[i].y, 930.0),
				sprite: Sprite {
					color: Color::CYAN,
					custom_size: Some(size),
					..default()
				},
				..default()
			},
			Slider(i),
		));
	}
}

#[derive(Component)]
pub struct Slider(usize);

fn adjust_volume_sliders(
	mut slider_query: Query<(&mut Transform, &Slider)>,
	volume: Res<Volume>,
) {
	for (mut transform, slider) in slider_query.iter_mut() {
		let vol = if slider.0 == 0 {volume.bgm} else {volume.sfx};
		transform.translation.x = (1.0 - vol as f32) * - 230.0;
		transform.scale.x = vol as f32;
	}
}

fn adjust_volume(
	volume: Res<Volume>,
	bgm_handles: Res<BGMHandles>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	if let Some(instance) = audio_instances.get_mut(&bgm_handles.title) {
		instance.set_volume(if volume.track == 0 {(volume.bgm*2.0).powf(2.0)}else{0.0}, AudioTween::default());
	}
	if let Some(instance) = audio_instances.get_mut(&bgm_handles.pure) {
		instance.set_volume(if volume.track == 1 {(volume.bgm*2.0).powf(2.0)}else{0.0}, AudioTween::default());
	}
	if let Some(instance) = audio_instances.get_mut(&bgm_handles.corrupt) {
		instance.set_volume(if !volume.track == 2{(volume.bgm*2.0).powf(2.0)}else{0.0}, AudioTween::default());
	}
}