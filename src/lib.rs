//! Generative-Midi-Musicbox Library Crate
//!
//! Devon Fox 2022

use midir::*;
use midir::{Ignore, MidiInput};
use rand::Rng;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use wmidi::MidiMessage::*;
use wmidi::*;

/// Scans midi ports and lets user connect to port of choice, if available
/// If no ports available, returns an error.
/// Sourced from the 'midir' crate 'test_play.rs' & 'test_read.rs' examples
pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // create an instance of a midi output
    let midi_out = MidiOutput::new("Main MIDI Output")?;
    // channel for sending midi notes
    let note_chan: (Sender<Note>, Receiver<Note>) = channel();
    // channel for sending stop unit message to stop running threads
    let end_chan: (Sender<()>, Receiver<()>) = channel();
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
        _ => match args.len() > 2 {
            true => out_ports
                .get(args[1].trim().parse::<usize>()?)
                .ok_or("invalid argument given for output port selected")?,
            false => {
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
        },
    };

    println!("\nOpening output connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    let atomicstop = Arc::new(AtomicBool::new(false));
    let stopflag = atomicstop.clone();

    // Get an input port (read from console if multiple are available)
    let mut midi_in = MidiInput::new("midir reading input")?;
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => {
            println!("Error: No input connection found. Press enter to quit.");
            let error = Err("closing connections.".into());
            let _ = end_chan.1.recv(); // closing end channel if error
            return error;
        }
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            match args.len() > 2 {
                true => in_ports
                    .get(args[2].trim().parse::<usize>()?)
                    .ok_or("invalid arguments given for input port selected")?,
                false => {
                    println!("\nAvailable input ports:");
                    for (i, p) in in_ports.iter().enumerate() {
                        println!("{}: {}", i, midi_in.port_name(p).unwrap());
                    }
                    print!("Please select input port: ");
                    stdout().flush()?;
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;
                    in_ports
                        .get(input.trim().parse::<usize>()?) // investigate
                        .ok_or("invalid input port selected")?
                }
            }
        }
    };
    midi_in.ignore(Ignore::None);
    let in_port_name = midi_in.port_name(in_port)?;
    let in_port = in_port.clone();

    // creating a thread to handle midi output
    // while waiting for user input to stop generation loop
    let gen_thread = spawn(move || {
        println!("Output connection open.");
        // generating random arp midi to output
        // while reading input
        generate_arp(&mut conn_out, &stopflag, note_chan.1);
    });

    let read_thread =
        spawn(
            move || match read(&in_port_name, in_port, midi_in, note_chan.0, end_chan.1) {
                Ok(_) => (),
                Err(err) => println!("Error: {}", err),
            },
        );
    let mut input = String::new();
    let stdin = stdin();
    input.clear();
    match stdin.read_line(&mut input) {
        Ok(_) => println!("Ending program..."),
        Err(err) => println!("Error: {}", err),
    }; // wait for next enter key press to end program

    // signal with atomic to stop receiving, and sending as well
    let _ = end_chan.0.send(()); // sending unit () to signal end via channel
    atomicstop.store(true, Ordering::Relaxed);
    //sleep(Duration::from_millis(150)); // why tf did i put this here
    println!("\nClosing output connection");

    // join send/receiving threads before quitting
    match gen_thread.join() {
        Ok(_) => (),
        Err(error) => println!("Error: {:?}", error),
    };
    match read_thread.join() {
        Ok(_) => (),
        Err(error) => println!("Error: {:?}", error),
    }
    println!("Output connection closed");
    Ok(())
}

/// Reads midi input and sends the midi notes parsed from midi messages through
/// channels to our output midi function operating in a separate thread.
pub fn read(
    in_port_name: &str,
    in_port: MidiInputPort,
    midi_in: MidiInput,
    tx: Sender<Note>,
    end_rx: Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    println!("\nOpening input connection");
    let input_connection = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |_, message, _| {
            //println!("{:?} (len = {})", message, message.len());
            let message = MidiMessage::try_from(message).unwrap(); //unwrapping message slice
            if let NoteOn(_, note, _) = message {
                let _ = tx.send(note); // sending note value through channel
            }
        },
        (),
    )?;

    println!(
        "Input connection open, reading input from '{}' (press enter to stop input): ",
        in_port_name
    );

    let _ = end_rx.recv();
    input_connection.close();
    println!("Closing input connection");
    Ok(())
}

/// Generates a random arpeggio and updates the mutable output 'connect'
/// Receives midi notes from the read midi input function 'read()'
pub fn generate_arp(
    connect: &mut MidiOutputConnection,
    atomicstop: &Arc<AtomicBool>,
    rx: Receiver<Note>,
) {
    // A data structure used to hold our FIFO 8 note collection
    let mut note_queue: VecDeque<u8> = VecDeque::new();

    let mut play_note = |note: u8| {
        let rand_vel = rand::thread_rng().gen_range(0..100);
        let _ = connect.send(&[144, note, rand_vel]);
        // note length
        sleep(Duration::from_millis(100));
        let _ = connect.send(&[128, note, rand_vel]);
    };

    sleep(Duration::from_millis(500)); // small pause
    loop {
        // Try_recv() doesn't block when there is failed recv,
        // and allows playing to continue while waiting for more
        // notes through channel
        let result = rx.try_recv();
        if let Ok(note) = result {
            if note_queue.len() == 8 {
                let _ = note_queue.pop_back();
            }
            let _ = note_queue.push_front(note as u8);
            display_note_queue(&note_queue);
        }
        if !note_queue.is_empty() {
            for variance in 0..4 {
                // pause between notes, consider adding another control paramter
                // to change this, in order to change speed (i.e. fader or knob
                sleep(Duration::from_millis(100));
                play_note(random_note(&note_queue, variance));
            }
        }
        if atomicstop.load(Ordering::Relaxed) {
            break;
        }
    }
}

/// Creates a random note given the input note, and uses
/// the 'variance' to raise or lower the octave when
/// generating
pub fn random_note(frame: &VecDeque<u8>, index: usize) -> u8 {
    assert!(index < 4, "invalid variance index");
    // base note chooses a random note in the VecDeque to seed
    // before variance is applied
    let base_note: usize = rand::thread_rng().gen_range(0..frame.len());
    let variance: [i16; 4] = [24, 12, 0, -12]; // define change in octave
    let note = {
        // checking to make sure we have a valid note value
        assert!(
            frame[base_note] <= 127 && frame[base_note] > 0,
            "incoming note value not between 0 and 127"
        );
        if frame[base_note] as i16 + variance[index] < 0
            || frame[base_note] as i16 + variance[index] > 127
        {
            frame[base_note] as i16
        } else {
            frame[base_note] as i16 + variance[index]
        }
    };
    note as u8
}

/// Displays contents of current note pool for source of generation to user
pub fn display_note_queue(notes: &VecDeque<u8>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("Note Pool: [");
    let mut index = 0;
    for note in notes {
        index += 1;
        let midi_note = Note::from_u8_lossy(*note);
        match index == notes.len() {
            true => print!("{}", midi_note),
            false => print!("{},", midi_note),
        }
    }
    println!("]\n");
}
