use strum::{Display, EnumIter};

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

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum ChordPitch {
    Root,
    Third,
    Fifth,
    Seventh,
}
