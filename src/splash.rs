use bevy::prelude::*;
use game_utils::cleanup_system;
use std::{fmt::Debug, hash::Hash};

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

struct SplashTimer(Timer);

pub struct SplashPlugin<T> {
  config: SplashScreenConfig<T>,
}

#[derive(Clone)]
struct SplashScreenConfig<T> {
  splash_state: T,
  next_state: T,
  duration: f32,
}

impl<T> Plugin for SplashPlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(self.config.clone())
      .add_system_set(SystemSet::on_enter(self.config.splash_state).with_system(Self::splash_setup))
      .add_system_set(SystemSet::on_update(self.config.splash_state).with_system(Self::countdown))
      .add_system_set(
        SystemSet::on_exit(self.config.splash_state).with_system(cleanup_system::<OnSplashScreen>),
      );
  }
}
impl<T> SplashPlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  pub fn create(splash_state: T, next_state: T, duration: f32) -> Self {
    Self {
      config: SplashScreenConfig {
        splash_state,
        next_state,
        duration,
      },
    }
  }

  fn countdown(
    splash_config: Res<SplashScreenConfig<T>>,
    time: Res<Time>,
    mut game_state: ResMut<State<T>>,
    mut timer: ResMut<SplashTimer>,
  ) {
    if timer.0.tick(time.delta()).finished() {
      game_state.set(splash_config.next_state).unwrap();
    }
  }

  fn splash_setup(
    splash_config: Res<SplashScreenConfig<T>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  ) {
    commands
      .spawn_bundle(Camera2dBundle::default())
      .insert(OnSplashScreen);

    commands
      .spawn_bundle(SpriteBundle {
        texture: asset_server.load("splash.png"),
        ..default()
      })
      .insert(OnSplashScreen);

    commands.insert_resource(SplashTimer(Timer::from_seconds(
      splash_config.duration,
      false,
    )));
  }
}
