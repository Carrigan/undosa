use core::iter::Take;
use super::waves::WaveGenerator;

pub fn samples_in_beat(tempo: u8) -> u32 {
    30 * 48000 / (tempo as u32)
}

pub fn quantize<A: WaveGenerator>(generator: A, tempo: u8, duration: u8) -> Take<A> {
    generator.take(10)
}
