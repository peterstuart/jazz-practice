use super::{base::*, enum_random::EnumRandom};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
pub enum ChordQuality {
    #[strum(to_string = "Δ7")]
    Major7,
    #[strum(to_string = "-7")]
    Minor7,
    #[strum(to_string = "7")]
    Dominant7,
    #[strum(to_string = "ø7")]
    HalfDimished7,
    #[strum(to_string = "o7")]
    Dimished7,
    #[strum(to_string = "-Δ7")]
    MinorMajor7,
    #[strum(to_string = "7sus4")]
    Dominant7Sus4,
    #[strum(to_string = "Δ7+5")]
    Major7Sharp5,
    #[strum(to_string = "Δ7♭5")]
    Major7Flat5,
    #[strum(to_string = "7+5")]
    Dominant7Sharp5,
    #[strum(to_string = "7♭5")]
    Dominant7Flag5,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Approach {
    #[strum(to_string = "Two above")]
    TwoAbove,
    #[strum(to_string = "Two below")]
    TwoBelow,
    #[strum(to_string = "Two above, two below")]
    TwoAboveTwoBelow,
    #[strum(to_string = "Two below, two above")]
    TwoBelowTwoAbove,
    #[strum(to_string = "One above")]
    OneAbove,
    #[strum(to_string = "One below")]
    OneBelow,
    #[strum(to_string = "One above, one below")]
    OneAboveOneBelow,
    #[strum(to_string = "One below, one above")]
    OneBelowOneAbove,
    #[strum(to_string = "One below, two above")]
    OneBelowTwoAbove,
    #[strum(to_string = "Two above, one below")]
    TwoAboveOneBelow,
    #[strum(to_string = "Two below, one above")]
    TwoBelowOneAbove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Assignment {
    root: Root,
    chord_quality: ChordQuality,
    chord_pitch: ChordPitch,
    approach: Approach,
}

impl Assignment {
    pub fn random() -> Self {
        Self {
            root: Root::random(),
            chord_quality: ChordQuality::random(),
            chord_pitch: ChordPitch::random(),
            approach: Approach::random(),
        }
    }

    pub fn chord_string(&self) -> String {
        format!("{}{}", self.root, self.chord_quality)
    }

    pub fn chord_pitch_string(&self) -> String {
        self.chord_pitch.to_string()
    }

    pub fn approach_string(&self) -> String {
        self.approach.to_string()
    }
}
