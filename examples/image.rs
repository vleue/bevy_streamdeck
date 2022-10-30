use bevy::{
    asset::AssetPlugin, log::LogPlugin, prelude::*, render::texture::ImagePlugin,
    time::FixedTimestep,
};
use bevy_streamdeck::{StreamDeck, StreamDeckPlugin};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(ImagePlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(StreamDeckPlugin)
        .add_startup_system(load_asset)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(change_image),
        )
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
