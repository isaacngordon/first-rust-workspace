use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let browser_counter = use_state(|| 0);
    let onclick = {
        let browser_counter = browser_counter.clone();
        move |_| {
            let value = *browser_counter + 1;
            browser_counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1 pls" }</button>
            <p>{ *browser_counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}