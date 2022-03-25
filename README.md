# Generative Midi Musicbox

*Devon Fox 2022*

**Version: 0.1.0**

A Rust library/binary crate that sends looping randomly generated MIDI signals defined by user input via an external MIDI input to an external MIDI source.

## Dependencies

* [Midir 0.7.0](https://crates.io/crates/midir) - Create MIDI input and output connections
* [Wmidi 4.0.4](https://crates.io/crates/wmidi) - Enum data for manipulating MIDI messages
* [Rand 0.8.3](https://crates.io/crates/rand) - Random Number Generation

## Build and Run Instructions

#### Windows / Mac / Linux

Build and run using cargo: `cargo run`

#### Raspberry Pi / Raspbian

To get ALSA working properly on the Raspberry Pi, I recommend installing `libasound2-dev` with the following command 
```
sudo apt install libasound2-dev
```
Then make sure cargo/rust is up-to-date and build and run with: `cargo run`

## Audio Examples

*Included below are some audio examples of the Rust crate in use with a Sequential Prophet Rev2.*

* [Audio Examples](https://soundcloud.com/foxdevpdx/sets/generative-midi-musicbox)

##  Todo: Features and Fixes
* [ ] Create midi message stop condition instead of keyboard input of 'enter'
* [ ] Implement control over speed of randomly generated arpeggio
* [ ] Expand Arpeggiator features (i.e. varying styles)
* [ ] Fix MIDI port argument bug as described below
* [ ] Re-write and re-organize

## Known Bugs

* Midi In/Out Port Arguments only work when there is more than 2 Midi sources available