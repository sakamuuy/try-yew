use yew::prelude::*;
use components::header::Header;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Header />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}