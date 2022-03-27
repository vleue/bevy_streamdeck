use bevy::{log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeckButton, StreamDeckInput, StreamDeckPlugin};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(StreamDeckPlugin)
        .add_system(print_streamdeck_events)
        .add_system(check_streamdeck_button_status)
        .run();
}

fn print_streamdeck_events(mut streamdeck_input_events: EventReader<StreamDeckInput>) {
    for event in streamdeck_input_events.iter() {
        info!("{:?}", event);
    }
}

fn check_streamdeck_button_status(streamdeck_button: Res<Input<StreamDeckButton>>) {
    for i in 0..50 {
        if streamdeck_button.just_pressed(StreamDeckButton(i)) {
            info!("button {} just pressed", i);
        }
        if streamdeck_button.pressed(StreamDeckButton(i)) {
            info!("button {} currently pressed", i);
        }
    }
}
