use bevy::{log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeckInput, StreamDeckKey, StreamDeckPlugin};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(StreamDeckPlugin)
        .add_system(print_streamdeck_events)
        .add_system(check_streamdeck_key_status)
        .run();
}

fn print_streamdeck_events(mut streamdeck_input_events: EventReader<StreamDeckInput>) {
    for event in streamdeck_input_events.iter() {
        info!("{:?}", event);
    }
}

fn check_streamdeck_key_status(streamdeck_key: Res<Input<StreamDeckKey>>) {
    for i in 0..50 {
        if streamdeck_key.just_pressed(StreamDeckKey(i)) {
            info!("key {} just pressed", i);
        }
        if streamdeck_key.pressed(StreamDeckKey(i)) {
            info!("key {} currently pressed", i);
        }
    }
}
