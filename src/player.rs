// Player module, for handling player movement and interactions
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use std::{f32::consts::PI, time::Duration};

use crate::{enemy::{GoatbirdMovement, Spit, Spitter}, goat::GoatMovement, helper::{cart_to_iso, iso_to_cart, GameState}, land::Land, menu::SFX_SCALING, setup::{BellEvent, Volume, ORTHO, REVERT}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::Game), (
				player_setup,
			))
			.add_systems(Update, (
				player_control,
				player_jump,
				player_movement,
				player_bell,
				move_bell_sel,
				sort_z_layer,
				sort_sprite_flip,
			).run_if(in_state(GameState::Game)))
		;
	}
}

#[derive(Component)]
struct BellSel;

#[derive(Component)]
struct Shadow;

#[derive(Component)]
struct BellCooldown(usize);

#[derive(Component)]
pub struct PlayerMovement{
	cart_transform: Vec2,
	velocity: Vec2,
	airtime: Timer,
	max_vel: f32,
	acceleration: f32,
	friction: f32,
	iso_mode: bool,
	iso_move: bool,
	pub selected_bell: f32,
	total_bells: f32,
	bell_cooldown: [Timer; 5],
}

fn player_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let cart_loc = Vec2::new(750.0, -50.0)*REVERT;
	let iso_loc = cart_to_iso(cart_loc);
	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 600.0),
		texture: asset_server.load("goatherd.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(64.0, 64.0)),
			..default()
		},
		..default()
		},
		PlayerMovement{
			cart_transform: cart_loc,
			velocity: Vec2::ZERO,
			airtime: Timer::from_seconds(1.0, TimerMode::Once).tick(Duration::from_secs(2)).clone(),
			max_vel: 5.0,
			acceleration: 40.0,
			friction: 12.0,
			iso_mode: true,
			iso_move: true,
			selected_bell: 0.0,
			total_bells: 5.0,
			bell_cooldown: [
				Timer::from_seconds(2.0, TimerMode::Once).tick(Duration::from_secs(16)).clone(),
				Timer::from_seconds(1.2, TimerMode::Once).tick(Duration::from_secs(16)).clone(),
				Timer::from_seconds(16.0, TimerMode::Once).tick(Duration::from_secs(16)).clone(),
				Timer::from_seconds(3.0, TimerMode::Once).tick(Duration::from_secs(16)).clone(),
				Timer::from_seconds(1.0, TimerMode::Once).tick(Duration::from_secs(16)).clone(),
			],
		},
	)).with_children(|parent| {
		let shadow_iso_loc = Vec2::new(0.0, -32.0);
		parent
			.spawn((SpriteBundle {
				transform: Transform::from_xyz(shadow_iso_loc.x, shadow_iso_loc.y, -100.0),
				texture: asset_server.load("shadow.png"),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, 0.6),
					custom_size: Some(Vec2::new(64.0, 64.0)),
					..default()
				},
				..default()
			},
			Shadow,
		));
	});

	for i in 0..5 {
		commands.spawn((SpriteBundle {
			transform: Transform::from_xyz(-222.0 + (96.0 + 15.0) * i as f32, -482.0, 552.0),
			sprite: Sprite {
				color: Color::rgba(0.8, 0.8, 0.2, 0.1),
				custom_size: Some(Vec2::new(96.0, 96.0)),
				..default()
			},
			..default()
			},
			BellCooldown(i),
		));
	}

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(-222.0 + (96.0 + 15.0) * 4.0, -410.0, 552.0),
		texture: asset_server.load("bell_sel.png"),
		sprite: Sprite {
			custom_size: Some(Vec2::new(50.0, 30.0)),
			..default()
		},
		..default()
		},
		BellSel,
	));

	// let iso_loc = cart_to_iso(cart_loc);
	// commands.spawn((SpriteBundle {
	// 	transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 600.0),
	// 	texture: asset_server.load("goatherd.png"),
	// 	sprite: Sprite {
	// 		color: Color::BLUE,
	// 		custom_size: Some(Vec2::new(64.0, 64.0)),
	// 		..default()
	// 	},
	// 	..default()
	// 	},
	// 	PlayerMovement{
	// 		cart_transform: cart_loc,
	// 		velocity: Vec2::ZERO,
	// 		grounded: true,
	// 		airtime: 0.0,
	// 		max_vel: 10.0,
	// 		acceleration: 60.0,
	// 		friction: 10.0,
	// 		iso_mode: true,
	// 	},
	// ));
}

