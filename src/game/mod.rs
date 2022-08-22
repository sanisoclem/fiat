use bevy::prelude::*;
use game_level_gen::{FiniteLevel, GenFiniteLevel};
use game_utils::cleanup_system;
use std::{fmt::Debug, hash::Hash};

// Tag component used to tag top level entities
#[derive(Component)]
struct OnGameScreen;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum PlayState {
  Disabled,
  Loading,
  Playing,
  //Paused
}

pub struct GamePlugin<T> {
  config: GameConfig<T>,
}

#[derive(Clone)]
struct GameConfig<T> {
  game_state: T,
}

impl<T> Plugin for GamePlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(self.config.clone())
      .add_state(PlayState::Disabled)
      .add_system_set(SystemSet::on_enter(self.config.game_state).with_system(start_loading))
      .add_system_set(SystemSet::on_enter(PlayState::Loading).with_system(game_setup))
      .add_system_set(SystemSet::on_update(PlayState::Loading).with_system(check_loaded))
      .add_system_set(
        SystemSet::on_exit(self.config.game_state)
          .with_system(cleanup_system::<OnGameScreen>)
          .with_system(game_exit),
      )
      .add_system_set(SystemSet::on_update(self.config.game_state).with_system(game_update))
      .add_plugin(game_level_gen::LevelGeneratorPlugin);
  }
}

impl<T> GamePlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  pub fn create(game_state: T) -> Self {
    Self {
      config: GameConfig { game_state },
    }
  }
}

fn start_loading(mut state: ResMut<State<PlayState>>) {
  state
    .set(PlayState::Loading)
    .expect("this should always succeed");
}

fn game_setup(mut commands: Commands) {
  commands
    .spawn_bundle(Camera2dBundle::default())
    .insert(OnGameScreen);

  commands
    .spawn()
    .insert(game_level_gen::GenFiniteLevel { max_width: 100, min_height: 2, ..default() });
}

fn check_loaded(
  pending_level_qry: Query<Entity, (With<GenFiniteLevel>, Without<FiniteLevel>)>,
  mut state: ResMut<State<PlayState>>,
) {
  if pending_level_qry.is_empty() {
    state
      .set(PlayState::Playing)
      .expect("set state should always succed");
  }
}
fn game_update() {}
fn game_exit(
  mut state: ResMut<State<PlayState>>,) {
    state
    .set(PlayState::Disabled)
    .expect("set state should always succed");
  }
