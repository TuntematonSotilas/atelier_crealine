use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-hero border border-(--border) rounded-[2rem] shadow-(--shadow) max-w-4xl mx-auto">
            <h3 class="home-hero-title">"Atelier Créaline"</h3>
            <h1 class="home-hero-subtitle">"Médiation artistique en relation d'aide."</h1>
            <h2 class="home-hero-words">
                <span class="home-hero-word">"Créer"</span>
                <span class="home-hero-word">" 🎨 "</span>
                <span class="home-hero-word">"Se ressourcer"</span>
                <span class="home-hero-word">" 💡 "</span>
                <span class="home-hero-word">"Partager"</span>
            </h2>
        </div>
    }
}