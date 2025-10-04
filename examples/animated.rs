use bevy::{asset::AssetPlugin, log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeck, StreamDeckPlugin};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ImagePlugin::default(),
            LogPlugin::default(),
        ))
        .add_plugins(StreamDeckPlugin)
        .insert_resource(Time::<Fixed>::from_seconds(0.075))
        .add_systems(Startup, load_asset)
        .add_systems(FixedUpdate, change_image)
        .run();
}

#[derive(Resource)]
struct Animated([Handle<Image>; 11], usize);

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Animated(
        [
            asset_server.load("p1_walk01.png"),
            asset_server.load("p1_walk02.png"),
            asset_server.load("p1_walk03.png"),
            asset_server.load("p1_walk04.png"),
            asset_server.load("p1_walk05.png"),
            asset_server.load("p1_walk06.png"),
            asset_server.load("p1_walk07.png"),
            asset_server.load("p1_walk08.png"),
            asset_server.load("p1_walk09.png"),
            asset_server.load("p1_walk10.png"),
            asset_server.load("p1_walk11.png"),
        ],
        0,
    ));
}

fn change_image(
    streamdeck: Res<StreamDeck>,
    mut animated: ResMut<Animated>,
    images: Res<Assets<Image>>,
) {
    if let Some(image) = images.get(&animated.0[animated.1]) {
        streamdeck.set_key_image(0, &image);
        animated.1 = (animated.1 + 1) % animated.0.len();
    }
}
