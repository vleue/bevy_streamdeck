use std::time::Duration;

use bevy::{
    prelude::{debug, App, Color, Commands, Plugin, Res, ResMut},
    tasks::{IoTaskPool, Task},
};
use crossbeam_channel::{bounded, Receiver, Sender};
use streamdeck::{Colour, Error, Kind};

pub struct StreamDeckPlugin;

impl Plugin for StreamDeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(listener).add_system(receiver);
    }
}

#[derive(Debug)]
enum StreamDeckEvent {
    LostConnection,
    Connected(streamdeck::Kind),
    ButtonPressed(Vec<u8>),
}

enum StreamDeckOrder {
    Color(u8, Color),
}

fn listener(taskpool: Res<IoTaskPool>, mut commands: Commands) {
    let (event_tx, event_rx) = bounded::<StreamDeckEvent>(10);
    let (order_tx, order_rx) = bounded::<StreamDeckOrder>(100);

    let task = taskpool.spawn(async move {
        let mut streamdeck: Option<streamdeck::StreamDeck> = None;
        loop {
            let mut lost_connection = false;
            if let Some(streamdeck) = streamdeck.as_mut() {
                let mut act = || {
                    let read = streamdeck.read_buttons(Some(Duration::from_millis(1)));
                    match read {
                        Ok(read) => {
                            let _ = event_tx.send(StreamDeckEvent::ButtonPressed(read));
                        }
                        Err(Error::NoData) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    }

                    for order in order_rx.try_iter() {
                        match order {
                            StreamDeckOrder::Color(k, color) => {
                                let [r, g, b, _] = color.as_rgba_f32();
                                match streamdeck.set_button_rgb(
                                    k,
                                    &Colour {
                                        r: (r * 255.0) as u8,
                                        g: (g * 255.0) as u8,
                                        b: (b * 255.0) as u8,
                                    },
                                ) {
                                    Ok(_) => (),
                                    Err(Error::Hid(error)) => {
                                        debug!("HidError {:?}", error)
                                    }
                                    Err(err) => return Err(err),
                                }
                            }
                        }
                    }
                    Ok(())
                };
                if let Err(error) = act() {
                    debug!("Error communicating with StreamDeck: {:?}", error);
                    let _ = event_tx.send(StreamDeckEvent::LostConnection);
                    lost_connection = true;
                }
            }
            if lost_connection {
                streamdeck = None;
            }
            if streamdeck.is_none() {
                if let Ok(new_streamdeck) = streamdeck::StreamDeck::connect(0x0fd9, 0x0063, None) {
                    let _ = event_tx.send(StreamDeckEvent::Connected(new_streamdeck.kind()));
                    streamdeck = Some(new_streamdeck);
                }
            }
        }
    });
    commands.insert_resource(StreamDeckInternal {
        task,
        events: event_rx,
    });
    commands.insert_resource(StreamDeck {
        orders: order_tx,
        kind: None,
    });
}

fn receiver(mut streamdeck: ResMut<StreamDeck>, internal: Res<StreamDeckInternal>) {
    for from_stream in internal.events.try_iter() {
        println!("-> {:?}", from_stream);
        match from_stream {
            StreamDeckEvent::LostConnection => streamdeck.kind = None,
            StreamDeckEvent::Connected(kind) => streamdeck.kind = Some(kind),
            StreamDeckEvent::ButtonPressed(_) => (),
        }
    }
}

struct StreamDeckInternal {
    #[allow(dead_code)]
    task: Task<()>,
    events: Receiver<StreamDeckEvent>,
}

pub struct StreamDeck {
    orders: Sender<StreamDeckOrder>,
    kind: Option<Kind>,
}

impl StreamDeck {
    pub fn set_key_color(&self, key: u8, color: Color) {
        let _ = self.orders.send(StreamDeckOrder::Color(key, color));
    }

    pub fn reset_key(&self, key: u8) {
        let _ = self.orders.send(StreamDeckOrder::Color(key, Color::BLACK));
    }
}
