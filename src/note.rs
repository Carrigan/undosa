use crate::pitch::Pitch;

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
