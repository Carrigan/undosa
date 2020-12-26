use super::WaveGenerator;

pub struct SawtoothWaveGenerator {
    current_position: i16,
    advancement: i16
}

impl SawtoothWaveGenerator {
    pub fn new(sample_frequency: usize, frequency: usize) -> Self {
        Self {
            current_position: 0,
            advancement: ((frequency as u32 * core::u16::MAX as u32) / sample_frequency as u32) as i16
        }
    }
}

impl Iterator for SawtoothWaveGenerator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_position: i32 = self.current_position as i32 + self.advancement as i32;
        if next_position > core::i16::MAX as i32 { next_position -= core::u16::MAX as i32 }
        self.current_position = next_position as i16;

        Some(self.current_position)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl ExactSizeIterator for SawtoothWaveGenerator {}
impl WaveGenerator for SawtoothWaveGenerator { }
