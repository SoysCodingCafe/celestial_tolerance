use std::time::Duration;

// Goat module, for handling goat movement and interactions
use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, Audio};

use crate::{enemy::{Arm, GoatbirdMovement, Spitter}, helper::{cart_to_iso, iso_to_cart, GameState}, land::{Anchors, Goal, Land}, menu::SFX_SCALING, setup::{GameMode, ProgressTracker, Volume, ORTHO, REVERT}};

pub struct GoatPlugin;

impl Plugin for GoatPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(GoatNumbers{
				spawned: 0,
				killed: 0,
				total: 0,
			})
			.add_systems(OnEnter(GameState::Game), (
				goat_setup,
			))
			.add_systems(Update, (
				goat_movement,
				spawn_goat_wave,
			).run_if(in_state(GameState::Game)))
		;
	}
}

const GOAT_SPAWN_ORDER: [f32; 32] = [
	1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 
	0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
];

const GOATBIRD_SPAWN_ORDER: [f32; 32] = [
	0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
	0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0,
	0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
];

#[derive(Component)]
pub struct GoatMovement{
	pub cart_transform: Vec2,
	pub velocity: Vec2,
	pub speed: f32,
	pub boost_timer: Timer,
	pub move_timer: Timer,
	pub idle_timer: Timer,
	pub iso_mode: bool,
	pub grabbed: bool,
	pub safe: bool,
	pub furthest_tile: usize,
}

#[derive(Resource)]
pub struct GoatSpawnTimer{
	pub goat_wave: usize,
	pub goatbird_wave: usize,
	pub goat_timer: Timer,
	pub goatbird_timer: Timer,
}

#[derive(Resource)]
pub struct GoatNumbers{
	pub spawned: usize,
	pub killed: usize,
	pub total: usize,
}

fn goat_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(GoatSpawnTimer{
		goat_wave: 0,
		goatbird_wave: 0,
		goat_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
		goatbird_timer: Timer::from_seconds(0.9, TimerMode::Repeating),
	});

	// let iso_loc = cart_to_iso(cart_loc);
	// commands.spawn((SpriteBundle {
	// 	transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 600.0),
	// 	texture: asset_server.load("goat.png"),
	// 	sprite: Sprite {
	// 		color: Color::BLUE,
	// 		custom_size: Some(Vec2::new(64.0, 64.0)),
	// 		..default()
	// 	},
	// 	..default()
	// 	},
	// 	GoatMovement {
	// 		cart_transform: cart_loc,
	// 		velocity: Vec2::new(0.0, -1.0),
	// 		move_timer: Timer::from_seconds(2.0, TimerMode::Once),
	// 		idle_timer: Timer::from_seconds(0.1, TimerMode::Once),
	// 		iso_mode: true,
	// 		launched: false,
	// 	}
	// ));
}

