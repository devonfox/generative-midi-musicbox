//! Channel = Atomic Boolean example I wrote after lecture this week
//! Devon Fox - Feb 2022
//!

use queues::*;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let atomicstop = Arc::new(AtomicBool::new(false));
    let (tx, rx): (SyncSender<u64>, Receiver<u64>) = sync_channel(1);
    let mut chords: Buffer<u64> = Buffer::new(8);
    let stopflag = atomicstop.clone();
    let thr = thread::spawn(move || bg_process(&stopflag, tx));

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    atomicstop.store(true, Ordering::Relaxed);

    thr.join().unwrap();
    loop {
        let result = rx.recv();
        match result {
            Ok(x) => {
                if chords.size() == chords.capacity() {
                    let _ = chords.remove();
                }
                let _ = chords.add(x);
            }
            Err(RecvError) => break,
        }
    }

    println!("{:?}", chords);
    Ok(())
}

fn bg_process(atomicstop: &Arc<AtomicBool>, thread_tx: SyncSender<u64>) {
    let mut count: u64 = 0;
    loop {
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", count);
        count += 1;
        thread_tx.send(count).unwrap();
        thread::sleep(Duration::from_secs(1));
        if atomicstop.load(Ordering::Relaxed) {
            break;
        }
    }
}
