//! Channel = Atomic Boolean example I wrote after lecture this week
//! Devon Fox - Feb 2022
//! 

use queues::*;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let atomicflag = Arc::new(AtomicBool::new(false));
    let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();
    let mut chords: Buffer<u64> = Buffer::new(8);
    let stopflag = atomicflag.clone();
    let thr = thread::spawn(move || tester(&stopflag, tx));

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    atomicflag.store(true, Ordering::Relaxed);

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

fn tester(atomicflag: &Arc<AtomicBool>, thread_tx: Sender<u64>) {
    let mut count: u64 = 0;
    loop {
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", count);
        count += 1;
        thread_tx.send(count).unwrap();
        thread::sleep(Duration::from_secs(1));
        if atomicflag.load(Ordering::Relaxed) {
            break;
        }
    }
}