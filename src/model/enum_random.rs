use rand::prelude::*;
use strum::IntoEnumIterator;

pub trait EnumRandom {
    fn random() -> Self;
}

impl<T: IntoEnumIterator> EnumRandom for T {
    fn random() -> Self {
        Self::iter().choose(&mut thread_rng()).unwrap()
    }
}
