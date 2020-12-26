pub struct Mixer {
    summation: i32
}

impl Mixer {
    pub fn new() -> Mixer {
        Mixer { summation: 0 }
    }

    pub fn add(self, sample: i16, level: u8) -> Mixer {
        Mixer { summation: self.summation + ((level as i32 + 1) * sample as i32) }
    }

    pub fn finish(self, level: u8) -> i16 {
        ((self.summation * (level as i32 + 1)) / (256 * 256)) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer() {
        let mixed = Mixer::new()
            .add(100, 255)
            .add(-100, 127)
            .finish(255);

        assert_eq!(mixed, 50);
    }
}
