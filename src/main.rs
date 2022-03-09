//! Generative Midi Musicbox Driver
//!
//! Devon Fox 2022

use generative_midi_musicbox::*;

// prophet patch for ambient testing -> U4 P69

fn main() {
    // Driver for running lib code
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
    println!("Program Ended Successfully.");
}
