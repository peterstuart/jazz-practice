use super::{
    approach_randomizer::ApproachRandomizer,
    two_handed_voicing_randomizer::TwoHandedVoicingRandomizer,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <section class="section is-flex is-vcentered">
                <ApproachRandomizer />
            </section>
            <section class="section is-flex is-vcentered">
                <TwoHandedVoicingRandomizer />
            </section>
        </>
    }
}
