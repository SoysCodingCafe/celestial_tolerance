// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import Bevy game engine essentials
use bevy::{asset::AssetMetaCheck, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};
use bevy_kira_audio::AudioPlugin;

// Include modules for different game aspects
mod helper;

mod cutscene;
mod enemy;
mod goat;
mod land;
mod loading;
mod menu;
mod player;
mod post_proc;
mod setup;

// Only include in debug builds
#[cfg(debug_assertions)]
mod debug;

fn main() {
	let default_plugins = DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Goat Heard".to_string(),
				mode: bevy::window::WindowMode::BorderlessFullscreen,
				canvas: Some("#canvas".into()),
				..default()
			}), 
			..default()
		})
		.set(ImagePlugin::default_nearest())
	;

	let mut app: App = App::new();
	app
		// Prevents bug in wasm builds where it checks for a file
		// that doesn't exist.
		.insert_resource(AssetMetaCheck::Never)
		.add_plugins((
			default_plugins,
			AudioPlugin,
		))
		.add_plugins((
			// Intro animatic and dialogue
			cutscene::CutscenePlugin,
			// Enemy movement and interactions
			enemy::EnemyPlugin,
			// Goat movement and interations
			goat::GoatPlugin,
			// Land placement and interations
			land::LandPlugin,
			// State transitions, loads assets
			loading::LoadingPlugin,
			// Title screen
			menu::MenuPlugin,
			// Player movement and interactions
			player::PlayerPlugin,
			// Post processing effects such as chromatic aberration
			post_proc::PostProcPlugin,
			// Spawns camera, splash screen, title, level
			setup::SetupPlugin,
		))
	;

	{
		#[cfg(debug_assertions)]
		app.add_plugins(debug::DebugPlugin);
	}

	{
		#[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
		// Prevents visual distortion in wasm builds
		app.insert_resource(Msaa::Off)
			.add_plugins(TemporalAntiAliasPlugin);
	}

	app.run();
}