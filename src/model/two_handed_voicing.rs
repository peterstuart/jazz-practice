use super::{base::*, enum_random::EnumRandom};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
pub enum ChordQuality {
    #[strum(to_string = "")]
    Major,
    #[strum(to_string = "-7")]
    Minor7,
    #[strum(to_string = "7")]
    Dominant7,
    #[strum(to_string = "ø7")]
    HalfDimished7,
    #[strum(to_string = "7(♭13♭9)")]
    Dominant7Flat13Flat9,
    #[strum(to_string = "-")]
    Minor,
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum Pitch {
    Root,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Assignment {
    root: Root,
    chord_quality: ChordQuality,
    top: Pitch,
}

impl Assignment {
    pub fn random() -> Self {
        Self {
            root: Root::random(),
            chord_quality: ChordQuality::random(),
            top: Pitch::random(),
        }
    }

    pub fn chord_string(&self) -> String {
        format!("{}{}", self.root, self.chord_quality)
    }

    pub fn top_string(&self) -> String {
        format!("{} on top", self.top)
    }
}