fn goat_movement(
	mut commands: Commands,
	time: Res<Time>,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut goat_query: Query<(Entity, &mut Transform, &mut GoatMovement)>,
	land_query: Query<(&Transform, &Land), Without<GoatMovement>>,
	goal_query: Query<(&Transform, &Goal), (Without<GoatMovement>, Without<Land>)>,
	anchors: Res<Anchors>,
	audio: Res<Audio>,
	volume: Res<Volume>,
	asset_server: Res<AssetServer>,
) {
	let mut iter = goat_query.iter_combinations_mut();
	while let Some([
		(entity_a, mut transform_a, mut goat_movement_a),
		(entity_b, mut transform_b, mut goat_movement_b),
	]) = iter.fetch_next() {
		let mut offset = goat_movement_a.cart_transform - goat_movement_b.cart_transform;
		// Change for variable radius
		if offset.length() <= 32.0 {
			// let relative_velocity = goat_movement_a.velocity - goat_movement_b.velocity;
			// // Change for variable mass
			// let dp = offset * relative_velocity.dot(offset) / ((offset.length_squared()) * (200.0 + 200.0));

			// // Change for variable mass
			// goat_movement_a.velocity -= 2.0 * 200.0 * dp;
			// goat_movement_b.velocity += 2.0 * 200.0 * dp;

			// // Clamp velocity within range
			// goat_movement_a.velocity = goat_movement_a.velocity.normalize_or_zero();
			// goat_movement_b.velocity = goat_movement_b.velocity.normalize_or_zero();

			// // Change for variable radius
			// let push = (offset.normalize() * 1.01 * (32.0) - offset).extend(0.0);
			// transform_a.translation += push;
			// transform_b.translation -= push;

			// Turn away from each other
			//goat_movement_a.velocity = offset.normalize();
			//goat_movement_b.velocity = -offset.normalize();

			// Flip with variance
			// let theta = (rand::random::<f32>() * 180.0 - 90.0).to_radians();
			// Remove normalization for yeet
			// offset = offset.normalize();
			// Change offset to goat_movement_b.velocity for flocking
			// goat_movement_a.velocity = Vec2::new(
			// 	offset.x * theta.cos() - offset.y * theta.sin(),
			// 	offset.x * theta.sin() + offset.y * theta.cos(),
			// );
			// goat_movement_b.velocity = -Vec2::new(
			// 	offset.x * theta.cos() - offset.y * theta.sin(),
			// 	offset.x * theta.sin() + offset.y * theta.cos(),
			// );
		}
	}

	for (entity, mut transform, mut goat_movement) in goat_query.iter_mut() {
		goat_movement.boost_timer.tick(time.delta());
		if goat_movement.boost_timer.finished() {
			goat_movement.speed = 90.0;
		} else {
			goat_movement.speed = 200.0;
		}
		// goat_movement.furthest_tile = 0;
		// for (land_transform, land) in land_query.iter() {
		// 	if (goat_movement.cart_transform.x - land_transform.translation.x).abs() < land.half_size.x
		// 	&& (goat_movement.cart_transform.y - land_transform.translation.y).abs() < land.half_size.y {
		// 		if land.tile_index > goat_movement.furthest_tile {
		// 			goat_movement.furthest_tile = land.tile_index.clamp(0, 5);
		// 		}
		// 	}
		// }
		let target = goat_movement.cart_transform + goat_movement.velocity * goat_movement.speed * time.delta_seconds() - iso_to_cart(Vec2::new(0.0, 32.0));
		let mut grounded = false;
		if goat_movement.safe {
			for (goal_transform, goal) in goal_query.iter() {
				if (target.x - goal_transform.translation.x).abs() < goal.half_size.x
				&& (target.y - goal_transform.translation.y).abs() < goal.half_size.y {
					grounded = true;
					break;
				}
			}
		} else {
			for (land_transform, land) in land_query.iter() {
				if (target.x - land_transform.translation.x).abs() < land.half_size.x
				&& (target.y - land_transform.translation.y).abs() < land.half_size.y {
					// goat_movement.cart_transform.x = goat_movement.cart_transform.x.clamp(land_transform.translation.x - land.half_size.x + 1.0, land_transform.translation.x + land.half_size.x - 1.0);
					// goat_movement.cart_transform.y = goat_movement.cart_transform.y.clamp(land_transform.translation.y - land.half_size.y + 1.0, land_transform.translation.y + land.half_size.y - 1.0);
					goat_movement.furthest_tile = land.tile_index.clamp(0, 5);
					//println!("New tile: {}", goat_movement.furthest_tile);
					grounded = true;
				}
			}
			for (goal_transform, goal) in goal_query.iter() {
				if (target.x - goal_transform.translation.x).abs() < goal.half_size.x
				&& (target.y - goal_transform.translation.y).abs() < goal.half_size.y {
					goat_movement.safe = true;
					break;
				}
			}
		}

		if !grounded {
			let rnd = rand::random::<f32>();
			if rnd < 0.25 {
				let rnd = rand::random::<f32>();
				let path = if rnd < 0.3 {
					"sfx/goat_short.ogg"
				} else if rnd < 0.6 {
					"sfx/goat_short_far.ogg"
				} else {
					"sfx/goat_meh_far.ogg"
				};
				audio.play(asset_server.load(path)).with_volume((volume.sfx*SFX_SCALING).powf(2.0));
			}
			// goat_movement.furthest_tile = 0;
			// for (land_transform, land) in land_query.iter() {
			// 	if (goat_movement.cart_transform.x - land_transform.translation.x).abs() < land.half_size.x
			// 	&& (goat_movement.cart_transform.y - land_transform.translation.y).abs() < land.half_size.y {
			// 		if land.tile_index > goat_movement.furthest_tile {
			// 			goat_movement.furthest_tile = land.tile_index.clamp(0, 5);
			// 		}
			// 	}
			// }
			// commands.entity(entity).despawn_recursive();
			// transform.translation.x = 0.0;
			// transform.translation.y = 0.0;
			// continue;
			//goat_movement.velocity = -goat_movement.velocity;

			let loc = goat_movement.cart_transform - iso_to_cart(Vec2::new(0.0, 32.0));
			goat_movement.velocity = (anchors.0[goat_movement.furthest_tile] - loc).normalize();
			// println!("Furthest tile: {}", goat_movement.furthest_tile);
			// for (land_transform, land) in land_query.iter() {
			// 	if (loc.x - land_transform.translation.x).abs() < land.half_size.x
			// 	&& (loc.y - land_transform.translation.y).abs() < land.half_size.y {
			// 		goat_movement.velocity = (land_transform.translation.xy() - loc).normalize();
			// 	}
			// }

			//goat_movement.move_timer.tick(Duration::from_secs(120));
		} else {
			// let theta = ((rand::random::<f32>() - 0.5) * 10.0).to_radians();
			// goat_movement.velocity = goat_movement.velocity.rotate(Vec2::from_angle(theta));
		}

		let goat_vel = if !goat_movement.iso_mode {
			goat_movement.velocity
		} else {
			cart_to_iso(goat_movement.velocity)
		};
		
		if !goat_movement.move_timer.finished() && !goat_movement.grabbed {
			goat_movement.move_timer.tick(time.delta());
			if goat_movement.move_timer.just_finished() {
				goat_movement.idle_timer.reset();
			}
			transform.translation.x = transform.translation.x + goat_vel.x * goat_movement.speed * time.delta_seconds();
			transform.translation.y = transform.translation.y + goat_vel.y * goat_movement.speed * time.delta_seconds();
			if goat_movement.iso_mode {
				goat_movement.cart_transform = iso_to_cart(transform.translation.xy());
			} else {
				goat_movement.cart_transform = transform.translation.xy();
			}
		} else {
			goat_movement.idle_timer.tick(time.delta());
			if goat_movement.idle_timer.just_finished() {
				goat_movement.move_timer.reset();

				//let theta = 30.0_f32.to_radians(); 
				let theta = (rand::random::<f32>() * 90.0 - 45.0).to_radians();

				goat_movement.velocity = Vec2::new(
					goat_movement.velocity.x * theta.cos() - goat_movement.velocity.y * theta.sin(),
					goat_movement.velocity.x * theta.sin() + goat_movement.velocity.y * theta.cos(),
				)
			}
		}
		
		// if keyboard.just_pressed(KeyCode::KeyI) && !goat_movement.iso_mode {
		// 	let iso_coords = cart_to_iso(transform.translation.xy());
		// 	transform.translation.x = iso_coords.x;
		// 	transform.translation.y = iso_coords.y;
		// 	goat_movement.iso_mode = true;
		// }
		
		// if keyboard.just_pressed(KeyCode::KeyC) && goat_movement.iso_mode {
		// 	let iso_coords = iso_to_cart(transform.translation.xy());
		// 	transform.translation.x = iso_coords.x;
		// 	transform.translation.y = iso_coords.y;
		// 	goat_movement.iso_mode = false;
		// }
	}
}

