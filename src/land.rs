// Land module, for handling land placement and collisions
use bevy::prelude::*;

use crate::{cutscene::{CutsceneState, CutsceneTracker, SceneName}, enemy::Arm, goat::{GoatMovement, GoatNumbers}, helper::GameState, menu::ScreenFade, player::PlayerMovement, setup::{GameMode, ProgressTracker, TargetGameState, ORTHO, REVERT}};

pub struct LandPlugin;

impl Plugin for LandPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(GoatsHerded(0))
			.insert_resource(TitleTimer(Timer::from_seconds(1.2, TimerMode::Once)))
			.add_systems(OnEnter(GameState::Game), (
				land_setup,
			))
			.add_systems(Update, (
				update_ui_text,
				return_to_title,
			).run_if(in_state(GameState::Game)))
		;
	}
}

#[derive(Resource)]
pub struct Anchors(pub Vec<Vec2>);

#[derive(Component)]
pub struct Land{
	pub half_size: Vec2,
	pub tile_index: usize,
}

#[derive(Component)]
pub struct Goal{
	pub half_size: Vec2,
}

fn land_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut progress_tracker: ResMut<ProgressTracker>,
	mut goat_numbers: ResMut<GoatNumbers>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	progress_tracker.win_timer.reset();
	progress_tracker.lose_timer.reset();

	goat_numbers.spawned = 0;
	goat_numbers.killed = 0;

	match progress_tracker.mode {
		GameMode::Tutorial => (),
		GameMode::Campaign(i) | GameMode::Endless(i) => {
			if i > 1 {
				let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(1920.0, 1080.0), 2, 1, Some(Vec2::ZERO), Some(Vec2::ZERO)));
				commands.spawn((SpriteSheetBundle {
					transform: Transform::from_xyz(0.0, 0.0,  553.0),
					texture: asset_server.load("arm.png"),
					atlas: TextureAtlas{
						layout: texture_atlas_layout,
						index: 1,
					},
					sprite: Sprite {
						custom_size: Some(ORTHO),
						..default()
					},
					..default()
					},
					Arm{
						wait_timer: Timer::from_seconds(6.1, TimerMode::Once),
						slam_timer: Timer::from_seconds(1.3, TimerMode::Once),
					}
				));
			}
		},
	}

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0,  870.0),
		texture: asset_server.load("fence.png"),
		sprite: Sprite {
			custom_size: Some(ORTHO),
			..default()
		},
		..default()
		},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0,  450.0),
		texture: asset_server.load("iso_map.png"),
		sprite: Sprite {
			custom_size: Some(ORTHO),
			..default()
		},
		..default()
		},
	));
	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0,  550.0),
		texture: asset_server.load("iso_map_top.png"),
		sprite: Sprite {
			custom_size: Some(ORTHO),
			..default()
		},
		..default()
		},
	));

	if progress_tracker.mode == GameMode::Tutorial {
		commands.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 0.0,  551.0),
			texture: asset_server.load("iso_map_tutorial.png"),
			sprite: Sprite {
				custom_size: Some(ORTHO),
				..default()
			},
			..default()
			},
		));
	}

	let margin = 10.0;
	let size = Vec2::new(500.0, 300.0)*REVERT;
	commands
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(-540.0*REVERT, -290.0*REVERT, 900.0),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, 0.0),
					custom_size: Some(size),
					..default()
				},
				..default()
			},
		)).with_children(|parent| {
			parent
				.spawn((Text2dBundle {
					text_2d_bounds: bevy::text::Text2dBounds{ size: Vec2::new(
						size.x - margin * 2.0,
						size.y - margin * 2.0,
					)},
					transform: Transform::from_xyz(
						-size.x / 2.0 + margin,
						-size.y / 2.0 + margin,
						10.0,
					),
					text_anchor: bevy::sprite::Anchor::BottomLeft,
					text: Text::from_section("Goats Herded: 0\nRequired Goats: 10\nEquipped Bell: Death", get_ui_text_style(&asset_server))
					.with_justify(JustifyText::Left),
					..default()
				},
				UIText,
			));
		});

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(750.0*REVERT, -50.0*REVERT,  210.0),
		sprite: Sprite {
			color: Color::rgba(1.0, 0.1, 0.1, 0.0),
			custom_size: Some(Vec2::new(200.0, 300.0)*REVERT),
			..default()
		},
		..default()
		},
		Goal{
			half_size: Vec2::new(200.0, 300.0)*REVERT/2.0,
		},
	));

	let locs = [
		Vec2::new(-650.0, 0.0)*REVERT,
		Vec2::new(-350.0, -150.0)*REVERT,
		Vec2::new(-100.0, 50.0)*REVERT,
		Vec2::new(150.0, 300.0)*REVERT,
		Vec2::new(387.5, 100.0)*REVERT,
		Vec2::new(650.0, -50.0)*REVERT,

		Vec2::new(750.0, -50.0)*REVERT, // Goal
		Vec2::new(-400.0, 225.0)*REVERT, // Island
		Vec2::new(175.0, 50.0)*REVERT, // Island
		Vec2::new(175.0, -175.0)*REVERT, // Island
	];

	let sizes = [
		Vec2::new(400.0, 200.0)*REVERT,
		Vec2::new(400.0, 200.0)*REVERT,
		Vec2::new(300.0, 550.0)*REVERT,
		Vec2::new(400.0, 150.0)*REVERT,
		Vec2::new(175.0, 500.0)*REVERT,
		Vec2::new(400.0, 300.0)*REVERT,

		Vec2::new(200.0, 300.0)*REVERT, // Goal
		Vec2::new(175.0, 150.0)*REVERT, // Island
		Vec2::new(125.0, 125.0)*REVERT, // Island
		Vec2::new(150.0, 150.0)*REVERT, // Island
	];

	let mut anchors = Vec::new();
	for i in 0..6 {
		let mut sign = (1.0, 1.0, 1.0, 1.0);
		if locs[i].x < locs[i+1].x {sign.1 = -1.0} else {sign.0 = -1.0};
		if locs[i].y < locs[i+1].y {sign.3 = -1.0} else {sign.2 = -1.0};
		anchors.push(
			Vec2::new(
				((locs[i].x + sign.0*sizes[i].x / 2.0) + (locs[i+1].x + sign.1*sizes[i+1].x / 2.0)) / 2.0,
				((locs[i].y + sign.2*sizes[i].y / 2.0) + (locs[i+1].y + sign.3*sizes[i+1].y / 2.0)) / 2.0,
			),
		);
	}

	// for i in 0..anchors.len() {
	// 	let anchor = cart_to_iso(anchors[i]);
	// 	commands.spawn((SpriteBundle {
	// 		transform: Transform::from_xyz(anchor.x, anchor.y,  999.0),
	// 		sprite: Sprite {
	// 			custom_size: Some(Vec2::new(20.0, 20.0)),
	// 			..default()
	// 		},
	// 		..default()
	// 		},
	// 	));
	// }

	commands.insert_resource(Anchors(anchors));

	// let locs = [
	// 	Vec2::new(0.0, 0.0)*REVERT,
	// 	Vec2::new(-450.0, 450.0)*REVERT,
	// 	Vec2::new(450.0, -450.0)*REVERT,
	// 	Vec2::new(-30.0, -500.0)*REVERT,
	// ];

	// let sizes = [
	// 	Vec2::new(800.0, 800.0)*REVERT,
	// 	Vec2::new(200.0, 600.0)*REVERT,
	// 	Vec2::new(300.0, 200.0)*REVERT,
	// 	Vec2::new(100.0, 100.0)*REVERT,
	// ];

	for i in 0..locs.len() {
		commands.spawn((SpriteBundle {
			transform: Transform::from_xyz(locs[i].x, locs[i].y,  210.0),
			sprite: Sprite {
				color: Color::rgba(1.0, 0.1, 0.1, 0.0),
				custom_size: Some(sizes[i]),
				..default()
			},
			..default()
			},
			Land{
				half_size: sizes[i]/2.0,
				tile_index: i,
			},
		));
	}

	// let corners = [
	// 	Vec2::new(-1.0, 1.0),
	// 	Vec2::new(1.0, 1.0),
	// 	Vec2::new(1.0, -1.0),
	// 	Vec2::new(-1.0, -1.0),
	// ];
	// let lerp = 100;
	// for i in 0..locs.len() {
	// 	for c in 0..corners.len() {
	// 		let coord_1 = cart_to_iso(Vec2::new(
	// 			locs[i].x + corners[c].x * sizes[i].x/2.0,
	// 			locs[i].y + corners[c].y * sizes[i].y/2.0,  
	// 		));
	// 		let coord_2 = cart_to_iso(Vec2::new(
	// 			locs[i].x + corners[(c+1)%corners.len()].x * sizes[i].x/2.0,
	// 			locs[i].y + corners[(c+1)%corners.len()].y * sizes[i].y/2.0,  
	// 		));
	// 		for l in 0..lerp {
	// 			let frac = l as f32/lerp as f32;
	// 			commands.spawn((SpriteBundle {
	// 				transform: Transform::from_xyz(
	// 					coord_1.x*frac + coord_2.x*(1.0-frac),
	// 					coord_1.y*frac + coord_2.y*(1.0-frac),
	// 				210.0),
	// 				sprite: Sprite {
	// 					color: Color::CYAN,
	// 					custom_size: Some(Vec2::new(5.0, 5.0)),
	// 					..default()
	// 				},
	// 				..default()
	// 				},
	// 			));
	// 		}
	// 	}
	// }

	// for i in 0..locs.len() {
	// 	for c in 0..corners.len() {
	// 		let coord_1 = Vec2::new(
	// 			locs[i].x + corners[c].x * sizes[i].x/2.0,
	// 			locs[i].y + corners[c].y * sizes[i].y/2.0,  
	// 		);
	// 		let coord_2 = Vec2::new(
	// 			locs[i].x + corners[(c+1)%corners.len()].x * sizes[i].x/2.0,
	// 			locs[i].y + corners[(c+1)%corners.len()].y * sizes[i].y/2.0,  
	// 		);
	// 		for l in 0..lerp {
	// 			let frac = l as f32/lerp as f32;
	// 			commands.spawn((SpriteBundle {
	// 				transform: Transform::from_xyz(
	// 					coord_1.x*frac + coord_2.x*(1.0-frac),
	// 					coord_1.y*frac + coord_2.y*(1.0-frac),
	// 				210.0),
	// 				sprite: Sprite {
	// 					color: Color::RED,
	// 					custom_size: Some(Vec2::new(5.0, 5.0)),
	// 					..default()
	// 				},
	// 				..default()
	// 				},
	// 			));
	// 		}
	// 	}
	// }

	
}

