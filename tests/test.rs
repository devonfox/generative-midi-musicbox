use generative_midi_musicbox::*;
use midir::*;
use rand::*;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;
use wmidi::*;

#[test]
fn test_random_note_bounds() {
    let mut notes: VecDeque<u8> = VecDeque::with_capacity(8);

    notes.push_back(1);
    notes.push_back(10);
    notes.push_back(25);
    notes.push_back(66);
    notes.push_back(54);
    notes.push_back(90);
    notes.push_back(110);
    notes.push_back(127);
    for _i in 0..300 {
        for variance in 0..4 {
            let note = random_note(&notes, variance);
            assert!(note > 0 && note <= 127);
        }
    }
}

#[test]
fn test_generate_arp() -> Result<(), Box<dyn Error>> {
    let note_chan: (Sender<Note>, Receiver<Note>) = channel();
    let atomicstop = Arc::new(AtomicBool::new(false));
    let stopflag = atomicstop.clone();
    let midi_out = MidiOutput::new("test client")?;
    let out_ports = midi_out.ports();
    let test_index = rand::thread_rng().gen_range(0..out_ports.len());
    let out_port = out_ports.get(test_index).ok_or("invalid port")?;
    let mut conn_out = midi_out.connect(out_port, "crate test")?;

    let gen_thread = spawn(move || {
        println!("Output connection open.");
        generate_arp(&mut conn_out, &stopflag, note_chan.1, 120);
    });

    let note_queue = vec![60, 64, 67, 69, 72, 76, 79, 84, 50, 53];

    for note in note_queue {
        let _ = note_chan.0.send(Note::from_u8_lossy(note));
    }

    // runs for 10 seconds to account for purposely slow note buffer.
    // todo: add a arp generation speed for quicker testing (or quicker arps!)
    sleep(Duration::from_secs(10));
    atomicstop.store(true, Ordering::Relaxed);
    match gen_thread.join() {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }

    Ok(())
}
