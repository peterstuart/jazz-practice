use crate::model::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let assignment = use_state_eq(Assignment::random);

    let on_generate = {
        let assignment = assignment.clone();
        Callback::from(move |_| assignment.set(Assignment::random()))
    };

    html! {
        <section class="section is-flex is-vcentered">
            <div class="container has-text-centered">
                <h1 class="title">{ assignment.chord_string() }</h1>
                <p class="subtitle">{ assignment.chord_pitch_string() }<br />{ assignment.approach_string() }</p>
                <button onclick={on_generate} class="button is-primary">{ "Generate" }</button>
            </div>
        </section>
    }
}
