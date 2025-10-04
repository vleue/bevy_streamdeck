use bevy::{log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeckInput, StreamDeckKey, StreamDeckPlugin};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(StreamDeckPlugin)
        .add_systems(
            Update,
            (print_streamdeck_events, check_streamdeck_key_status),
        )
        .run();
}

fn print_streamdeck_events(mut streamdeck_input: MessageReader<StreamDeckInput>) {
    for event in streamdeck_input.read() {
        info!("{:?}", event);
    }
}

fn check_streamdeck_key_status(streamdeck_key: Res<ButtonInput<StreamDeckKey>>) {
    for i in 0..50 {
        // TODO: check with the number of keys on the deck
        if streamdeck_key.just_pressed(StreamDeckKey(i)) {
            info!("key {} just pressed", i);
        }
        if streamdeck_key.pressed(StreamDeckKey(i)) {
            info!("key {} currently pressed", i);
        }
    }
}
