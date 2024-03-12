use bevy::prelude::*;

use crate::menu::{GoatHead, ScreenFade};

// STATES
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	PreConfig,
	Config,
	Transition,
	Menu,
	Cutscene,
	Game,
}

// COMPONENTS
// #[derive(Component)]
// struct DontDespawnOnLoad;

// RESOURCES

// HELPER FUNCTIONS

// pub fn despawn_everything(
// 	mut commands: Commands,
// 	despawn_query: Query<Entity, Without<Window>>,
// ) {
// 	for entity in &despawn_query {
// 		commands.entity(entity).despawn_recursive();
// 	}
// }

// pub fn despawn_entities_with<T: Component>(
// 	mut commands: Commands,
// 	despawn_query: Query<Entity, With<T>>,
// ) {
// 	for entity in &despawn_query {
// 		commands.entity(entity).despawn_recursive();
// 	}
// }

pub fn despawn_entities_without<T: Component>(
	mut commands: Commands,
	despawn_query: Query<Entity, (Without<T>, Without<ScreenFade>, Without<Camera>, Without<Window>)>,
) {
	for entity in &despawn_query {
		commands.entity(entity).despawn_recursive();
	}
}

pub fn cart_to_iso(
	coords: Vec2,
) -> Vec2 {
	let abcd = (1.0, 1.0, -0.5, 0.5);
	Vec2::new(coords.x * abcd.0 + coords.y * abcd.1, coords.x * abcd.2 + coords.y * abcd.3)
}

pub fn iso_to_cart(
	coords: Vec2,
) -> Vec2 {
	let abcd = (1.0, 1.0, -0.5, 0.5);
	let det = 1.0/(abcd.0 * abcd.3 - abcd.1 * abcd.2);
	Vec2::new(coords.x * abcd.3 + coords.y * -abcd.1, coords.x * -abcd.2 + coords.y * abcd.0) * det
}