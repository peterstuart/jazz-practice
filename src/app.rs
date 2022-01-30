use crate::model::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let assignment = use_state_eq(|| Assignment::random());

    let formatted = assignment.to_string();
    let on_generate = Callback::from(move |_| assignment.set(Assignment::random()));

    html! {
        <main>
            <p>{ formatted }</p>
            <button onclick={on_generate}>{ "Generate" }</button>
        </main>
    }
}
