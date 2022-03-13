//! Genmidi Library Crate
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
/// Sourced from the 'midir' crate 'test_play.rs' example
pub fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("Main MIDI Output")?;
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = 0;
    let mut midi_in = MidiInput::new("midir reading input")?;
    open(midi_in, midi_out)?;
    // channel for sending midi notes
    let (tx, rx): (Sender<Note>, Receiver<Note>) = channel();

    // channel for sending stop unit message
    let (end_tx, end_rx): (Sender<()>, Receiver<()>) = channel();
    println!("\nOpening output connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    let atomicstop = Arc::new(AtomicBool::new(false));
    let stopflag = atomicstop.clone();

    let in_port_name = midi_in.port_name(in_port)?;
    let in_port = in_port.clone();

    // creating a thread to handle midi output
    // while waiting for user input to stop generation loop
    let gen_thread = spawn(move || {
        println!("Output connection open.");
        generate_arp(&mut conn_out, &stopflag, rx); // currently generating output connection
    });

    let read_thread = spawn(
        move || match read(&in_port_name, in_port, midi_in, tx, end_rx) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        },
    );
    // put midi generation menu and/or functions here
    let mut input = String::new();
    let stdin = stdin();
    input.clear();
    match stdin.read_line(&mut input) {
        Ok(_) => println!("Ending program..."),
        Err(err) => println!("Error: {}", err),
    }; // wait for next enter key press

    // signal with atomic to stop receiving, and sending as well
    let _ = end_tx.send(()); // sending unit () to signal end via channel
    atomicstop.store(true, Ordering::Relaxed);
    //sleep(Duration::from_millis(150)); // why tf did i put this here
    println!("\nClosing output connection");

    // join send/receiving threads before quitting
    match gen_thread.join() {
        Ok(_) => println!("Debug: Gen_thread successfully joined."),
        Err(error) => println!("Error: {:?}", error),
    };
    match read_thread.join() {
        Ok(_) => println!("Debug: Read_thread successfully joined."),
        Err(error) => println!("Error: {:?}", error),
    }

    println!("Output connection closed");
    Ok(())
}

pub fn open(
    midi_in: MidiInput,
    in_ports: Vec<MidiInputPort>,
    in_port: &MidiInputPort,
    midi_out: MidiOutput,
    out_ports: Vec<MidiOutputPort>,
    out_port: &MidiOutputPort,
    end_rx: Receiver<()>
) -> Result<(), Box<dyn Error>> {
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

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => {
            println!("Error: No input connection found. Press enter to quit.");
            let error = Err("closing connections.".into());
            let _ = end_rx.recv();
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
    };
    Ok(())
}

/// Reads midi input and sends the midi notes parsed from midi messages through
/// channels to our output midi function operating in a separate thread.
///
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
        "Input connection open, reading input from '{}' (press enter to stop input) ...",
        in_port_name
    );

    let _ = end_rx.recv();
    input_connection.close();
    println!("Closing input connection");
    Ok(())
}

/// Generates a random arpeggio and updates the mutable output 'connect'
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
        sleep(Duration::from_millis(100));
        let _ = connect.send(&[128, note, rand_vel]);
    };

    sleep(Duration::from_millis(4 * 150)); // small pause
    loop {
        // Try_recv() doesn't block when there is failed recv,
        // and allows playing to continue while waiting for more
        // notes through channel
        let result = rx.try_recv();
        if let Ok(note) = result {
            println!("Debug: Note -> {}, Deque Size: {}", note, note_queue.len());
            if note_queue.len() == 8 {
                let _ = note_queue.pop_back();
            }
            let _ = note_queue.push_front(note as u8);
        }
        if !note_queue.is_empty() {
            for variance in 0..4 {
                sleep(Duration::from_millis(100));

                // consider creating a variable to change
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
    let base_note: usize = rand::thread_rng().gen_range(0..frame.len());
    let variance: [i8; 4] = [24, 12, 0, -12]; // define change in octave
                                              //here make sure value doesn't exceed potential values
    let note = frame[base_note] as i8 + variance[index]; // add change in octave to generated note
    note as u8
}
