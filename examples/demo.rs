use bevy::{
    app::AppExit, asset::AssetPlugin, log::LogPlugin, prelude::*, render::texture::ImagePlugin,
    time::FixedTimestep,
};
use bevy_streamdeck::{ImageMode, StreamDeck, StreamDeckPlugin};
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
                .with_run_criteria(FixedTimestep::step(0.075))
                .with_system(animated),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(change_color),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(invert_image)
                .with_system(background_image),
        )
        .add_system(setup)
        .run();
}

fn change_color(
    streamdeck: Res<StreamDeck>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(_) = streamdeck.kind() {
        let color = Color::hsl(
            (((time.elapsed_seconds() / 5.0).cos() + 1.0) / 2.0) as f32 * 360.0,
            1.0,
            0.5,
        );
        streamdeck.set_key_color(1, color);
        if (time.elapsed_seconds() / 5.0).cos() + 0.9995 < 0.0 {
            app_exit_events.send(AppExit);
        }
    }
}

#[derive(Resource)]
struct Animated([Handle<Image>; 11], usize);

#[derive(Resource)]
struct Logos(Handle<Image>, Handle<Image>, Handle<Image>);

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
    commands.insert_resource(Logos(
        asset_server.load("bevy.png"),
        asset_server.load("vleue.png"),
        asset_server.load("birdoggo.png"),
    ))
}

fn animated(
    streamdeck: Res<StreamDeck>,
    mut animated: ResMut<Animated>,
    images: Res<Assets<Image>>,
) {
    if let Some(image) = images.get(&animated.0[animated.1]) {
        streamdeck.set_key_image(0, &image);
        animated.1 = (animated.1 + 1) % animated.0.len();
    }
}

fn invert_image(
    streamdeck: Res<StreamDeck>,
    logos: Res<Logos>,
    images: Res<Assets<Image>>,
    mut inverted: Local<bool>,
) {
    if let Some(image) = images.get(&logos.1) {
        if let Some(_) = streamdeck.kind() {
            streamdeck.set_key_image_with_mode(
                2,
                &image,
                ImageMode {
                    invert: *inverted,
                    ..Default::default()
                },
            );
            *inverted = !*inverted;
        }
    }
}

fn background_image(streamdeck: Res<StreamDeck>, logos: Res<Logos>, images: Res<Assets<Image>>) {
    if let Some(image) = images.get(&logos.2) {
        if let Some(_) = streamdeck.kind() {
            let mut rng = rand::thread_rng();
            let color = Color::rgb(rng.gen(), rng.gen(), rng.gen());

            streamdeck.set_key_color(4, color);

            streamdeck.set_key_image_with_mode(
                5,
                &image,
                ImageMode {
                    background: Some(color),
                    ..Default::default()
                },
            );
        }
    }
}

fn setup(
    streamdeck: Res<StreamDeck>,
    logos: Res<Logos>,
    images: Res<Assets<Image>>,
    mut done: Local<usize>,
) {
    if *done < 3 {
        *done = 0;
        if let Some(_) = streamdeck.kind() {
            if let Some(image) = images.get(&logos.1) {
                streamdeck.set_key_image(2, &image);
                *done += 1;
            }
            if let Some(image) = images.get(&logos.0) {
                streamdeck.set_key_image(3, &image);
                *done += 1;
            }
            if let Some(image) = images.get(&logos.2) {
                streamdeck.set_key_image(5, &image);
                *done += 1;
            }
        }
    }
}
