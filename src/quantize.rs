use core::iter::Take;
use super::waves::WaveGenerator;

pub fn samples_in_beat(tempo: u8) -> u32 {
    30 * 48000 / (tempo as u32)
}

pub trait Quantizable<T>: WaveGenerator {
    fn quantize(self, tempo: u8, duration: u8, cutoff: u8) -> Take<T>;
}

impl <T> Quantizable<T> for T where T: WaveGenerator {
    fn quantize(self, tempo: u8, duration: u8, cutoff: u8) -> Take<T> {
        let take_amount = samples_in_beat(tempo) * (duration as u32) * (cutoff as u32 + 1) / 256;
        self.take(take_amount as usize)
    }
}

impl <T> WaveGenerator for Take<T> where T: WaveGenerator {}
