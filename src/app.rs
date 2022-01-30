use rand::prelude::*;
use std::{collections::HashSet, fmt};
use strum::{Display, EnumIter, IntoEnumIterator};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
enum Root {
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

impl Root {
    fn random() -> Self {
        Self::iter().choose(&mut thread_rng()).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
enum ChordQuality {
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

impl ChordQuality {
    fn all() -> HashSet<ChordQuality> {
        ChordQuality::iter().collect()
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
enum ChordPitch {
    Root,
    Third,
    Fifth,
    Seventh,
}

impl ChordPitch {
    fn random() -> Self {
        Self::iter().choose(&mut thread_rng()).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
enum Approach {
    TwoAbove,
    TwoBelow,
    TwoAboveTwoBelow,
    TwoBelowTwoAbove,
    OneAbove,
    OneBelow,
    OneAboveOneBelow,
}

impl Approach {
    fn random() -> Self {
        Self::iter().choose(&mut thread_rng()).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Assignment {
    root: Root,
    chord_pitch: ChordPitch,
    chord_quality: ChordQuality,
    approach: Approach,
}

impl Assignment {
    fn random(chord_qualities: &HashSet<ChordQuality>) -> Self {
        Self {
            root: Root::random(),
            chord_pitch: ChordPitch::random(),
            chord_quality: *chord_qualities.iter().choose(&mut thread_rng()).unwrap(),
            approach: Approach::random(),
        }
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

#[function_component(App)]
pub fn app() -> Html {
    let chord_qualities = use_state_eq(|| ChordQuality::all());
    let assignment = use_state_eq(|| Assignment::random(&chord_qualities));

    let formatted = assignment.to_string();
    let on_generate = Callback::from(move |_| assignment.set(Assignment::random(&chord_qualities)));

    html! {
        <main>
            <p>{ formatted }</p>
            <button onclick={on_generate}>{ "Generate" }</button>
        </main>
    }
}