fn spawn_goat_wave(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut goat_spawn_timer: ResMut<GoatSpawnTimer>,
	mut goat_numbers: ResMut<GoatNumbers>,
	progress_tracker: Res<ProgressTracker>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	let speed_up = match progress_tracker.mode {
		GameMode::Tutorial => 1.0,
		GameMode::Campaign(i) => if i < 1 {1.0} else if i < 2 {1.5} else {2.0},
		GameMode::Endless(i) => if i < 1 {1.0} 
		else if i < 2 {2.0} else {3.0},
	};
	goat_spawn_timer.goat_timer.tick(time.delta().mul_f32(speed_up));
	if goat_spawn_timer.goat_timer.just_finished() {
		if GOAT_SPAWN_ORDER[goat_spawn_timer.goat_wave] == 1.0 && goat_numbers.spawned < goat_numbers.total {
			goat_numbers.spawned += 1;
			let cart_loc = Vec2::new(-800.0, 50.0)*REVERT;
			let iso_loc = cart_to_iso(cart_loc);
			commands.spawn((SpriteBundle {
				transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 600.0),
				texture: asset_server.load("goat.png"),
				sprite: Sprite {
					// color: Color::RED,
					custom_size: Some(Vec2::new(64.0, 64.0)),
					..default()
				},
				..default()
				},
				GoatMovement {
					cart_transform: cart_loc,
					velocity: Vec2::new(1.0, 0.0),
					speed: 90.0,
					boost_timer: Timer::from_seconds(3.0, TimerMode::Once).tick(Duration::from_secs(3)).clone(),
					move_timer: Timer::from_seconds(3.0, TimerMode::Once),
					idle_timer: Timer::from_seconds(1.0, TimerMode::Once),
					iso_mode: true,
					grabbed: false,
					safe: false,
					furthest_tile: 0,
				}
			));
		}
		goat_spawn_timer.goat_wave = (goat_spawn_timer.goat_wave + 1) % 32;
	}
	let speed_up = match progress_tracker.mode {
		GameMode::Tutorial => 1.0,
		GameMode::Campaign(i) => if i < 1 {1.0} else if i < 2 {1.2} else {1.4},
		GameMode::Endless(i) => if i < 1 {1.0} 
		else if i < 3 {1.5} else {2.0},
	};
	goat_spawn_timer.goatbird_timer.tick(time.delta().mul_f32(speed_up));
	if goat_spawn_timer.goatbird_timer.just_finished() {
 		if GOATBIRD_SPAWN_ORDER[goat_spawn_timer.goatbird_wave] == 1.0 {
			let rand_loc = rand::random::<f32>() * 0.33;
			let iso_loc = Vec2::new(rand_loc*960.0 + 640.0, 600.0);
				// if rand_loc < 0.5 {Vec2::new(-500.0, 500.0)*REVERT}
				// else if rand_loc < 0.8 {Vec2::new(-500.0, -500.0)*REVERT}
				// else {Vec2::new(500.0, 500.0)*REVERT};
			let cart_loc = iso_to_cart(iso_loc);
			commands.spawn((SpriteBundle {
				transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 800.0),
				texture: asset_server.load("goatbird.png"),
				sprite: Sprite {
					//color: Color::RED,
					custom_size: Some(Vec2::new(64.0, 64.0)),
					..default()
				},
				..default()
				},
				GoatbirdMovement {
					cart_transform: cart_loc,
					velocity: Vec2::new(1.0, 0.0),
					speed: 110.0,
					feeding: false,
					feed_timer: Timer::from_seconds(3.0, TimerMode::Once),
					scared: false,
				}
			));
		} else if GOATBIRD_SPAWN_ORDER[goat_spawn_timer.goatbird_wave] == 2.0 {
			match progress_tracker.mode {
				GameMode::Campaign(i) | GameMode::Endless(i) => {
					if i > 0 {
						let rnd = rand::random::<f32>();
						let cart_loc = if rnd < 0.33 {Vec2::new(-400.0, 225.0)*REVERT}
						else if rnd < 0.66 {Vec2::new(175.0, 50.0)*REVERT}
						else {Vec2::new(175.0, -175.0)*REVERT};
						let mut iso_loc = cart_to_iso(cart_loc);
						iso_loc.y += 16.0;
						commands.spawn((SpriteBundle {
							transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 800.0),
							texture: asset_server.load("spitter.png"),
							sprite: Sprite {
								custom_size: Some(Vec2::new(128.0, 128.0)),
								..default()
							},
							..default()
							},
							Spitter{
								cart_transform: iso_to_cart(iso_loc),
								charge_timer: Timer::from_seconds(3.0, TimerMode::Once),
								spit_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
							}
						));
					}
				}
				_ => (),
			}

		} else if GOATBIRD_SPAWN_ORDER[goat_spawn_timer.goatbird_wave] == 3.0 {
			// match progress_tracker.mode {
			// 	GameMode::Campaign(i) | GameMode::Endless(i) => {
			// 		if true {//i > 2 {
			// 			let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(1920.0, 1080.0), 2, 1, Some(Vec2::ZERO), Some(Vec2::ZERO)));
			// 			commands.spawn((SpriteSheetBundle {
			// 				transform: Transform::from_xyz(0.0, 0.0,  750.0),
			// 				texture: asset_server.load("arm.png"),
			// 				atlas: TextureAtlas{
			// 					layout: texture_atlas_layout,
			// 					index: 1,
			// 				},
			// 				sprite: Sprite {
			// 					custom_size: Some(ORTHO),
			// 					..default()
			// 				},
			// 				..default()
			// 				},
			// 				Arm{
			// 					wait_timer: Timer::from_seconds(4.0, TimerMode::Once),
			// 					slam_timer: Timer::from_seconds(4.0, TimerMode::Once),
			// 				}
			// 			));
			// 		}
			// 	},
			// 	_ => (),
			// }
		}
		goat_spawn_timer.goatbird_wave = (goat_spawn_timer.goatbird_wave + 1) % 32;
	}
}