fn move_bell_sel(
	mut bell_sel_query: Query<&mut Transform, With<BellSel>>,
	player_query: Query<(&PlayerMovement)>,
	mut bell_cooldown_query: Query<(&mut Transform, &BellCooldown), Without<BellSel>>,
) {
	for (player) in player_query.iter() {
		for (mut transform) in bell_sel_query.iter_mut() {
			transform.translation.x = -223.0 + (96.0 + 15.0) * player.selected_bell;
		}
		for (mut transform, bell_index) in bell_cooldown_query.iter_mut() {
			let frac = player.bell_cooldown[bell_index.0].fraction();
			transform.scale.y = frac;
			transform.translation.y = -482.0 - (1.0-frac)*96.0/2.0;
		}
	}
}

fn player_control(
	time: Res<Time>,
	keyboard: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<(&mut PlayerMovement)>,
) {
	let mut mov_dir = Vec2::splat(0.0);
	if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
		mov_dir.x -= 1.0;
	}
	if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
		mov_dir.x += 1.0;
	}
	if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
		mov_dir.y += 1.0;
	}
	if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
		mov_dir.y -= 1.0;
	}
	for (mut player_movement) in player_query.iter_mut() {
		if player_movement.iso_move {
			player_movement.velocity = (player_movement.velocity + cart_to_iso(mov_dir) * player_movement.acceleration * time.delta_seconds()).clamp_length_max(player_movement.max_vel);
			//player_movement.velocity = (player_movement.velocity + mov_dir * player_movement.acceleration * time.delta_seconds()).clamp_length_max(player_movement.max_vel);
		} else {
			player_movement.velocity = (player_movement.velocity + mov_dir * player_movement.acceleration * time.delta_seconds()).clamp_length_max(player_movement.max_vel);
		}
		player_movement.velocity = player_movement.velocity / (1.0 + player_movement.friction * time.delta_seconds());
	}
}

fn player_movement(
	keyboard: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut player_query: Query<(&mut Transform, &mut PlayerMovement)>,
	mut shadow_query: Query<&mut Transform, (With<Shadow>, Without<PlayerMovement>)>,
	land_query: Query<(&Transform, &Land), (Without<Shadow>, Without<PlayerMovement>)>,
) {
	for (mut player_transform, mut player_movement) in player_query.iter_mut() {
		let target = player_movement.cart_transform + player_movement.velocity - iso_to_cart(Vec2::new(0.0, 32.0));
		let mut grounded = false;
		for (transform, land) in land_query.iter() {
			if (target.x - transform.translation.x).abs() < land.half_size.x
			&& (target.y - transform.translation.y).abs() < land.half_size.y {
				grounded = true;
				break;
			}
		}

		if !grounded && player_movement.airtime.finished() {
			//player_movement.velocity = -player_movement.velocity;
			let cart_loc = Vec2::new(750.0, -50.0)*REVERT;
			let iso_loc = cart_to_iso(cart_loc);
			player_transform.translation.x = iso_loc.x;
			player_transform.translation.y = iso_loc.y;
			player_movement.cart_transform = cart_loc;
		}

		player_transform.translation.x = player_transform.translation.x + player_movement.velocity.x;
		player_transform.translation.y = player_transform.translation.y + player_movement.velocity.y;
		if !player_movement.airtime.finished() {
			player_transform.translation.y = player_transform.translation.y + (player_movement.airtime.fraction() * PI).cos() * 80.0 * time.delta_seconds();
			for mut child_transform in shadow_query.iter_mut() {
				child_transform.scale.x = (player_movement.airtime.fraction() * 2.0 * PI).cos().clamp(0.4, 1.0);
				child_transform.scale.y = (player_movement.airtime.fraction() * 2.0 * PI).cos().clamp(0.4, 1.0);
				child_transform.translation.y = child_transform.translation.y - (player_movement.airtime.fraction() * PI).cos() * 80.0 * time.delta_seconds();
			}
		} else if player_movement.airtime.just_finished() {
			for mut child_transform in shadow_query.iter_mut() {
				child_transform.translation.y = -32.0;
			}
		}

		if player_movement.iso_mode {
			player_movement.cart_transform = iso_to_cart(player_transform.translation.xy());
		} else {
			player_movement.cart_transform = player_transform.translation.xy();
		}

		// if keyboard.just_pressed(KeyCode::KeyP) {
		// 	println!("Player {} position: {}", if player_movement.iso_mode{"iso"}else{"cart"}, if player_movement.iso_mode{player_movement.cart_transform}else{transform.translation.xy()});
		// 	println!("Locs: {}, {}, {}, {}", iso_to_cart(Vec2::new(800.0, 450.0)), iso_to_cart(Vec2::new(-800.0, 450.0)), iso_to_cart(Vec2::new(800.0, -450.0)), iso_to_cart(Vec2::new(-800.0, -450.0)),)
		// }

		if keyboard.just_pressed(KeyCode::KeyI) && !player_movement.iso_move {
			// let iso_coords = cart_to_iso(transform.translation.xy());
			// transform.translation.x = iso_coords.x;
			// transform.translation.y = iso_coords.y;
			player_movement.iso_move = true;
		}
		
		if keyboard.just_pressed(KeyCode::KeyC) && player_movement.iso_move {
			// let iso_coords = iso_to_cart(transform.translation.xy());
			// transform.translation.x = iso_coords.x;
			// transform.translation.y = iso_coords.y;
			player_movement.iso_move = false;
		}
	}
}

