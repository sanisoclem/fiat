use bevy::prelude::*;
use game_level_gen::{FiniteLevel, GenFiniteLevel};
use game_utils::cleanup_system;
use heron::PhysicsPlugin;
use std::{fmt::Debug, hash::Hash};
use bevy_ecs_tilemap::prelude::*;

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
      .add_plugin(game_level_gen::LevelGeneratorPlugin)
      .add_plugin(PhysicsPlugin::default())
      .add_plugin(TilemapPlugin);
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

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn_bundle(Camera2dBundle::default())
    .insert(OnGameScreen);

  // commands
  //   .spawn()
  //   .insert(game_level_gen::GenFiniteLevel { max_width: 100, min_height: 2, ..default() });


    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let tilemap_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..32u32 {
        for y in 0..32u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });
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
