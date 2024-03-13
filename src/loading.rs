// Loading module, for handling transistions and asset loading
use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::AudioSource;

use crate::helper::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(OnEnter(GameState::PreConfig), 
				add_assets_to_load,
			)
			.add_systems(Update, 
			check_all_assets_loaded.run_if(in_state(GameState::PreConfig)),
			)
		;
	}
}

#[derive(Resource)]
struct AssetsLoading(Vec<UntypedHandle>);

fn add_assets_to_load(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let mut assets_to_load = Vec::new();
	assets_to_load.push(asset_server.load::<Font>("fonts/rony-siswadi-architect-1-font/smooth.ttf").untyped());

	assets_to_load.push(asset_server.load::<AudioSource>("bgm/title.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("bgm/title_pure.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("bgm/title_corrupt.ogg").untyped());

	assets_to_load.push(asset_server.load::<AudioSource>("sfx/goat_meh_far.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("sfx/goat_short_far.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("sfx/goat_short.ogg").untyped());

	assets_to_load.push(asset_server.load::<AudioSource>("sfx/bell.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("sfx/bell_long.ogg").untyped());
	assets_to_load.push(asset_server.load::<AudioSource>("sfx/bell_quick.ogg").untyped());

	assets_to_load.push(asset_server.load::<Image>("config.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("title.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("goathead.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("c_aberrant.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("c_celeste.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("c_farmhand.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("c_goatherd.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("b_caelum.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("b_betrayal.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("b_hillside.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("goatherd.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("shadow.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("goat.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("goatbird.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("spitter.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("spit.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("arm.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("bell_sel.png").untyped());

	assets_to_load.push(asset_server.load::<Image>("fence.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("iso_map.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("iso_map_top.png").untyped());
	assets_to_load.push(asset_server.load::<Image>("iso_map_tutorial.png").untyped());

	commands.insert_resource(AssetsLoading(assets_to_load));
}

fn check_all_assets_loaded(
	asset_server: Res<AssetServer>,
	loading: Res<AssetsLoading>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	let total_assets: usize = loading.0.len();
	let mut loaded_assets: usize = 0;
	for asset in loading.0.iter() {
		match asset_server.get_load_state(asset.id()) {
			Some(LoadState::Loaded) => {
				loaded_assets += 1;
			}
			Some(_) | None => (),
		}
	}
	if loaded_assets == total_assets {
		next_state.set(GameState::Config);
	} else {
		println!("Loading...");
	}
}