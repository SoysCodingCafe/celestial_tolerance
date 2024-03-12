// Enemy module, for handling enemy movement and interactions
use bevy::prelude::*;

use crate::{goat::{GoatMovement, GoatNumbers}, helper::{cart_to_iso, iso_to_cart, GameState}, setup::{ORTHO, REVERT}};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, (
				goatbird_movement,
				spitter_stuff,
				arm_stuff,
			).run_if(in_state(GameState::Game)))
		;
	}
}

#[derive(Component)]
pub struct GoatbirdMovement{
	pub cart_transform: Vec2,
	pub velocity: Vec2,
	pub speed: f32,
	pub feeding: bool,
	pub feed_timer: Timer,
	pub scared: bool,
}

#[derive(Component)]
pub struct Spitter{
	pub cart_transform: Vec2,
	pub charge_timer: Timer,
	pub spit_timer: Timer,
}

fn goatbird_movement(
	time: Res<Time>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut goatbird_query: Query<(Entity, &mut Transform, &mut GoatbirdMovement)>,
	mut goat_query: Query<(Entity, &mut Transform, &mut GoatMovement), Without<GoatbirdMovement>>,
	mut goat_numbers: ResMut<GoatNumbers>,
) {
	for (goatbird_entity, mut goatbird_transform, mut goatbird_movement) in goatbird_query.iter_mut() {
		if !goatbird_movement.scared {
			let mut closest_goat = 99999.0;
			let mut closest_goat_loc = Vec2::new(0.0, 800.0);
			for (goat_entity, goat_transform, goat_movement) in goat_query.iter_mut() {
				let distance = (goat_movement.cart_transform + iso_to_cart(Vec2::new(0.0, 16.0)) - goatbird_movement.cart_transform).length();
				if distance < closest_goat && !goat_movement.safe {
					closest_goat = distance;
					closest_goat_loc = goat_movement.cart_transform + iso_to_cart(Vec2::new(0.0, 16.0));
				}
			}
			goatbird_movement.velocity = (closest_goat_loc - goatbird_movement.cart_transform).normalize_or_zero();
			if closest_goat < 16.0 {
				goatbird_movement.feeding = true;
				goatbird_movement.feed_timer.tick(time.delta());
				for (goat_entity, goat_transform, mut goat_movement) in goat_query.iter_mut() {
					if (goat_movement.cart_transform + iso_to_cart(Vec2::new(0.0, 16.0)) - goatbird_movement.cart_transform).length() < 16.0 {
						goat_movement.grabbed = true;
					}
				}
				if goatbird_movement.feed_timer.just_finished() {
					goatbird_movement.feeding = false;
					for (goat_entity, goat_transform, mut goat_movement) in goat_query.iter_mut() {
						let distance = (goat_movement.cart_transform + iso_to_cart(Vec2::new(0.0, 16.0)) - goatbird_movement.cart_transform).length();
						if distance < 16.0 && !goat_movement.safe {
							commands.entity(goat_entity).despawn_recursive();
							goatbird_movement.scared = true;
							goatbird_movement.velocity = goatbird_movement.cart_transform.normalize();
							let corpse = commands.spawn((SpriteBundle {
								transform: Transform::from_xyz(0.0, -16.0, -1.0),
								texture: asset_server.load("goat.png"),
								sprite: Sprite {
									color: Color::GRAY,
									flip_x: goatbird_movement.velocity.x > 0.0,
									custom_size: Some(Vec2::new(64.0, 64.0)),
									..default()
								},
								..default()
								},
							)).id();
							commands.entity(goatbird_entity).add_child(corpse);
							goat_numbers.killed += 1;
							break;
						}
					}
				}
			} else {
				goatbird_movement.feeding = false;
				goatbird_movement.feed_timer.reset();
			}
		} 
		let goatbird_vel = cart_to_iso(goatbird_movement.velocity);
		if !goatbird_movement.feeding {
			goatbird_transform.translation.x = goatbird_transform.translation.x + goatbird_vel.x * goatbird_movement.speed * time.delta_seconds();
			goatbird_transform.translation.y = goatbird_transform.translation.y + goatbird_vel.y * goatbird_movement.speed * time.delta_seconds();
		}
		goatbird_movement.cart_transform = iso_to_cart(goatbird_transform.translation.xy());
		if goatbird_movement.scared {
			if goatbird_transform.translation.x.abs() > ORTHO.x + 64.0
			&& goatbird_transform.translation.y.abs() > ORTHO.y + 64.0 {
				commands.entity(goatbird_entity).despawn_recursive();
			}
		}
	}
}

