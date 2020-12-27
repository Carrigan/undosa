use crate::{pitch::Pitch, quantize};
use crate::note::Note;

pub struct Melody<'a> {
    notes: &'a [Note],
    current_sample: u32,
    current_note: u32,
    next_note_start: u32,
    pub tempo: u8
}

impl <'a> Melody<'a> {
    pub fn new(notes: &'a [Note], tempo: u8) -> Self {
        Self {
            notes,
            current_sample: 0,
            current_note: 0,
            next_note_start: 0,
            tempo: tempo
        }
    }

    pub fn next_note(&mut self) -> Option<Note> {
        if self.next_note_start == self.current_sample {
            let next_note = self.notes[self.current_note as usize].clone();

            self.current_note += 1;
            if self.current_note == self.notes.len() as u32 {
                self.current_note = 0;
            }

            self.current_sample = 0;
            self.next_note_start = quantize::samples_in_beat(self.tempo) * (next_note.duration() as u32);

            Some(next_note)
        } else {
            self.current_sample += 1;
            None
        }
    }
}
