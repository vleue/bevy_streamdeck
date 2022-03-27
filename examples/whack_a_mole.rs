use std::iter;

use bevy::{app::AppExit, core::FixedTimestep, log::LogPlugin, prelude::*};
use bevy_streamdeck::{StreamDeck, StreamDeckButton, StreamDeckPlugin};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(StreamDeckPlugin)
        .add_startup_system(clean)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spawn_mole),
        )
        .add_system_to_stage(CoreStage::PostUpdate, despawn_mole)
        .add_system(whack)
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

    if let Some(kind) = streamdeck.kind {
        let current = moles.iter().map(|m| m.button).collect::<Vec<_>>();
        let key = iter::repeat(())
            .map(|_| rng.gen_range(0..kind.keys()))
            .find(|k| !current.contains(k))
            .unwrap();

        let max_duration = 180.0;
        if rng.gen_bool(
            (1.0 - (max_duration - time.seconds_since_startup()) / max_duration).clamp(0.2, 0.9),
        ) {
            streamdeck.set_key_color(key, Color::RED);
            commands.spawn_bundle((Mole {
                button: key,
                good: false,
                timer: Timer::from_seconds(rng.gen_range(0.9..1.3), false),
            },));
        } else {
            streamdeck.set_key_color(key, Color::GREEN);
            commands.spawn_bundle((Mole {
                button: key,
                good: true,
                timer: Timer::from_seconds(rng.gen_range(0.7..1.2), false),
            },));
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
            streamdeck.reset_key(mole.button);
        }
    }
}

fn whack(
    mut commands: Commands,
    moles: Query<(Entity, &Mole)>,
    streamdeck_button: Res<Input<StreamDeckButton>>,
    streamdeck: Res<StreamDeck>,
    mut player: ResMut<Player>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (entity, mole) in moles.iter() {
        if streamdeck_button.just_pressed(StreamDeckButton(mole.button)) {
            commands.entity(entity).despawn();
            streamdeck.reset_key(mole.button);
            if mole.good {
                player.score += 1;
                info!("Current score: {}", player.score);
            } else {
                player.lives -= 1;
                info!("Ouch!");
                if player.lives == 0 {
                    info!("You lost!");
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Mole {
    button: u8,
    good: bool,
    timer: Timer,
}

struct Player {
    lives: usize,
    score: usize,
}