fn player_jump(
	mut player_query: Query<&mut PlayerMovement>,
	keyboard: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	for mut player in player_query.iter_mut() {
		player.airtime.tick(time.delta());
		if keyboard.just_pressed(KeyCode::Space) && player.airtime.finished() {
			player.airtime.reset();
		}
	}
}

fn player_bell(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<(&Transform, &mut PlayerMovement)>,
	mut goat_query: Query<&mut GoatMovement>,
	mut goatbird_query: Query<&mut GoatbirdMovement>,
	spitter_query: Query<(Entity, &Spitter)>,
	mut spit_query: Query<(Entity, &mut Spit), Without<Spitter>>,
	mut ev_w_bell: EventWriter<BellEvent>,
	time: Res<Time>,
	mut commands: Commands,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	volume: Res<Volume>,
) {
	let e = keyboard.just_pressed(KeyCode::KeyE);
	let k = keyboard.just_pressed(KeyCode::KeyK);
	let q = keyboard.just_pressed(KeyCode::KeyQ);
	let j = keyboard.pressed(KeyCode::KeyJ);

	for (_, mut player_movement) in player_query.iter_mut() {
		for i in 0..player_movement.total_bells as usize {
			player_movement.bell_cooldown[i].tick(time.delta());
		}
		if e || k {
			player_movement.selected_bell = (player_movement.selected_bell + 1.0) % player_movement.total_bells;
		}
		if q {
			player_movement.selected_bell = (player_movement.selected_bell - 1.0);
			if player_movement.selected_bell == -1.0 {player_movement.selected_bell = player_movement.total_bells - 1.0};
		}
	}

	if j {
		for (transform, mut player_movement) in player_query.iter_mut() {
			let selected_bell = player_movement.selected_bell as usize;
			if player_movement.bell_cooldown[selected_bell].finished() {
				let rnd = rand::random::<f32>();
					let path = if rnd < 0.3 {
						"sfx/bell.ogg"
					} else if rnd < 0.6 {
						"sfx/bell_long.ogg"
					} else {
						"sfx/bell_quick.ogg"
					};
					audio.play(asset_server.load(path)).with_volume((volume.sfx*SFX_SCALING).powf(2.0));
				player_movement.bell_cooldown[selected_bell].reset();
				let max_distance = match selected_bell {
					0 => 300.0,
					1 => 200.0,
					2 => 1600.0,
					3 => 500.0,
					_ => 500.0,
				};
				ev_w_bell.send(BellEvent{location: transform.translation.xy(), selected_bell: selected_bell});
				for mut goat_movement in goat_query.iter_mut() {
					let g_t_p = player_movement.cart_transform - goat_movement.cart_transform;
					let distance = g_t_p.length();
					if distance > 0.0 && distance < max_distance {
						goat_movement.grabbed = false;
						goat_movement.move_timer.reset();
						goat_movement.move_timer.tick(Duration::from_millis((rand::random::<f32>() * 2500.0) as u64));
						match selected_bell {
							4 => {
								let gp_v = g_t_p.normalize();
								let theta_offset = (rand::random::<f32>() * -20.0).to_radians();
								goat_movement.velocity = gp_v.rotate(Vec2::from_angle((90.0 + theta_offset).to_radians()))
							}
							3 => {
								goat_movement.boost_timer.reset();
							}
							_ => {
								let mut gp_v = g_t_p.normalize();
								if selected_bell == 1 {gp_v = -gp_v};
								let gv_v = goat_movement.velocity.normalize();
								let theta = gv_v.angle_between(gp_v);
								//println!("Theta: {}", theta.to_degrees());
								let range = distance/max_distance;
								let theta_offset = ((rand::random::<f32>() - 0.5) * range * 60.0).to_radians();
								goat_movement.velocity = goat_movement.velocity.rotate(Vec2::from_angle(theta + theta_offset));
							}					
						}
					}
				}
				for mut goatbird_movement in goatbird_query.iter_mut() {
					if !goatbird_movement.scared {
						let g_t_p = player_movement.cart_transform - goatbird_movement.cart_transform;
						let distance = g_t_p.length();
						if distance > 0.0 && distance < max_distance && selected_bell < 3 {
							let mut gp_v = g_t_p.normalize();
							let gv_v = goatbird_movement.velocity.normalize();
							let theta = gv_v.angle_between(gp_v);
							let range = distance/max_distance;
							let theta_offset = ((rand::random::<f32>() - 0.5) * range * 60.0).to_radians();
							goatbird_movement.velocity = -goatbird_movement.velocity.rotate(Vec2::from_angle(theta + theta_offset));
							goatbird_movement.feeding = false;
							goatbird_movement.scared = true;
						}
					}
				}
				for (entity, spitter) in spitter_query.iter() {
					let g_t_p = player_movement.cart_transform - spitter.cart_transform;
					let distance = g_t_p.length();
					if distance > 0.0 && distance < max_distance && selected_bell < 3 {
						commands.entity(entity).despawn_recursive();
					}
				}
				for (entity, mut spit) in spit_query.iter_mut() {
					let g_t_p = player_movement.cart_transform - spit.cart_transform;
					let distance = g_t_p.length();
					if distance > 0.0 && distance < max_distance && selected_bell < 3 {
						if selected_bell == 1 {
							if !spit.scared {
								spit.velocity = -g_t_p.normalize();
								spit.scared = true;
							}
						} else {
							commands.entity(entity).despawn_recursive();
						}
					}
				}
			}
		}
	}
}

