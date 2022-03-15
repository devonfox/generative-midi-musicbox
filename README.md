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

## Planning and Implementation

* [x] Use enum for note names
* [x] Atomic Boolean
* [x] Unravel Channels
* [x] Transfer notes from receiving midi data to sending thread
* [x] Learn how to merge branch after concurrency woes
* [x] Write instructions for running on Pi
* [x] Write unit tests in tests folder
* [ ] ~~Re-write and re-organize~~

## Stretch Goals
* [ ] Create midi message stop condition instead of keyboard input of 'enter'
* [ ] Implement control over speed of randomly generated arpeggio
* [ ] Expand Arpeggiator features (i.e. varying styles)
* [ ] Fix MIDI port argument bug as described below
* [ ] Re-write and re-organize

## How Things Went

This was fun! I learned quite a lot while getting things together to a semi-workable state, and it seemed rather painless up until that point, however once I   That being said, the final push to reorganize caused me more trouble that I thought it would, as it was my first brush with concurrency, and tt was a marginally tough to get things more streamlined that I had them originally.  I felt there was a lack of modularity to the code, and I found this to be more difficult to test the units, as there weren't many units to consider.  I was able to test both the arp generation (it sends a predetermined set of notes over a 10 second period), as well as the note randomization, accounting for whether or not my randomization will put the note values outside of midi's standard 0-127.  In my project, I had a quality in this function called "variance" and we added either -12, 0, 12, or 24 to the note passed in.  This was to keep with an upward arpeggio motion.  That way if a note of 120 was entered, then we would not apply variance, same with a note falling at 2 or 3 (although my synthesizer doesn't even allow notes that low, I still tested with a test function).

I learned a LOT about concurrency, as well ways to make my code more 'rustic'.  I believe I will not stop after the due date, and will push ahead to rewrite and reorganize the entire project to make it more modular, when time allows.  As well, as push for the stretch goals so I can push this binary application as an actual download on crates.io.

## Known Bugs

* Midi In/Out Port Arguments only work when there is more than 2 Midi sources available

I have it set up so if you already know your desired port numbers, you can simply run the program with arguments and it will select the ports give with the following output.  This should be a rather straight forward fix, but I only implemented this last minute so I could try to get this working headlessly on the rpi.  Had troubles, but going to add to my stretch goals.
