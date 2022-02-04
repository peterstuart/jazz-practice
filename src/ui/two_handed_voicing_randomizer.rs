use crate::model::two_handed_voicing::*;
use yew::prelude::*;

#[function_component(TwoHandedVoicingRandomizer)]
pub fn two_handed_voicing_randomizer() -> Html {
    let assignment = use_state_eq(Assignment::random);

    let on_generate = {
        let assignment = assignment.clone();
        Callback::from(move |_| assignment.set(Assignment::random()))
    };

    html! {
        <div class="container has-text-centered">
            <h1 class="title is-1">{"Two-Handed Voicing"}</h1>
            <p class="title is-2">{ assignment.chord_string() }</p>
            <p class="subtitle">{ assignment.top_string() }</p>
            <button onclick={on_generate} class="button is-primary">{ "Generate" }</button>
        </div>
    }
}
