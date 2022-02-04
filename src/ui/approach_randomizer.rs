use crate::model::approach::*;
use yew::prelude::*;

#[function_component(ApproachRandomizer)]
pub fn approach_randomizer() -> Html {
    let assignment = use_state_eq(Assignment::random);

    let on_generate = {
        let assignment = assignment.clone();
        Callback::from(move |_| assignment.set(Assignment::random()))
    };

    html! {
        <div class="container has-text-centered">
            <h1 class="title is-1">{"Approach"}</h1>
            <p class="title is-2">{ assignment.chord_string() }</p>
            <p class="subtitle">{ assignment.chord_pitch_string() }<br />{ assignment.approach_string() }</p>
            <button onclick={on_generate} class="button is-primary">{ "Generate" }</button>
        </div>
    }
}
