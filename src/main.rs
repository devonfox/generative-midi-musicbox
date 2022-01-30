//! Generative Midi Musicbox Driver
//! 
//! Devon Fox 2022

use generative_midi_musicbox::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
