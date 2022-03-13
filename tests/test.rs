use generative_midi_musicbox::*;
use std::collections::VecDeque;

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

    for variance in 0..4 {
        
        let note = random_note(&notes, variance);
        assert!(note > 0 && note <= 127);
    }
}
