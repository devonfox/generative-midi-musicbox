use std::collections::VecDeque;
use generative_midi_musicbox::*;

#[test]
fn test_random_note() {
    let mut notes: VecDeque<u8> = VecDeque::with_capacity(8);
    notes.push_back(0);
    notes.push_back(10);
    notes.push_back(25);
    notes.push_back(66);
    notes.push_back(54);
    notes.push_back(90);
    notes.push_back(110);
    notes.push_back(127);

    for variance in 0..4 {
        random_note(&notes, variance);
    }
    
}