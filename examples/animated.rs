use bevy::{
    asset::AssetPlugin, core::FixedTimestep, log::LogPlugin, prelude::*,
    render::texture::ImagePlugin,
};
use bevy_streamdeck::{StreamDeck, StreamDeckPlugin};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(ImagePlugin)
        .add_plugin(LogPlugin)
        .add_plugin(StreamDeckPlugin)
        .add_startup_system(load_asset)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.075))
                .with_system(change_image),
        )
        .run();
}

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
