use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {<h1>{"Hello World"}</h1>}
}

pub fn main() {
    yew::start_app::<App>();
}
