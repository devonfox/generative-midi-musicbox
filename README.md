# Generative Midi Musicbox

A Rust library/binary crate that sends looping randomly generated MIDI signals defined by user input via an external MIDI input to an external MIDI source.

## Dependencies

* Midir - Create MIDI input and output connections
* Wmidi - Enum data for manipulating MIDI messages
* Rand - Random Number Generation

## Planning and Implementation

* [x] Use enum for note names
* [x] Atomic Boolean
* [x] Unravel Channels
* [x] Transfer notes from receiving midi data to sending thread
* [x] Learn how to merge branch after concurrency woes
* [ ] Rewrite and organize now that I understand the complete flow
* [ ] Write instructions for running on Pi
* [ ] Write unit tests in tests folder
* [ ] Fill out README a bit more

## Stretch Goals
* [ ] Implement control over speed of randomly generated arpeggio