#[derive(Resource)]
pub struct GoatsHerded(pub usize);

#[derive(Component)]
pub struct UIText;

pub fn get_ui_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/rony-siswadi-architect-1-font/smooth.ttf"),
		font_size: 54.0,
		color: Color::rgba(0.08, 0.12, 0.12, 1.0),
		..default()
	}
}

fn update_ui_text(
	mut text_query: Query<(&mut Text, &UIText)>,
	mut goats_herded: ResMut<GoatsHerded>,
	time: Res<Time>,
	player_query: Query<&PlayerMovement>,
	goat_query: Query<&GoatMovement>,
	asset_server: Res<AssetServer>,
	mut progress_tracker: ResMut<ProgressTracker>,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	mut target_state: ResMut<TargetGameState>,
	mut next_state: ResMut<NextState<GameState>>,
	mut commands: Commands,
	mut goat_numbers: ResMut<GoatNumbers>,
) {
	let mut safe_goats = 0;
	for goat in goat_query.iter() {
		if goat.safe {safe_goats += 1;};
	}
	goats_herded.0 = safe_goats;
	let mut selected_bell = "";
	for player in player_query.iter() {
		selected_bell = match player.selected_bell as usize {
			0 => "Beckon",
			1 => "Warn",
			2 => "Gather",
			3 => "Energize",
			_ => "Dance",
		};
	}
	let goats_required = match progress_tracker.mode {
		GameMode::Tutorial => 10,
		GameMode::Campaign(i) => match i {
			0 => 15,
			1 => 20,
			2 => 25,
			_ => 50,
		},
		GameMode::Endless(i) => 10 + i * 5,
	};
	let mut total_goats = match progress_tracker.mode {
		GameMode::Tutorial => 20,
		GameMode::Campaign(i) => match i {
			0 => 30,
			1 => 40,
			2 => 50,
			_ => 100,
		},
		GameMode::Endless(i) => goats_required + 20,
	};
	goat_numbers.total = total_goats;
	total_goats = (total_goats as isize - goat_numbers.killed as isize).clamp(0, 9999) as usize;
	for (mut text, ui_text) in text_query.iter_mut() {
		text.sections = vec![
			TextSection::new(
				format!("Goats Herded: {}\nRequired Goats: {}\nGoats Left: {}\nHeld Bell: {}", safe_goats, goats_required, total_goats, selected_bell),
				get_ui_text_style(&asset_server), 
			)
		];
	}

	if safe_goats >= goats_required {
		progress_tracker.win_timer.tick(time.delta());
		if progress_tracker.win_timer.just_finished() {
			goat_numbers.spawned = 0;
			goat_numbers.killed = 0;
			match progress_tracker.mode {
				GameMode::Tutorial => {
					progress_tracker.mode = GameMode::Tutorial;
					target_state.state = GameState::Cutscene;
					cutscene_tracker.current_scene = SceneName::TutorialOutro;
					cutscene_tracker.cutscene_state = CutsceneState::Initialize;
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
				},
				GameMode::Campaign(i) => {
					progress_tracker.max_campaign = i+1;
					progress_tracker.mode = GameMode::Campaign(i+1);
					target_state.state = GameState::Cutscene;
					cutscene_tracker.current_scene = SceneName::CampaignDay(i);
					cutscene_tracker.cutscene_state = CutsceneState::Initialize;
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
				},
				GameMode::Endless(i) => {
					progress_tracker.mode = GameMode::Endless(i+1);
					target_state.state = GameState::Cutscene;
					cutscene_tracker.current_scene = SceneName::EndlessOutro;
					cutscene_tracker.cutscene_state = CutsceneState::Initialize;
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
				},
			}
		}
	}

	if total_goats < goats_required {
		progress_tracker.lose_timer.tick(time.delta());
		if progress_tracker.lose_timer.just_finished() {
			goat_numbers.spawned = 0;
			goat_numbers.killed = 0;
			target_state.state = GameState::Cutscene;
			cutscene_tracker.current_scene = SceneName::Failure;
			cutscene_tracker.cutscene_state = CutsceneState::Initialize;
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
	}
}

#[derive(Resource)]
struct TitleTimer(Timer);

fn return_to_title(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut title_timer: ResMut<TitleTimer>,
	time: Res<Time>,
	mut commands: Commands,
	mut target_state: ResMut<TargetGameState>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	if keyboard.pressed(KeyCode::KeyP) {
		title_timer.0.tick(time.delta());
		if title_timer.0.just_finished() {
			target_state.state = GameState::Menu;
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
	}
}