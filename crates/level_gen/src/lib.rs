use bevy::{
  prelude::*,
  sprite::MaterialMesh2dBundle,
  tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

#[derive(Component, Default)]
pub struct GenFiniteLevel {
  pub max_jump_height: u32, // in blocks
  pub min_height: u32,
  pub max_height: u32,
  pub min_width: u32,
  pub max_width: u32,
}

#[derive(Component)]
pub struct FiniteLevel;

#[derive(Component)]
struct LevelPending(Task<Mesh>);

pub struct LevelGeneratorPlugin;

impl Plugin for LevelGeneratorPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(spawn_build_tasks).add_system(poll_tasks);
  }
}

fn spawn_build_tasks(
  mut commands: Commands,
  qry: Query<(Entity, &GenFiniteLevel), Or<(Changed<GenFiniteLevel>, Added<GenFiniteLevel>)>>,
) {
  for (entity, gen_config) in qry.iter() {
    let thread_pool = AsyncComputeTaskPool::get();
    let w = gen_config.max_width as f32 * 32.;
    let h = gen_config.min_height as f32 * 32.;
    let task = thread_pool.spawn(async move {
      // TODO: generate level

      Mesh::from(shape::Quad {
        size: Vec2::new(w, h),
        flip: false,
      })
    });

    // Spawn new entity and add our new task as a component
    commands.entity(entity).insert(LevelPending(task));
  }
}

fn poll_tasks(
  mut commands: Commands,
  mut transform_tasks: Query<(Entity, &mut LevelPending)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for (entity, mut task) in &mut transform_tasks {
    if let Some(mesh) = future::block_on(future::poll_once(&mut task.0)) {
      commands
        .entity(entity)
        .insert_bundle(MaterialMesh2dBundle {
          mesh: meshes.add(mesh).into(),
          transform: Transform::default(),
          material: materials.add(ColorMaterial::from(Color::PURPLE)),
          ..default()
        })
        .insert(FiniteLevel);
      commands.entity(entity).remove::<LevelPending>();
    }
  }
}
