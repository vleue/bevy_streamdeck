# Bevy Stream Deck

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Doc](https://docs.rs/bevy_streamdeck/badge.svg)](https://docs.rs/bevy_streamdeck)
[![Crate](https://img.shields.io/crates/v/bevy_streamdeck.svg)](https://crates.io/crates/bevy_streamdeck)
[![Bevy Tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/vleue/bevy_streamdeck/actions/workflows/ci.yml/badge.svg)](https://github.com/vleue/bevy_streamdeck/actions/workflows/ci.yml)

This is a plugin for integrating the [Elgato Stream Deck](https://www.elgato.com/en/stream-deck) with the [Bevy](https://bevyengine.org) game engine. It allows you to handle input events from the Stream Deck and control its display.

![capture of a Stream Deck](https://raw.githubusercontent.com/vleue/bevy_streamdeck/main/capture.png)

## Getting Started

### Adding the Plugin

```rust
app.add_plugin(StreamDeckPlugin);
```

### Handling Input Events

Receive events from button presses (see [inputs example](https://github.com/vleue/bevy_streamdeck/blob/main/examples/inputs.rs)):

```rust
fn print_streamdeck_events(mut streamdeck_input_events: EventReader<StreamDeckInput>) {
    for event in streamdeck_input_events.iter() {
        info!("{:?}", event);
    }
}
```

### Setting Button Colors

Set a button color (see [colors example](https://github.com/vleue/bevy_streamdeck/blob/main/examples/colors.rs)):

```rust
fn set_color(streamdeck: Res<StreamDeck>) {
    streamdeck.set_key_color(1, Color::BLUE);
}

```

### Displaying Images

Display an image on a button (see [image example](https://github.com/vleue/bevy_streamdeck/blob/main/examples/image.rs)):

```rust
fn set_image(streamdeck: Res<StreamDeck>, logo: Res<Logo>, images: Res<Assets<Image>>) {
    let handle: Handle<Image> = ...;
    if let Some(image) = images.get(&handle) {
        streamdeck.set_key_image(1, &image);
    }
}
```

## Platform-Specific Setup

### Linux

For Linux setup, please refer to the [rust-streamdeck getting started guide](https://github.com/ryankurte/rust-streamdeck#getting-started).

## Compatibility

### Bevy Compatibility

| Bevy | bevy_streamdeck |
| ---- | --------------- |
| 0.17 | 0.7             |
| 0.16 | 0.6             |
| 0.15 | 0.5             |
| 0.14 | 0.4             |
| 0.13 | 0.3             |
| 0.9  | 0.2             |
| 0.8  | 0.1             |
