use leptos::prelude::*;
use crate::components::ui::button::Button;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="bg-white dark:bg-black dark:text-white">
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
            <Button>"Button"</Button>
        </div>
    }
}