fn sort_z_layer(
	mut sortable_query: Query<&mut Transform, Or<(With<PlayerMovement>, With<GoatMovement>)>>,
) {
	for mut transform in sortable_query.iter_mut() {
		transform.translation.z = (1.0 - (transform.translation.y + ORTHO.y/2.0) / ORTHO.y) + 600.0;
	}
}

fn sort_sprite_flip(
	mut player_query: Query<(&mut Sprite, &PlayerMovement)>,
	mut shadow_query: Query<&mut Sprite, (With<Shadow>, Without<PlayerMovement>)>,
	mut goat_query: Query<(&mut Sprite,  &GoatMovement), (Without<Shadow>, Without<PlayerMovement>)>,
	mut goatbird_query: Query<(&mut Sprite,  &GoatbirdMovement), (Without<Shadow>, Without<PlayerMovement>, Without<GoatMovement>)>,
) {
	for (mut player_sprite, player_movement) in player_query.iter_mut() {
		for mut shadow_sprite in shadow_query.iter_mut() {
			let x_comp = player_movement.velocity.x;
			if x_comp > 0.0 {
				player_sprite.flip_x = false;
				shadow_sprite.flip_x = false;
			} else if x_comp < 0.0 {
				player_sprite.flip_x = true;
				shadow_sprite.flip_x = true;
			}
		}
	}

	for (mut goat_sprite, goat_movement) in goat_query.iter_mut() {
		let x_comp = cart_to_iso(goat_movement.velocity).x;
		if x_comp > 0.0 {
			goat_sprite.flip_x = true;
		} else if x_comp < 0.0 {
			goat_sprite.flip_x = false;
		}
	}

	for (mut goatbird_sprite, goatbird_movement) in goatbird_query.iter_mut() {
		if !goatbird_movement.feeding {
			let x_comp = cart_to_iso(goatbird_movement.velocity).x;
			if x_comp > 0.0 {
				goatbird_sprite.flip_x = true;
			} else if x_comp < 0.0 {
				goatbird_sprite.flip_x = false;
			}
		}
	}
}