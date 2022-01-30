use rand::prelude::*;
use std::fmt;
use strum::{Display, EnumIter, IntoEnumIterator};

trait EnumRandom {
    fn random() -> Self;
}

impl<T: IntoEnumIterator> EnumRandom for T {
    fn random() -> Self {
        Self::iter().choose(&mut thread_rng()).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Root {
    C,
    #[strum(to_string = "C♯/D♭")]
    CSharpDFlat,
    D,
    #[strum(to_string = "D♯/E♭")]
    DSharpEFlat,
    E,
    F,
    #[strum(to_string = "F♯/G♭")]
    FSharpGFlat,
    G,
    #[strum(to_string = "A♭")]
    AFlat,
    A,
    #[strum(to_string = "B♭")]
    BFlat,
    B,
}

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
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum ChordPitch {
    Root,
    Third,
    Fifth,
    Seventh,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Approach {
    TwoAbove,
    TwoBelow,
    TwoAboveTwoBelow,
    TwoBelowTwoAbove,
    OneAbove,
    OneBelow,
    OneAboveOneBelow,
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

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!(
            "{}{} {} {}",
            self.root, self.chord_quality, self.chord_pitch, self.approach
        )
        .fmt(f)
    }
}
