use leptos::prelude::*;
use crate::components::ui::button::Button;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div>
            <h1>"Welcome to Leptos!"</h1>
            <h2>Count: {count}</h2>
            <Button on:click=on_click>"Button"</Button>
        </div>
    }
}