#[derive(Component)]
pub struct Spit{
	pub scared: bool,
	pub velocity: Vec2,
	pub cart_transform: Vec2,
}

#[derive(Component)]
pub struct Arm{
	pub wait_timer: Timer,
	pub slam_timer: Timer,
}

fn spitter_stuff(
	mut commands: Commands,
	time: Res<Time>,
	mut spitter_query: Query<(&mut Spitter)>,
	mut spit_query: Query<(Entity, &mut Transform, &mut Spit)>,
	goat_query: Query<(Entity, &GoatMovement)>,
	asset_server: Res<AssetServer>,
	mut goat_numbers: ResMut<GoatNumbers>,
) {
	for (mut spitter) in spitter_query.iter_mut() {
		if !spitter.charge_timer.finished() {
			spitter.charge_timer.tick(time.delta());
		} else {
			spitter.spit_timer.tick(time.delta());
			if spitter.spit_timer.just_finished() {
				let mut target = Vec2::ZERO;
				for (_, goat) in goat_query.iter() {
					target = goat.cart_transform;
				}
				let theta = ((rand::random::<f32>() - 0.5) * 10.0).to_radians();
				let iso_loc = cart_to_iso(spitter.cart_transform);
				commands.spawn((SpriteBundle {
					transform: Transform::from_xyz(iso_loc.x, iso_loc.y, 800.0),
					texture: asset_server.load("spit.png"),
					sprite: Sprite {
						color: Color::RED,
						custom_size: Some(Vec2::new(32.0, 32.0)),
						..default()
					},
					..default()
					},
					Spit{
						scared: false,
						velocity: (target-spitter.cart_transform).normalize().rotate(Vec2::from_angle(theta)),
						cart_transform: spitter.cart_transform
					},
				));
			}
		}
	}
	for (spit_entity, mut transform, mut spit) in spit_query.iter_mut() {
		for (goat_entity, goat) in goat_query.iter() {
			if (goat.cart_transform - iso_to_cart(transform.translation.xy())).length() < 24.0 {
				commands.entity(goat_entity).despawn_recursive();
				commands.entity(spit_entity).despawn_recursive();
				goat_numbers.killed += 1;
				break;
			}
		}
		let dir = cart_to_iso(spit.velocity);
		transform.translation.x += dir.x * 300.0 * time.delta_seconds();
		transform.translation.y += dir.y * 300.0 * time.delta_seconds();
		spit.cart_transform = iso_to_cart(transform.translation.xy());
		if transform.translation.x.abs() > 1000.0
		|| transform.translation.y.abs() > 600.0 {
			commands.entity(spit_entity).despawn_recursive();
		}
	}
}

fn arm_stuff(
	mut commands: Commands,
	time: Res<Time>,
	mut arm_query: Query<(Entity, &mut Transform, &mut TextureAtlas, &mut Arm)>,
	goat_query: Query<(Entity, &GoatMovement)>,
	mut goat_numbers: ResMut<GoatNumbers>,
) {
	for (entity, mut transform, mut atlas, mut arm) in arm_query.iter_mut() {
		if !arm.wait_timer.finished() {
			arm.wait_timer.tick(time.delta());
			if arm.wait_timer.just_finished() {
				arm.slam_timer.reset();
				atlas.index = 0;
				transform.translation.z = 553.0;
				for (entity, goat) in goat_query.iter() {
					if (goat.cart_transform.x - 150.0*REVERT).abs() < 100.0*REVERT
					&& (goat.cart_transform.y - 300.0*REVERT).abs() < 150.0*REVERT {
						commands.entity(entity).despawn_recursive();
						goat_numbers.killed += 1;
					}
				}
			}
		} else if !arm.slam_timer.finished() {
			arm.slam_timer.tick(time.delta());
			if arm.slam_timer.just_finished() {
				arm.wait_timer.reset();
				atlas.index = 1;
				transform.translation.z = 553.0;
			}
		}
	}
}