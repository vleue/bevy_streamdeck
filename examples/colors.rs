use bevy::{log::LogPlugin, prelude::*};
use bevy_streamdeck::{Color, StreamDeck, StreamDeckPlugin};
use rand::RngExt;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(StreamDeckPlugin)
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(FixedUpdate, change_color)
        .run();
}

fn change_color(streamdeck: Res<StreamDeck>) {
    let mut rng = rand::rng();

    if let Some(kind) = streamdeck.kind() {
        let key = rng.random_range(0..kind.keys());
        let color = Color::linear_rgb(rng.random(), rng.random(), rng.random());

        streamdeck.set_key_color(key, color);
        for i in 0..kind.keys() {
            if i != key {
                streamdeck.reset_key(i);
            }
        }
    }
}
