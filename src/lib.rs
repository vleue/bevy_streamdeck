use std::time::Duration;

#[cfg(feature = "color_compatibility")]
pub use bevy::color::{Color, LinearRgba};
#[cfg(feature = "image_compatibility")]
use bevy::image::Image;
use bevy::{
    app::{App, AppExit, Plugin},
    color::ColorToComponents,
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut},
    },
    input::ButtonInput,
    log::debug,
    prelude::{Event, Last, PreStartup, PreUpdate, Resource},
    tasks::IoTaskPool,
};
use crossbeam_channel::{bounded, Receiver, Sender};
#[cfg(feature = "images")]
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Pixel, Rgba};
pub use streamdeck::Kind;
use streamdeck::{Colour, Error};

pub struct StreamDeckPlugin;

impl Plugin for StreamDeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StreamDeckInput>()
            .init_resource::<ButtonInput<StreamDeckKey>>()
            .add_systems(PreStartup, listener)
            .add_systems(PreUpdate, receiver)
            .add_systems(Last, exit_on_exit);
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct StreamDeckKey(pub u8);

#[derive(Event, Debug)]
pub enum StreamDeckInput {
    Press(u8),
    Release(u8),
    Disconnected,
    Connected(Kind),
}

#[derive(Debug)]
enum StreamDeckEvent {
    LostConnection,
    Connected(streamdeck::Kind),
    KeyPressed(Vec<u8>),
}

#[cfg(not(feature = "color_compatibility"))]
#[derive(Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[cfg(not(feature = "color_compatibility"))]
impl Color {
    const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    fn as_rgba_f32(&self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }
}

enum StreamDeckOrder {
    Reset,
    Color(u8, Color),
    #[cfg(feature = "images")]
    Image(u8, DynamicImage),
    Exit,
}

