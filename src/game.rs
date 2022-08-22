use bevy::prelude::*;
use game_utils::cleanup_system;
use std::{fmt::Debug, hash::Hash};

// Tag component used to tag top level entities
#[derive(Component)]
struct OnGameScreen;

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
      .add_system_set(SystemSet::on_enter(self.config.game_state).with_system(game_setup))
      .add_system_set(
        SystemSet::on_exit(self.config.game_state)
          .with_system(cleanup_system::<OnGameScreen>)
          .with_system(game_exit),
      )
      .add_system_set(SystemSet::on_update(self.config.game_state).with_system(game_update));
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

fn game_setup(mut commands: Commands) {
  commands
    .spawn_bundle(Camera2dBundle::default())
    .insert(OnGameScreen);
}
fn game_update() {}
fn game_exit() {}
