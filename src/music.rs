use std::iter::Cycle;
use std::vec::IntoIter;

use amethyst::assets::Loader;
use amethyst::audio::{AudioSink, OggFormat, SourceHandle};
use amethyst::ecs::{World, WorldExt};

use crate::resource;

const MUSIC: &[&str] = &["audio/song.ogg"];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub fn initialize_music(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.5);

        let music = MUSIC
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        Music { music }
    };

    world.insert(music);
}

// from `amethyst/examples/pong/audio.rs#18`
// Loads an ogg audio track.
fn load_audio_track(
    loader: &Loader,
    world: &World,
    file: &str,
) -> SourceHandle {
    loader.load(resource(file), OggFormat, (), &world.read_resource())
}