fn listener(mut commands: Commands) {
    let (event_tx, event_rx) = bounded::<StreamDeckEvent>(10);
    let (order_tx, order_rx) = bounded::<StreamDeckOrder>(100);

    let taskpool = IoTaskPool::get();
    let task = taskpool.spawn(async move {
        let mut streamdeck: Option<streamdeck::StreamDeck> = None;
        loop {
            let mut lost_connection = false;
            if let Some(streamdeck) = streamdeck.as_mut() {
                let mut act = || {
                    let read = streamdeck.read_buttons(Some(Duration::from_millis(1)));
                    match read {
                        Ok(read) => {
                            let _ = event_tx.send(StreamDeckEvent::KeyPressed(read));
                        }
                        Err(Error::NoData) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    }

                    for order in order_rx.try_iter() {
                        match match order {
                            StreamDeckOrder::Exit => return Ok(false),
                            StreamDeckOrder::Reset => streamdeck.reset(),
                            StreamDeckOrder::Color(k, color) => {
                                let [r, g, b, _] = color.to_linear().to_f32_array();
                                streamdeck.set_button_rgb(
                                    k + 1,
                                    &Colour {
                                        r: (r * 255.0) as u8,
                                        g: (g * 255.0) as u8,
                                        b: (b * 255.0) as u8,
                                    },
                                )
                            }
                            #[cfg(feature = "images")]
                            StreamDeckOrder::Image(k, image) => {
                                streamdeck.set_button_image(k + 1, image)
                            }
                        } {
                            Ok(_) => (),
                            Err(Error::Hid(error)) => {
                                debug!("HidError {:?}", error)
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                    Ok(true)
                };
                match act() {
                    Ok(true) => (),
                    Ok(false) => break,
                    Err(error) => {
                        debug!("Error communicating with StreamDeck: {:?}", error);
                        let _ = event_tx.send(StreamDeckEvent::LostConnection);
                        lost_connection = true;
                    }
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
    task.detach();
    commands.insert_resource(StreamDeckInternal { events: event_rx });
    commands.insert_resource(StreamDeck {
        orders: order_tx,
        kind: None,
    });
}

fn receiver(
    mut streamdeck: ResMut<StreamDeck>,
    internal: Res<StreamDeckInternal>,
    mut inputs: ResMut<ButtonInput<StreamDeckKey>>,
    mut input_events: EventWriter<StreamDeckInput>,
) {
    inputs.clear();
    for from_stream in internal.events.try_iter() {
        match from_stream {
            StreamDeckEvent::LostConnection => {
                streamdeck.kind = None;
                input_events.send(StreamDeckInput::Disconnected);
            }
            StreamDeckEvent::Connected(kind) => {
                streamdeck.kind = Some(kind);
                input_events.send(StreamDeckInput::Connected(kind));
            }
            StreamDeckEvent::KeyPressed(keys) => {
                for (k, s) in keys.iter().enumerate() {
                    if *s == 1 && !inputs.pressed(StreamDeckKey(k as u8)) {
                        inputs.press(StreamDeckKey(k as u8));
                        input_events.send(StreamDeckInput::Press(k as u8));
                    }

                    if *s == 0 && inputs.pressed(StreamDeckKey(k as u8)) {
                        inputs.release(StreamDeckKey(k as u8));
                        input_events.send(StreamDeckInput::Release(k as u8));
                    }
                }
            }
        }
    }
}

#[derive(Resource)]
struct StreamDeckInternal {
    events: Receiver<StreamDeckEvent>,
}

#[derive(Resource)]
pub struct StreamDeck {
    orders: Sender<StreamDeckOrder>,
    kind: Option<Kind>,
}

impl StreamDeck {
    pub fn kind(&self) -> Option<Kind> {
        self.kind
    }

    pub fn set_key_color(&self, key: u8, color: Color) {
        let _ = self.orders.send(StreamDeckOrder::Color(key, color));
    }

    #[cfg(feature = "images")]
    pub fn set_key_image(&self, key: u8, image: &Image) {
        self.set_key_image_with_mode(key, image, ImageMode::default())
    }

    #[cfg(feature = "images")]
    pub fn set_key_image_with_mode(&self, key: u8, image: &Image, image_mode: ImageMode) {
        if let Some(kind) = self.kind {
            // Convert the texture to an image
            let mut dynamic_image = match image.texture_descriptor.format {
                bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb => {
                    ImageBuffer::from_raw(
                        image.texture_descriptor.size.width,
                        image.texture_descriptor.size.height,
                        image.data.clone(),
                    )
                    .map(DynamicImage::ImageRgba8)
                }
                _ => unimplemented!(),
            }
            .unwrap();

            // Resize the image to the size supported by the Stream Deck
            let (x, y) = kind.image_size();
            dynamic_image = match image_mode.resize {
                ImageResize::Exact => {
                    dynamic_image.resize_exact(x as u32, y as u32, FilterType::Gaussian)
                }
                ImageResize::Aspect => {
                    dynamic_image.resize(x as u32, y as u32, FilterType::Gaussian)
                }
                ImageResize::AspectFill => {
                    dynamic_image.resize_to_fill(x as u32, y as u32, FilterType::Gaussian)
                }
            };

            // Apply a background
            if let Some(background) = image_mode.background {
                let LinearRgba {
                    red, green, blue, ..
                } = background.to_linear();

                for pixel in dynamic_image.as_mut_rgba8().unwrap().pixels_mut() {
                    pixel.blend(&Rgba([
                        (red * 255.0) as u8,
                        (green * 255.0) as u8,
                        (blue * 255.0) as u8,
                        255 - pixel.0[3],
                    ]));
                }
            }

            // Invert
            if image_mode.invert {
                dynamic_image.invert();
            }

            let _ = self.orders.send(StreamDeckOrder::Image(key, dynamic_image));
        }
    }

    pub fn reset_key(&self, key: u8) {
        let _ = self.orders.send(StreamDeckOrder::Color(key, Color::BLACK));
    }

    pub fn reset(&self) {
        let _ = self.orders.send(StreamDeckOrder::Reset);
    }
}

fn exit_on_exit(streamdeck: Res<StreamDeck>, mut exit_events: EventReader<AppExit>) {
    if exit_events.read().next().is_some() {
        let _ = streamdeck.orders.send(StreamDeckOrder::Reset);
        let _ = streamdeck.orders.send(StreamDeckOrder::Exit);
    }
}

#[cfg(feature = "images")]
pub enum ImageResize {
    /// Does not preserve aspect ratio.
    Exact,
    /// The image's aspect ratio is preserved.
    Aspect,
    /// The image's aspect ratio is preserved.
    /// The image is scaled to the maximum possible size that fits within the
    /// larger (relative to aspect ratio) of the bounds, then cropped to fit
    /// within the other bound.
    AspectFill,
}

#[cfg(feature = "images")]
impl Default for ImageResize {
    fn default() -> Self {
        Self::Exact
    }
}

#[cfg(feature = "images")]
#[derive(Default)]
pub struct ImageMode {
    pub resize: ImageResize,
    pub invert: bool,
    pub background: Option<Color>,
}
