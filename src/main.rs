use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
  Splash,
  Menu,
  Game,
}

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
    // .add_plugin(systems::AudioPlugin)
    // .add_plugin(systems::AnimationPlugin)
    // .add_plugin(systems::CombatPlugin)
    // //.add_plugin(systems::DebugPlugin)
    // //.add_plugin(InspectorPlugin::<Data>::new())
    // //.add_plugin(ShapePlugin)
    // .add_plugin(systems::PhysicsPlugin)
    // .add_plugin(systems::MousePlugin)
    // .add_plugin(systems::MovementPlugin)
    // .add_plugin(splash::SplashPlugin)
    // .add_plugin(menu::MenuPlugin)
    // .add_plugin(game::GamePlugin)
    // .add_plugin(EditorPlugin)
    .add_startup_system(setup)
    .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {}
