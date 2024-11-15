use std::iter;

use bevy::{app::AppExit, log::LogPlugin, prelude::*};
use bevy_streamdeck::{Color, StreamDeck, StreamDeckKey, StreamDeckPlugin};
use rand::Rng;

// Lower to make it harder
const FACTOR: f64 = 1.0;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(StreamDeckPlugin)
        .add_systems(Startup, clean)
        .insert_resource(Time::<Fixed>::from_hz(1.0 / FACTOR))
        .add_systems(FixedUpdate, spawn_mole)
        .add_systems(PostUpdate, despawn_mole)
        .add_systems(Update, whack)
        .insert_resource(Player { lives: 3, score: 0 })
        .run();
}

fn clean(streamdeck: Res<StreamDeck>) {
    streamdeck.reset();
}

fn spawn_mole(
    streamdeck: Res<StreamDeck>,
    mut commands: Commands,
    time: Res<Time>,
    moles: Query<&Mole>,
) {
    let mut rng = rand::thread_rng();

    if let Some(kind) = streamdeck.kind() {
        let current = moles.iter().map(|m| m.key).collect::<Vec<_>>();
        let key = iter::repeat(())
            .map(|_| rng.gen_range(0..kind.keys()))
            .find(|k| !current.contains(k))
            .unwrap();

        let max_duration = 180.0;
        if rng.gen_bool(
            (1.0 - (max_duration - time.elapsed_secs_f64()) / max_duration).clamp(0.2, 0.5),
        ) {
            if rng.gen_bool(0.33) {
                streamdeck.set_key_color(key, Color::linear_rgb(1.0, 0.0, 0.0));
                commands.spawn(Mole {
                    key,
                    ty: MoleType::ExtraBad,
                    timer: Timer::from_seconds(
                        rng.gen_range(0.9..1.3) * FACTOR as f32,
                        TimerMode::Once,
                    ),
                });
            } else {
                streamdeck.set_key_color(key, Color::linear_rgb(1.0, 0.25, 0.0));
                commands.spawn(Mole {
                    key,
                    ty: MoleType::Bad,
                    timer: Timer::from_seconds(
                        rng.gen_range(0.9..1.3) * FACTOR as f32,
                        TimerMode::Once,
                    ),
                });
            }
        } else {
            let reduction = (1.0 - (max_duration - time.elapsed_secs_f64()) / max_duration) / 2.0;
            if rng.gen_bool(0.15) {
                streamdeck.set_key_color(key, Color::linear_rgb(0.0, 0.0, 1.0));
                commands.spawn(Mole {
                    key,
                    ty: MoleType::Extra,
                    timer: Timer::from_seconds(
                        (rng.gen_range(0.5..1.0) - reduction as f32).max(0.1) * FACTOR as f32,
                        TimerMode::Once,
                    ),
                });
            } else {
                streamdeck.set_key_color(key, Color::linear_rgb(0.0, 1.0, 0.0));
                commands.spawn(Mole {
                    key,
                    ty: MoleType::Good,
                    timer: Timer::from_seconds(
                        (rng.gen_range(0.7..1.2) - reduction as f32).max(0.1) * FACTOR as f32,
                        TimerMode::Once,
                    ),
                });
            }
        }
    }
}

fn despawn_mole(
    mut commands: Commands,
    mut moles: Query<(Entity, &mut Mole)>,
    time: Res<Time>,
    streamdeck: Res<StreamDeck>,
) {
    for (entity, mut mole) in moles.iter_mut() {
        if mole.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            streamdeck.reset_key(mole.key);
        }
    }
}

fn whack(
    mut commands: Commands,
    moles: Query<(Entity, &Mole)>,
    streamdeck_key: Res<ButtonInput<StreamDeckKey>>,
    streamdeck: Res<StreamDeck>,
    mut player: ResMut<Player>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (entity, mole) in moles.iter() {
        if streamdeck_key.just_pressed(StreamDeckKey(mole.key)) {
            commands.entity(entity).despawn();
            streamdeck.reset_key(mole.key);
            match mole.ty {
                MoleType::ExtraBad => {
                    player.lives = player.lives.saturating_sub(2);
                    info!("Mega ouch!");
                    if player.lives == 0 {
                        info!("You lost!");
                        app_exit_events.send(AppExit::error());
                    }
                }
                MoleType::Bad => {
                    player.lives = player.lives.saturating_sub(1);
                    info!("Ouch!");
                    if player.lives == 0 {
                        info!("You lost!");
                        app_exit_events.send(AppExit::error());
                    }
                }
                MoleType::Good => {
                    player.score += 1;
                    info!("Current score: {}", player.score);
                }
                MoleType::Extra => {
                    player.score += 2;
                    info!("Current score: {}", player.score);
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Mole {
    key: u8,
    ty: MoleType,
    timer: Timer,
}

#[derive(Debug)]
enum MoleType {
    ExtraBad,
    Bad,
    Good,
    Extra,
}

#[derive(Resource)]
struct Player {
    lives: usize,
    score: usize,
}
