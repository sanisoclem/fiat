use std::collections::HashMap;

use bevy::{prelude::*, audio::AudioSink};

#[derive(Clone, Debug)]
pub enum AudioCommand {
  Play(String),
  PlayInLayer(String, String),
  StopLayer(String)
}

pub struct SoundController {
  layers: HashMap<String, Handle<AudioSink>>
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(SoundController { layers: HashMap::new() })
      .add_system(play_audio);
  }
}

fn play_audio(
  mut cmds: EventReader<AudioCommand>,
  mut controller: ResMut<SoundController>,
  asset_server: Res<AssetServer>,
  audio: Res<Audio>,
  audio_sinks: Res<Assets<AudioSink>>,
) {
  for cmd in cmds.iter() {
    match cmd {
      AudioCommand::Play(path) => {
        let src = asset_server.load(path);
        audio.play(src);
      },
      AudioCommand::PlayInLayer(path, layer) => {
        let src = asset_server.load(path);
        let handle = audio_sinks.get_handle(audio.play(src));
        if let Some(prev) = controller.layers.insert(layer.clone(), handle) {
          if let Some(sink) = audio_sinks.get(&prev) {
            sink.stop();
          }
        }

      },
      AudioCommand::StopLayer(layer) => {
        if let Some(prev) = controller.layers.get(layer) {
          if let Some(sink) = audio_sinks.get(&prev) {
            sink.stop();
          }
        }
      }
    }
  }
}