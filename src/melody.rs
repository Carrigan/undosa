use super::pitch::Pitch;

#[derive(Clone, Copy)]
pub enum Note {
    Eighth(Pitch),
    EightRest,
    Quarter(Pitch),
    QuarterRest,
    Half(Pitch),
    HalfRest,
    Whole(Pitch),
    WholeRest
}

impl Note {
    pub fn duration(self) -> u8{
        match self {
            Note::Eighth(_) | Note::EightRest => 1,
            Note::Quarter(_) | Note::QuarterRest => 2,
            Note::Half(_) | Note::HalfRest => 4,
            Note::Whole(_) | Note::WholeRest => 8,
        }
    }
}

impl Into<Option<Pitch>> for Note {
    fn into(self) -> Option<Pitch> {
        match self {
            Note::Eighth(pitch) => Some(pitch),
            Note::Quarter(pitch) => Some(pitch),
            Note::Half(pitch) => Some(pitch),
            Note::Whole(pitch) => Some(pitch),
            _ => None
        }
    }
}

enum MelodyEvent {
    NoteStart(u32),
    NoteStop(u32)
}

pub struct Melody<'a> {
    notes: &'a [Note],
    current_sample: u32,
    current_note: u32,

    next_event: MelodyEvent,
    on_spacing: u32,
    off_spacing: u32
}

impl <'a> Melody<'a> {
    pub fn new(notes: &'a [Note], tempo: u8) -> Self {
        let note_spacing = 30 * 48000 / (tempo as u32);
        let off_spacing = note_spacing / 10;
        let on_spacing = note_spacing - off_spacing;

        Self {
            notes,
            on_spacing,
            off_spacing,
            current_sample: 0,
            current_note: 0,
            next_event: MelodyEvent::NoteStart(0)
        }
    }

    pub fn next_sample(&mut self) -> (bool, Option<Pitch>) {
        let next_timing = match self.next_event {
            MelodyEvent::NoteStart(t) => t,
            MelodyEvent::NoteStop(t) => t
        };

        let event_ready = self.current_sample == next_timing;
        self.current_sample += 1;

        if !event_ready { return (false, None) }

        let note = self.notes[self.current_note as usize];

        match self.next_event {
            MelodyEvent::NoteStart(_t) => {
                self.next_event = MelodyEvent::NoteStop(self.current_sample + (self.on_spacing * note.duration() as u32));
                (true, note.into())
            },

            MelodyEvent::NoteStop(_) => {
                self.next_event = MelodyEvent::NoteStart(self.current_sample + (self.off_spacing * note.duration() as u32));

                self.current_note += 1;
                if self.current_note == self.notes.len() as u32 {
                    self.current_note = 0;
                }

                (false, None)
            }
        }
    }
}
