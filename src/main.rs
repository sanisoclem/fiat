use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
  Splash,
  Menu,
  Game,
}

mod game;
mod main_menu;
mod splash;

fn main() {
  App::new()
    .insert_resource(WindowDescriptor {
      title: "Let it be done!".to_string(),
      width: 1920.,
      height: 1080.,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(
      0.1568627450980392,
      0.1568627450980392,
      0.1568627450980392,
    )))
    .add_state(GameState::Splash)
    .add_plugins(DefaultPlugins)
    .add_plugin(splash::SplashPlugin::<GameState>::create(
      GameState::Splash,
      GameState::Menu,
      4.0,
    ))
    .add_plugin(game_audio::AudioPlugin)
    .add_plugin(main_menu::MainMenuPlugin::<GameState>::create(
      GameState::Menu,
      GameState::Game,
    ))
    .add_plugin(game::GamePlugin::<GameState>::create(GameState::Game))
    .run();
}
