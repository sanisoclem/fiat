use bevy::{app::AppExit, prelude::*};
use game_utils::cleanup_system;
use std::{fmt::Debug, hash::Hash};

pub struct MainMenuPlugin<T> {
  config: MainMenuConfig<T>,
}

#[derive(Clone)]
struct MainMenuConfig<T> {
  menu_state: T,
  game_state: T,
}

impl<T> Plugin for MainMenuPlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(self.config.clone())
      .add_system_set(SystemSet::on_enter(self.config.menu_state).with_system(main_menu_setup))
      .add_system_set(
        SystemSet::on_exit(self.config.menu_state).with_system(cleanup_system::<OnMainMenuScreen>),
      )
      .add_system_set(
        SystemSet::on_update(self.config.menu_state)
          .with_system(Self::menu_action)
          .with_system(button_system),
      );
  }
}

impl<T> MainMenuPlugin<T>
where
  T: Copy + Send + Sync + Eq + Debug + Hash + 'static,
{
  pub fn create(menu_state: T, game_state: T) -> Self {
    Self {
      config: MainMenuConfig {
        menu_state,
        game_state,
      },
    }
  }

  fn menu_action(
    config: Res<MainMenuConfig<T>>,
    interaction_query: Query<
      (&Interaction, &MenuButtonAction),
      (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<T>>,
  ) {
    for (interaction, menu_button_action) in interaction_query.iter() {
      if *interaction == Interaction::Clicked {
        match menu_button_action {
          MenuButtonAction::Quit => app_exit_events.send(AppExit),
          MenuButtonAction::Play => {
            game_state.set(config.game_state).unwrap();
          }
        }
      }
    }
  }
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
  Play,
  Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut UiColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color, selected) in interaction_query.iter_mut() {
    *color = match (*interaction, selected) {
      (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
      (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
      (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
      (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
      (Interaction::None, None) => NORMAL_BUTTON.into(),
    }
  }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
      .spawn_bundle(Camera2dBundle::default())
      .insert(OnMainMenuScreen);

  let font = asset_server.load("ui/Shizuru-Regular.ttf");
  // Common style for all buttons on the screen
  let button_style = Style {
    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
    margin: UiRect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Default::default()
  };
  let button_icon_style = Style {
    size: Size::new(Val::Px(30.0), Val::Auto),
    position_type: PositionType::Absolute,
    position: UiRect {
      left: Val::Px(10.0),
      right: Val::Auto,
      top: Val::Auto,
      bottom: Val::Auto,
    },
    ..Default::default()
  };
  let button_text_style = TextStyle {
    font: font.clone(),
    font_size: 40.0,
    color: TEXT_COLOR,
  };

  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        margin: UiRect::all(Val::Auto),
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      color: Color::CRIMSON.into(),
      ..Default::default()
    })
    .insert(OnMainMenuScreen)
    .with_children(|parent| {
      // Display the game name
      parent.spawn_bundle(TextBundle {
        style: Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..Default::default()
        },
        text: Text::from_section(
          "Combination Game",
          TextStyle {
            font: font.clone(),
            font_size: 80.0,
            color: TEXT_COLOR,
          },
        ),
        ..Default::default()
      });

      parent
        .spawn_bundle(ButtonBundle {
          style: button_style.clone(),
          color: NORMAL_BUTTON.into(),
          ..Default::default()
        })
        .insert(MenuButtonAction::Play)
        .with_children(|parent| {
          let icon = asset_server.load("ui/right.png");
          parent.spawn_bundle(ImageBundle {
            style: button_icon_style.clone(),
            image: UiImage(icon),
            ..Default::default()
          });
          parent.spawn_bundle(TextBundle {
            text: Text::from_section("New Game", button_text_style.clone()),
            ..Default::default()
          });
        });
      parent
        .spawn_bundle(ButtonBundle {
          style: button_style,
          color: NORMAL_BUTTON.into(),
          ..Default::default()
        })
        .insert(MenuButtonAction::Quit)
        .with_children(|parent| {
          let icon = asset_server.load("ui/exitRight.png");
          parent.spawn_bundle(ImageBundle {
            style: button_icon_style,
            image: UiImage(icon),
            ..Default::default()
          });
          parent.spawn_bundle(TextBundle {
            text: Text::from_section("Quit", button_text_style),
            ..Default::default()
          });
        });
    });
}
