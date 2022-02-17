//! Genmidi Library Crate
//!
//! Devon Fox 2022

use midir::*;
use rand::Rng;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

enum CHORD {
    C0 = 1,
}

/// Scans midi ports and lets user connect to port of choice, if available
/// If no ports available, returns an error.
/// Sourced from the 'midir' crate 'test_play.rs' example
pub fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    let mut input = String::new();

    // creating a thread to handle midi output
    // while waiting for user input to stop generation loop
    let _gen_thread = spawn(move || {
        println!("Connection open.");
        generate_arp(&mut conn_out); // currently generating output connection
                                     //generate_random(&mut conn_out); // currently generating output connection
    });
    // put midi generation menu and/or functions here

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    sleep(Duration::from_millis(150));
    println!("\nClosing connection");
    //_test.join().unwrap();
    //conn_out.close();
    println!("Connection closed");
    Ok(())
}

/// Generates a random arpeggio and updates the mutable output 'connect'
pub fn generate_arp(connect: &mut MidiOutputConnection) {
    // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
    let mut play_note = |note: u8, duration: u64| {
        let rand_vel = rand::thread_rng().gen_range(0..100);
        let _ = connect.send(&[144, note, rand_vel]);
        sleep(Duration::from_millis(duration * 50));
        let _ = connect.send(&[128, note, rand_vel]);
    };

    // TODO: Make a single data structure to hold all my chord data;
    let _pretty_chord: [u8; 5] = [60, 63, 65, 67, 70];
    let _happy_chord: [u8; 6] = [60, 64, 67, 69, 71, 74];
    let _a: [u8; 5] = [37, 53, 56, 60, 63];
    let _b: [u8; 5] = [39, 55, 58, 62, 65];
    let _c: [u8; 5] = [41, 57, 60, 64, 67];

    sleep(Duration::from_millis(4 * 150));
    loop {
        for i in 0..4 {
            sleep(Duration::from_millis(100));
            // choosing chord here
            // maybe maybe make 2d vector holding all potential chords
            play_note(random_note(&_c, i), 1);
        }
    }
}

/// Generates random notes(within a chord) and updates the mutable output 'connect'
/// Also, random spacing and velocity
pub fn generate_random(connect: &mut MidiOutputConnection) {
    // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
    let mut play_note = |note: u8, duration: u64| {
        let rand_vel = rand::thread_rng().gen_range(0..100);
        let _ = connect.send(&[144, note, rand_vel]);
        sleep(Duration::from_millis(
            duration * rand::thread_rng().gen_range(1..100),
        ));
        let _ = connect.send(&[128, note, rand_vel]);
    };

    // FIX: Redundant
    let _pretty_chord: [u8; 5] = [60, 63, 65, 67, 70];
    let happy_chord: [u8; 6] = [60, 64, 67, 69, 71, 74];

    sleep(Duration::from_millis(4 * 150));
    //let mut count = 0;
    loop {
        sleep(Duration::from_millis(50));
        play_note(
            random_note(&happy_chord, rand::thread_rng().gen_range(0..4)),
            2,
        );
    }
}

/// A 'normal' arpeggio (within a chord) and updates the mutable output 'connect'
/// Also, random velocity
pub fn arp(connect: &mut MidiOutputConnection) {
    let mut play_note = |note: u8, duration: u64| {
        let rand_vel = rand::thread_rng().gen_range(60..100);
        let _ = connect.send(&[144, note, rand_vel]);
        sleep(Duration::from_millis(duration * 30));
        let _ = connect.send(&[128, note, rand_vel]);
    };

    // FIX: Redundant
    let _pretty_chord: [u8; 5] = [60, 63, 65, 67, 70];
    let _happy_chord: [u8; 6] = [60, 64, 67, 69, 71, 74];
    let _a: [u8; 5] = [37, 53, 56, 60, 63];
    let _b: [u8; 5] = [39, 55, 58, 62, 65];
    let _c: [u8; 5] = [41, 57, 60, 64, 67];

    loop {
        // Maybe use function iterators?

        for _i in 0..5 {
            for j in _a {
                sleep(Duration::from_millis(100));
                play_note(j, 2);
            }
        }
        for _i in 0..5 {
            for j in _b {
                sleep(Duration::from_millis(100));
                play_note(j, 2);
            }
        }
        for _i in 0..5 {
            for j in _c {
                sleep(Duration::from_millis(100));
                play_note(j, 2);
            }
        }
        for _i in 0..5 {
            for j in _b {
                sleep(Duration::from_millis(100));
                play_note(j, 2);
            }
        }
    }
}

/// Creates a random note given the input note, and uses
/// the 'variance' to raise or lower the octave when
/// generating
pub fn random_note(frame: &[u8], index: usize) -> u8 {
    assert!(index < 4, "invalid variance index");
    let base_note: usize = rand::thread_rng().gen_range(0..frame.len());
    let variance: [i8; 4] = [24, 12, 0, -12]; // define change in octave
    let note = frame[base_note] as i8 + variance[index]; // add change in octave to generated note
    note as u8
}
