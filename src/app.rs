use crate::model::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let assignment = use_state_eq(|| Assignment::random());

    let on_generate = {
        let assignment = assignment.clone();
        Callback::from(move |_| assignment.set(Assignment::random()))
    };

    html! {
        <main>
            <p>{ assignment.chord_string() }</p>
            <p>{ assignment.chord_pitch_string() }</p>
            <p>{ assignment.approach_string() }</p>

            <button onclick={on_generate}>{ "Generate" }</button>
        </main>
    }
}
