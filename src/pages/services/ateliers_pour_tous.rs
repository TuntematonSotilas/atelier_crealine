use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the "Ateliers Créatifs pour Tous" page.
#[component]
pub fn AteliersCreatifsPourTous() -> impl IntoView {
    view! {
        <div class="atelier-page">
            <h1>"Ateliers Créatifs pour Tous"</h1>
            
            <div class="atelier-content">
                <section class="atelier-section">
                    <h2>"La Créativité sans Limite"</h2>
                    <p>"Accessibles à tous les niveaux et tous les âges, ces ateliers offrent une porte d'entrée à la créativité. Aucune expérience préalable n'est requise. Venez comme vous êtes et créez sans limites!"</p>
                </section>
                
                <section class="atelier-section">
                    <h2>"Ce que Nous Proposons"</h2>
                    <ul>
                        <li>"Ateliers d'initiation aux arts plastiques"</li>
                        <li>"Exploration de différentes techniques"</li>
                        <li>"Ateliers d'expression artistique libre"</li>
                        <li>"Découverte de nouveaux matériaux"</li>
                        <li>"Projets créatifs inclusifs et accessibles"</li>
                    </ul>
                </section>
                
                <section class="atelier-section">
                    <h2>"Pour Qui ?"</h2>
                    <p>"Enfants, adolescents, adultes, personnes en quête de créativité, personnes en situation de handicap... Vraiment pour TOUS!"</p>
                    <p>"Durée : 2 heures"</p>
                    <p>"Calendrier : Toute l'année"</p>
                </section>
                
                <div class="atelier-nav">
                    <A href="/services">"← Retour aux services"</A>
                </div>
            </div>
        </div>
    }
}
