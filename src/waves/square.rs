use super::WaveGenerator;

pub struct SquareWaveGenerator {
    current_position: u16,
    advancement: u16
}

impl SquareWaveGenerator {
    pub fn new(sample_frequency: usize, frequency: usize) -> Self {
        Self {
            current_position: 0,
            advancement: ((frequency as u32 * core::u16::MAX as u32) / sample_frequency as u32) as u16
        }
    }
}

impl Iterator for SquareWaveGenerator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_position: u32 = self.current_position as u32 + self.advancement as u32;
        if next_position > core::u16::MAX as u32 { next_position -= core::u16::MAX as u32 }
        self.current_position = next_position as u16;

        if self.current_position < (core::u16::MAX / 2) { Some(core::i16::MIN) } else { Some(core::i16::MAX) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl ExactSizeIterator for SquareWaveGenerator {}
impl WaveGenerator for SquareWaveGenerator { }
