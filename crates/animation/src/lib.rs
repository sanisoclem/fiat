use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

#[derive(Clone)]
pub struct AnimationDefinition {
  pub start: usize,
  pub end: usize,
  pub fps: f32,
  pub repeat: bool,
  pub repeat_from: Option<usize>,
  pub random_start: bool,
}
impl AnimationDefinition {
  pub fn duration_seconds(&self) -> f32 {
    (self.end - self.start + 1) as f32 / self.fps
  }
}

#[derive(Clone)]
pub struct AnimationSet<T> {
  animations: HashMap<T, AnimationDefinition>,
}

#[derive(Component, Clone)]
pub struct RequestedAnimation<T> {
  play: T,
}

#[derive(Component)]
pub struct PlayingAnimation {
  pub timer: Timer,
  pub start_frame: usize,
  pub complete: bool,
}

pub struct AnimationPlugin<T> {
  phantom: PhantomData<T>,
}

impl<T> Plugin for AnimationPlugin<T>
where
  T: Send + Sync + Eq + Hash + 'static,
{
  fn build(&self, app: &mut App) {
    app
      .add_system(Self::init_atlas_animation)
      .add_system(Self::animate_sprites);
  }
}

impl<T> AnimationPlugin<T>
where
  T: Send + Sync + Eq + Hash + 'static,
{
  fn init_atlas_animation(
    mut commands: Commands,
    mut qry: Query<
      (
        Entity,
        Option<&mut PlayingAnimation>,
        &RequestedAnimation<T>,
        &mut TextureAtlasSprite,
      ),
      Or<(Changed<RequestedAnimation<T>>, Added<RequestedAnimation<T>>)>,
    >,
    defs: Res<AnimationSet<T>>,
  ) {
    if !qry.is_empty() {
      let mut rng = rand::thread_rng();

      for (entity, maybe_anim, req, mut sprite) in qry.iter_mut() {
        if let Some(def) = defs.animations.get(&req.play) {
          let timer = Timer::from_seconds(1. / def.fps, true);
          let start_frame = if def.random_start {
            let between = Uniform::from(def.start..(def.end + 1));
            between.sample(&mut rng)
          } else {
            def.start
          };

          if let Some(mut anim) = maybe_anim {
            anim.timer = timer;
            anim.start_frame = start_frame;
            anim.complete = false;
          } else {
            commands.entity(entity).insert(PlayingAnimation {
              timer,
              start_frame,
              complete: false,
            });
          }

          sprite.index = start_frame;
        }
      }
    }
  }

  fn animate_sprites(
    time: Res<Time>,
    defs: Res<AnimationSet<T>>,
    mut query: Query<(
      &RequestedAnimation<T>,
      &mut PlayingAnimation,
      &mut TextureAtlasSprite,
    )>,
  ) {
    for (req, mut animation, mut sprite) in query.iter_mut() {
      if let Some(def) = defs.animations.get(&req.play) {
        if !animation.complete {
          animation.timer.tick(time.delta());
          if animation.timer.just_finished() {
            let new_index = sprite.index + 1;
            if new_index > def.end {
              if let Some(repeat_from) = def.repeat_from {
                sprite.index = repeat_from;
              } else {
                sprite.index = def.start;
              }
            } else {
              sprite.index = new_index;
            }

            if !def.repeat
              && ((!def.random_start && sprite.index == def.end)
                || sprite.index == animation.start_frame)
            {
              animation.complete = true;
            }
          }
        }
      }
    }
  }
}
