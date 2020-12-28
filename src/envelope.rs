use crate::waves::WaveGenerator;

pub struct AdsrEnvelope<T> {
    parent_generator: T,
    attack: u8,
    decay: u8,
    sustain: u8,
    release: u8,
    samples_in_note: usize,
    index: usize
}

impl <T> Iterator for AdsrEnvelope<T> where T: WaveGenerator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let next_sample = match self.parent_generator.next() {
            Some(sample) => sample,
            None => return None
        };

        let attack_samples = self.attack as usize * self.samples_in_note / 256;
        let decay_samples = self.decay as usize * self.samples_in_note / 256;
        let release_samples = self.release as usize * self.samples_in_note / 256;

        let multiplier: u8 = match self.index {
            i if i < attack_samples => {
                let progress = (i * 256) / attack_samples;
                progress as u8
            },
            i if i >= attack_samples && i < (attack_samples + decay_samples) => {
                let decay_i = i - attack_samples;
                let progress = ((decay_samples - decay_i) * 256) / decay_samples;

                (progress * self.sustain as usize / 256) as u8 + self.sustain
            },
            i if i >= (self.samples_in_note - release_samples) => {
                let release_i = i - (self.samples_in_note - release_samples);
                let progress = ((release_samples - release_i) * 256) / release_samples;

                (progress * self.sustain as usize / 256) as u8
            },
            _ => {
                self.sustain
            }
        };

        self.index += 1;

        Some(((next_sample as i32 * (multiplier as i32 + 1)) / (256 as i32)) as i16)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.parent_generator.size_hint()
    }
}

impl <T> ExactSizeIterator for AdsrEnvelope<T> where T: WaveGenerator {}
impl <T> WaveGenerator for AdsrEnvelope<T> where T: WaveGenerator {}

pub trait Enveloped<T>: WaveGenerator {
    fn envelope(self, attack: u8, decay: u8, sustain: u8, release: u8) -> AdsrEnvelope<T>;
}

impl <T> Enveloped<T> for T where T: WaveGenerator {
    fn envelope(self, attack: u8, decay: u8, sustain: u8, release: u8) -> AdsrEnvelope<T> {
        let samples_in_note = self.size_hint().1.unwrap();

        AdsrEnvelope {
            parent_generator: self, index: 0, samples_in_note,
            attack, decay, sustain, release
        }
    }
}
