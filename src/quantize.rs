use core::iter::Take;
use super::waves::WaveGenerator;

pub fn samples_in_beat(tempo: u8) -> u32 {
    30 * 48000 / (tempo as u32)
}

pub fn quantize<A: WaveGenerator>(generator: A, tempo: u8, duration: u8, cutoff: u8) -> Take<A> {
    let take_amount = samples_in_beat(tempo) * (duration as u32) * (cutoff as u32 + 1) / 256;
    generator.take(take_amount as usize)
}
