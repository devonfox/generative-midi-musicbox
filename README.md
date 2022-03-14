# Generative Midi Musicbox

A Rust library/binary crate that sends looping randomly generated MIDI signals defined by user input via an external MIDI input to an external MIDI source.

## Dependencies

* [Midir 0.7.0](https://crates.io/crates/midir) - Create MIDI input and output connections
* [Wmidi 4.0.4](https://crates.io/crates/wmidi) - Enum data for manipulating MIDI messages
* [Rand 0.8.3](https://crates.io/crates/rand) - Random Number Generation

## Build and Run instructions

#### Windows / Mac

Build and run using cargo: `cargo run`

#### Linux / Raspberry Pi

*todo!*

## Planning and Implementation

* [x] Use enum for note names
* [x] Atomic Boolean
* [x] Unravel Channels
* [x] Transfer notes from receiving midi data to sending thread
* [x] Learn how to merge branch after concurrency woes
* [ ] Rewrite and organize now that I understand the complete flow
* [ ] Write instructions for running on Pi
* [ ] Write unit tests in tests folder
* [x] Fill out README a bit more

## Stretch Goals
* [ ] Create midi message stop condition instead of keyboard input of 'enter'
* [ ] Implement control over speed of randomly generated arpeggio
* [ ] Expand Arpeggiator features (i.e. varying styles)
