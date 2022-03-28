use bevy::{core::FixedTimestep, log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeck, StreamDeckPlugin};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(StreamDeckPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(change_color),
        )
        .run();
}

fn change_color(streamdeck: Res<StreamDeck>) {
    let mut rng = rand::thread_rng();

    if let Some(kind) = streamdeck.kind {
        let key = rng.gen_range(0..kind.keys());
        let color = Color::rgb(rng.gen(), rng.gen(), rng.gen());

        streamdeck.set_key_color(key, color);
        for i in 0..kind.keys() {
            if i != key {
                streamdeck.reset_key(i);
            }
        }
    }
}
