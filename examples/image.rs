use bevy::{asset::AssetPlugin, log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeck, StreamDeckPlugin};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ImagePlugin::default(),
            LogPlugin::default(),
        ))
        .add_plugins(StreamDeckPlugin)
        .insert_resource(Time::<Fixed>::from_seconds(1.0))
        .add_systems(Startup, load_asset)
        .add_systems(FixedUpdate, change_image)
        .run();
}

#[derive(Resource)]
struct Logo(Handle<Image>);

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Logo(asset_server.load("birdoggo.png")));
}

fn change_image(streamdeck: Res<StreamDeck>, logo: Res<Logo>, images: Res<Assets<Image>>) {
    if let Some(image) = images.get(&logo.0) {
        let mut rng = rand::thread_rng();

        if let Some(kind) = streamdeck.kind() {
            let key = rng.gen_range(0..kind.keys());

            streamdeck.set_key_image(key, &image);
            for i in 0..kind.keys() {
                if i != key {
                    streamdeck.reset_key(i);
                }
            }
        }
    }
}
