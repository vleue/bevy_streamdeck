use std::time::Duration;

use bevy::{
    app::AppExit, asset::AssetPlugin, log::LogPlugin, prelude::*, render::texture::ImagePlugin,
    time::common_conditions::on_timer,
};
use bevy_streamdeck::{ImageMode, StreamDeck, StreamDeckPlugin};
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
        .add_systems(Startup, load_asset)
        .add_systems(
            Update,
            animated.run_if(on_timer(Duration::from_secs_f32(0.075))),
        )
        .add_systems(
            Update,
            change_color.run_if(on_timer(Duration::from_secs_f32(0.1))),
        )
        .add_systems(
            Update,
            (invert_image, background_image).run_if(on_timer(Duration::from_secs_f32(1.0))),
        )
        .add_systems(Update, setup)
        .run();
}

fn change_color(
    streamdeck: Res<StreamDeck>,
    time: Res<Time>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(_) = streamdeck.kind() {
        let color = Color::hsl(
            (((time.elapsed_secs() / 5.0).cos() + 1.0) / 2.0) as f32 * 360.0,
            1.0,
            0.5,
        );
        streamdeck.set_key_color(1, color);
        if (time.elapsed_secs() / 5.0).cos() + 0.9995 < 0.0 {
            app_exit_events.send(AppExit::Success);
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
            let color = Color::linear_rgb(rng.r#gen(), rng.r#gen(), rng.r#gen());

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
