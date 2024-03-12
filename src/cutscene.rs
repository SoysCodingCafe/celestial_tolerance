// Cutscene module, for intro animatic and dialogue
use bevy::prelude::*;

use crate::{helper::GameState, menu::ScreenFade, setup::{ProgressTracker, TargetGameState, ORTHO, REVERT}};

#[derive(Resource)]
pub struct AllDialogue{
	pub tutorial_intro: [(String, ActorInfo, BackgroundInfo); 14],
	pub tutorial_outro: [(String, ActorInfo, BackgroundInfo); 10],
	pub campaign_intro: [(String, ActorInfo, BackgroundInfo); 7],
	pub campaign_day_0: [(String, ActorInfo, BackgroundInfo); 9],
	pub campaign_day_1: [(String, ActorInfo, BackgroundInfo); 9],
	pub campaign_day_2: [(String, ActorInfo, BackgroundInfo); 5],
	pub campaign_day_3: [(String, ActorInfo, BackgroundInfo); 9],
	pub endless_outro: [(String, ActorInfo, BackgroundInfo); 3],
	pub failure: [(String, ActorInfo, BackgroundInfo); 3],
}

pub fn lines_per_scene(
	current_scene: SceneName,
) -> usize {
	match current_scene {
		SceneName::TutorialIntro => 14-1,
		SceneName::TutorialOutro => 10-1,
		SceneName::CampaignIntro => 7-1,
		SceneName::CampaignDay(i) => {
			match i {
				0 => 9-1,
				1 => 9-1,
				2 => 5-1,
				_ => 9-1,
			}
		},
		SceneName::EndlessOutro => 3-1,
		SceneName::Failure => 3-1,

	}
}

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(AllDialogue{
				tutorial_intro:	[
					("[Press Space to advance, hold Space to skip.]".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Caelum}),
					("Lightning splits the sky as two entities clash, the ringing of bells and baying of goats echoing around like thunder.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Caelum}),
					("On one side, a horned beast roars, their horizontal pupils fixed on their opponent.".to_string(), ActorInfo{actor: Actor::Aberrant}, BackgroundInfo{background: Background::Caelum}),
					("On the other, an amorphous mass wields a glowing spear gripped with too many fingers.".to_string(), ActorInfo{actor: Actor::Celeste}, BackgroundInfo{background: Background::Caelum}),
					("In a moment, it is over.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Betrayal}),
					("As the spear pierced the beast's heart it fragmented, and their chromatic energy spilled forth to corrupt the denizens of the land into aberrations.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Betrayal}),
					("An interesting story, but that's not how it happened. Perhaps I will tell you the tale once you prove yourself.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("For now I have brought you a gift, from the smith. These bells will help you guide the goats down the mountain, and will ward off any aberrations that impede you.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Thank you, and thank her for me. The goats have been restless, and I have seen dark shadows flitting through the clouds.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Nothing you can't handle I'm sure, especially with the smith worrying about you.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Things will start to heat up once the ritual begins next cycle. Better get your practice in while things are relatively calm.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("You only need to bring back ten goats, we can round up the rest later.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("I'll see what I can do, and I'll be back soon for that story. See you next cycle.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Stay safe out there, goatherd. Remember, ten goats.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),				
				],
				tutorial_outro: [
					("You made it back, I'm impressed. It's a good start, but you still have much work to do. Did you seen many aberrations out there?".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("There were winged beasts that scoured the hillside for isolated goats. The bells helped a lot, though I feel it will take time to find the best use of each.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("I'm sure you'll work it out. Now, I believe I promised you a story.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("A story about the great horned Calcyon, spontaneously created in the chaotic maelstrom of Caelum. They were immensely powerful, yet desperately lonely. ".to_string(), ActorInfo{actor: Actor::Aberrant}, BackgroundInfo{background: Background::Caelum}),
					("In their isolation they created Dynamei, a being filled with creativity and curiosity. Together they created Ala, our world, and filled it with a variety of life.".to_string(), ActorInfo{actor: Actor::Celeste}, BackgroundInfo{background: Background::Caelum}),
					("Calcyon was content, but Dynamei's curiosity could never be sated, and they sought to create ever wilder creatures.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Caelum}),
					("From this arose their disagreement, and Calcyon was struck down, leaving Dynamei to plague us with increasingly powerful aberrations.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Betrayal}),
					("There will be more of them, and more powerful? I'm not sure how long I can keep this up...".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("All hope is not lost. We still have the ritual. But my throat grows tired, and you still have a way to go before I can trust you with those secrets.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("For now, we rest. I will meet you back here next cycle, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				campaign_intro: [
					("Greetings, goatherd. The initiation of the ritual is upon us. Are you ready?".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("I am not sure what to be ready for, but I will try my best.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("That's the spirit. I will tell you more once you return, but for now we are on a tight schedule.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("The smith urged me to remind you to make good use of the bells. I'm sure you will have no problem working out their unique abilities.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Tell her she worries too much, I will be fine. How many goats are required to start this ritual?".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Fifteen to begin with, and we will need much more in the following cycles. Things will get harder as Dynamei notices our efforts here.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				campaign_day_0: [
					("I have brought what you require, but it was not easy. The clouds grow dark with wings, and I felt movement under the ground, as though roots tunnel beneath me.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("If I am to continue to be a part of this you must tell me more about this ritual you intend to conduct.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Of course, goatherd, you have proven your loyalty. But this talk of roots worries me. Make sure you stomp out any weeds before they grow into a problem.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("The ritual will provide us a way to fight back against these aberrations, but for it to work we need many lives.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("When Calcyon was struck down, a fragment of the spear that Dynamei had created broke off inside them. Greviously wounded, Calcyon fled to the world that they had created.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Betrayal}),
					("It is from that fragment that your bells are forged, and the energy within seems to resonate with both the goats and the aberrations.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Betrayal}),
					("If you have the fragment then... what happened to Calcyon?".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("A story for another time. Now that the ritual has begun we will have less time for idle talk.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				campaign_day_1: [
					("This ritual better be worth it. I am attacked from the ground and the skies, and I see dark shapes twist beneath the waves.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("And yet you do not falter! Your mastery over the bells grows, and even the smith grows confident in you. She only mentions you every second sentence now.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Very funny. When I wield these bells, is it the strength of Calcyon, or the smith, that I feel flowing through me?".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Perhaps it is both. You wished to know what happened to Calcyon after their fall? Well, the ritual may be a way to bring them back.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("I can't disclose their location, as Dynamei searches the land for them as we speak, but they are safe, and it's with their aid that the smith forged your bells.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Is bringing them back truly the solution? How do we know they are on our side?".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("They were willing to take a spear for us, and without their help our village would have fallen long ago. I trust the smith, and I trust them.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Now you must go forth once more. In two more cycles the ritual will be complete, and I fear that Dynamei will not make it easy for us.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				campaign_day_2: [
					("The hillside shakes, battered from the sea by some unseen creature, and still I am tormented by aberrations from the sky and burrowing beneath the ground.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("And still you persist, as does the village. The aberrations have laid seige to our hideout, but the smith holds them off for now. The ritual must be completed this cycle.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("This will be a tall order, but you have proven yourself by now. Go forth and do what you do best. The smith awaits your return.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("If this is what must be done, then I shall not let any aberrations stop me. When we meet again my bells, our bells, shall ring out in celebration.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				campaign_day_3: [
					("Hurry, there is no time to waste. They have broken down the barricades, and the smith can only hold off the aberrations for so long. Follow me.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Lightning splits the sky as the two figures race down the hillside, the ringing of bells and baying of goats echoing around like thunder.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Hillside}),
					("A mass of writhing tentacles ensnares the cliff, dragging it down into the sea, as the air fills with the buzzing of wings.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Hillside}),
					("Then, as the pair reach the base of the hill, the goats surge forward as one into the sea caves, and a bleating roar echoes outwards.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Hillside}),
					("As the hillside is peeled back by grasping tendrils, an immense figure rises upwards from the debris.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Hillside}),
					("Come, my creations. My friends. You have defended me. I shall show you the same kindness.".to_string(), ActorInfo{actor: Actor::Aberrant}, BackgroundInfo{background: Background::Hillside}),
					("Another roar echoed forth, amplified through the bells, and the aberrations faltered, then began to retreat.".to_string(), ActorInfo{actor: Actor::Nobody}, BackgroundInfo{background: Background::Hillside}),
					("Our work here is complete, goatherd. Now return to the smith so that I no longer have to put up with her constant fussing.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Thank you for your help. Stay safe out there.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
				],
				endless_outro: [
					("These bells have served me well once more, and I have brought the goats to you.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Great work, your skill never ceases to impress. Our humble village grows each cycle!".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("But still there are goats lost up that hill seeking guidance and protection. Stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
				failure: [
					("There were too many aberrations, and the herd was scattered. I had no choice but to flee.".to_string(), ActorInfo{actor: Actor::Goatherd}, BackgroundInfo{background: Background::Hillside}),
					("Do not worry, goatherd. This is only a minor setback, all is not lost. Just remember to make use of all the bells at your disposal, and that your agility allows you to take routes the goats cannot.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
					("Now get out there and lead them back to us. And stay safe out there, goatherd.".to_string(), ActorInfo{actor: Actor::Farmhand}, BackgroundInfo{background: Background::Hillside}),
				],
			})
			.insert_resource(CutsceneTracker{
				current_scene: SceneName::TutorialIntro,
				current_line: 0,
				current_character: 0,
				full_line: "".to_string(),
				actor_info: ActorInfo{actor: Actor::Nobody},
				background_info: BackgroundInfo{background: Background::None},
				cutscene_state: CutsceneState::Initialize,
			})
			.insert_resource(TextSpeedTimer(Timer::from_seconds(0.005, TimerMode::Repeating)))
			.insert_resource(SkipTimer(Timer::from_seconds(1.2, TimerMode::Once)))
			.add_systems(OnEnter(GameState::Cutscene),
				cutscene_setup,
			)
			.add_systems(Update, (
				update_cutscene_text,
				fade_actors,
				fade_backgrounds,
			).run_if(in_state(GameState::Cutscene)))
		;
	}
}

fn cutscene_setup(
	mut commands: Commands,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	asset_server: Res<AssetServer>,
	all_dialogue: Res<AllDialogue>,
) {
	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(-400.0*REVERT, 0.0, 700.0),
		texture: asset_server.load("c_aberrant.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x/2.0, ORTHO.y)),
			..default()
		},
		..default()
		},
		ActorInfo{actor: Actor::Aberrant},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(400.0*REVERT, 0.0, 700.0),
		texture: asset_server.load("c_celeste.png"),
		sprite: Sprite {
			color: Color::rgba(0.9, 0.9, 0.9, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x/2.0, ORTHO.y)),
			..default()
		},
		..default()
		},
		ActorInfo{actor: Actor::Celeste},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(400.0*REVERT, 0.0, 700.0),
		texture: asset_server.load("c_farmhand.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x/2.0, ORTHO.y)),
			..default()
		},
		..default()
		},
		ActorInfo{actor: Actor::Farmhand},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(-400.0*REVERT, 0.0, 700.0),
		texture: asset_server.load("c_goatherd.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x/2.0, ORTHO.y)),
			..default()
		},
		..default()
		},
		ActorInfo{actor: Actor::Goatherd},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0, 600.0),
		texture: asset_server.load("b_caelum.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x, ORTHO.y)),
			..default()
		},
		..default()
		},
		BackgroundInfo{background: Background::Caelum},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0, 600.0),
		texture: asset_server.load("b_betrayal.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x, ORTHO.y)),
			..default()
		},
		..default()
		},
		BackgroundInfo{background: Background::Betrayal},
	));

	commands.spawn((SpriteBundle {
		transform: Transform::from_xyz(0.0, 0.0, 600.0),
		texture: asset_server.load("b_hillside.png"),
		sprite: Sprite {
			color: Color::rgba(1.0, 1.0, 1.0, 0.0),
			custom_size: Some(Vec2::new(ORTHO.x, ORTHO.y)),
			..default()
		},
		..default()
		},
		BackgroundInfo{background: Background::Hillside},
	));

	let (initial_line, initial_actor, initial_background) = next_line(cutscene_tracker.current_scene, 0, all_dialogue);
	cutscene_tracker.actor_info = initial_actor;
	cutscene_tracker.background_info = initial_background;

	let size = Vec2::new(ORTHO.x*3.0/4.0, ORTHO.y*2.0/9.0);
	let margin = 10.0;
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, -300.0*REVERT, 900.0),
			sprite: Sprite {
				color: Color::rgba(1.0, 1.0, 1.0, 0.8),
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
					size.y / 2.0 - margin,
					10.0,
				),
				text_anchor: bevy::sprite::Anchor::TopLeft,
				text: Text::from_section(initial_line.clone(), get_cutscene_text_style(&asset_server))
				.with_justify(JustifyText::Left),
				..default()
			},
			CutsceneText(true),
		));
		for i in 0..8 {
			let offset = match i {
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
						-size.x / 2.0 + margin + offset.0 * 2.0,
						size.y / 2.0 - margin + offset.1 * 2.0,
						9.0,
					),
					text_anchor: bevy::sprite::Anchor::TopLeft,
					text: Text::from_section(initial_line.clone(), get_shadow_text_style(&asset_server))
					.with_justify(JustifyText::Left),
					..default()
				},
				CutsceneText(false),
			));
		}
	});
}

#[derive(Eq, PartialEq, Default)]
pub enum CutsceneState {
	#[default]
	Initialize,
	Started,
	Ended,
}

#[derive(Copy, Clone)]
pub enum SceneName {
	TutorialIntro,
	TutorialOutro,
	CampaignIntro,
	CampaignDay(usize),
	EndlessOutro,
	Failure,
}

#[derive(Resource)]
pub struct CutsceneTracker {
	pub current_scene: SceneName,
	current_line: usize,
	current_character: usize,
	full_line: String,
	actor_info: ActorInfo,
	background_info: BackgroundInfo,
	pub cutscene_state: CutsceneState,
}

#[derive(Resource)]
struct TextSpeedTimer(Timer);

#[derive(Resource)]
struct SkipTimer(Timer);

#[derive(Component)]
struct CutsceneText(bool);

fn update_cutscene_text(
	mut commands: Commands,
	mut cutscene_text_query: Query<(&mut Text, &CutsceneText)>,
	mut cutscene_tracker: ResMut<CutsceneTracker>,
	mut text_speed_timer: ResMut<TextSpeedTimer>,
	asset_server: Res<AssetServer>,
	keyboard: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	mut target_state: ResMut<TargetGameState>,
	mut next_state: ResMut<NextState<GameState>>,
	mut skip_timer: ResMut<SkipTimer>,
	all_dialogue: Res<AllDialogue>,
	mut progress_tracker: ResMut<ProgressTracker>,
) {
	let mut skip = false;
	if keyboard.pressed(KeyCode::Space){
		skip_timer.0.tick(time.delta());
		if skip_timer.0.just_finished() {
			skip = true;
			cutscene_tracker.cutscene_state = CutsceneState::Started;
			cutscene_tracker.current_character = cutscene_tracker.full_line.len();
			cutscene_tracker.current_line = lines_per_scene(cutscene_tracker.current_scene);
		}
	}
	if keyboard.just_released(KeyCode::Space){
		skip_timer.0.reset();
	}
	if keyboard.just_pressed(KeyCode::Space) || skip {
		match cutscene_tracker.cutscene_state {
			CutsceneState::Initialize => {
				cutscene_tracker.current_character = 0;
				cutscene_tracker.current_line = 1;
				(cutscene_tracker.full_line, cutscene_tracker.actor_info, cutscene_tracker.background_info) = next_line(cutscene_tracker.current_scene, cutscene_tracker.current_line, all_dialogue);
				cutscene_tracker.cutscene_state = CutsceneState::Started;
			},
			CutsceneState::Started => {
				if cutscene_tracker.current_character != cutscene_tracker.full_line.len() {
					cutscene_tracker.current_character = cutscene_tracker.full_line.len() - 1;
				} else {
					if cutscene_tracker.current_line != lines_per_scene(cutscene_tracker.current_scene) {
						cutscene_tracker.current_character = 0;
						cutscene_tracker.current_line += 1;
						(cutscene_tracker.full_line, cutscene_tracker.actor_info, cutscene_tracker.background_info) = next_line(cutscene_tracker.current_scene, cutscene_tracker.current_line, all_dialogue);
					} else {
						match cutscene_tracker.current_scene {
							SceneName::TutorialIntro | SceneName::CampaignIntro | 
							SceneName::EndlessOutro | SceneName::Failure => {
								target_state.state = GameState::Game;
							}
							SceneName::TutorialOutro  => {
								target_state.state = GameState::Menu;
							}
							SceneName::CampaignDay(i) => {
								if i < 3 {
									target_state.state = GameState::Game;
								} else {
									progress_tracker.max_campaign = 0;
									target_state.state = GameState::Menu;
								}
							}
						}
						cutscene_tracker.current_line = 0;
						cutscene_tracker.current_character = 0;
						cutscene_tracker.actor_info = ActorInfo{actor: Actor::Nobody};
						cutscene_tracker.cutscene_state = CutsceneState::Ended;
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
			},
			CutsceneState::Ended => {

			}
		}
	}
	text_speed_timer.0.tick(time.delta());
	if cutscene_tracker.cutscene_state == CutsceneState::Started {
		if text_speed_timer.0.just_finished() {
			if cutscene_tracker.current_character + 1 <= cutscene_tracker.full_line.len() {
				cutscene_tracker.current_character += 1;
				for (mut text, cutscene_text) in cutscene_text_query.iter_mut() {
					text.sections = vec![
						TextSection::new(
							&cutscene_tracker.full_line[0..cutscene_tracker.current_character],
							if cutscene_text.0 {get_cutscene_text_style(&asset_server)}
							else {get_shadow_text_style(&asset_server)},
						)
					];
				}
			}
		}
	}
}

fn fade_actors(
	mut actor_query: Query<(&mut Sprite, &ActorInfo)>,
	cutscene_tracker: Res<CutsceneTracker>,
	time: Res<Time>,
) {
	for (mut sprite, info) in actor_query.iter_mut() {
		let current_a = sprite.color.a();
		if info.actor == cutscene_tracker.actor_info.actor {
			sprite.color.set_a((current_a + 6.0 * time.delta_seconds()).clamp(0.0, 1.0));
		} else {
			sprite.color.set_a((current_a - 6.0 * time.delta_seconds()).clamp(0.0, 1.0));
		}
	}
}

fn fade_backgrounds(
	mut background_query: Query<(&mut Sprite, &BackgroundInfo)>,
	cutscene_tracker: Res<CutsceneTracker>,
	time: Res<Time>,
) {
	for (mut sprite, info) in background_query.iter_mut() {
		let current_a = sprite.color.a();
		if info.background == cutscene_tracker.background_info.background {
			sprite.color.set_a((current_a + 6.0 * time.delta_seconds()).clamp(0.0, 1.0));
		} else {
			sprite.color.set_a((current_a - 6.0 * time.delta_seconds()).clamp(0.0, 1.0));
		}
	}
}

pub fn get_cutscene_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/rony-siswadi-architect-1-font/smooth.ttf"),
		font_size: 44.0,
		color: Color::rgba(0.9, 0.9, 0.9, 1.0),
		..default()
	}
}

pub fn get_shadow_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/rony-siswadi-architect-1-font/smooth.ttf"),
		font_size: 44.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
		..default()
	}
}

pub fn next_line(
	current_scene: SceneName,
	current_line: usize,
	all_dialogue: Res<AllDialogue>,
) -> (String, ActorInfo, BackgroundInfo) {
	match current_scene {
		SceneName::TutorialIntro => all_dialogue.tutorial_intro[current_line].clone(),
		SceneName::TutorialOutro => all_dialogue.tutorial_outro[current_line].clone(),
		SceneName::CampaignIntro => all_dialogue.campaign_intro[current_line].clone(),
		SceneName::CampaignDay(i) => {
			match i {
				0 => all_dialogue.campaign_day_0[current_line].clone(),
				1 => all_dialogue.campaign_day_1[current_line].clone(),
				2 => all_dialogue.campaign_day_2[current_line].clone(),
				_ => all_dialogue.campaign_day_3[current_line].clone(),
			}
		},
		SceneName::EndlessOutro => all_dialogue.endless_outro[current_line].clone(),
		SceneName::Failure => all_dialogue.failure[current_line].clone(),	
	}
}

#[derive(Component, Copy, Clone, Default)]
pub struct ActorInfo {
	pub actor: Actor,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub enum Actor {
	#[default]
	Nobody,
	Aberrant,
	Celeste,
	Goatherd,
	Farmhand,
}

#[derive(Component, Copy, Clone, Default)]
pub struct BackgroundInfo {
	pub background: Background,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub enum Background {
	#[default]
	None,
	Caelum,
	Betrayal,
	Hillside,
}