pub mod square;
pub mod sawtooth;

pub trait WaveGenerator: ExactSizeIterator<Item = i16> {}
