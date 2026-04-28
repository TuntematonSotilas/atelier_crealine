use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the "Ateliers Hors les Murs" page.
#[component]
pub fn AteliersHorsLesAu() -> impl IntoView {
    view! {
        <div class="atelier-page">
            <h1>"Ateliers Hors les Murs"</h1>
            
            <div class="atelier-content">
                <section class="atelier-section">
                    <h2>"Sortez de l'Ordinaire"</h2>
                    <p>"Les ateliers hors les murs offrent une expérience créative en plein air, loin des murs traditionnels. Découvrez la nature comme source d'inspiration et créez dans un environnement nouveau et stimulant."</p>
                </section>
                
                <section class="atelier-section">
                    <h2>"Nos Activités"</h2>
                    <ul>
                        <li>"Land art et créations éphémères"</li>
                        <li>"Peinture en plein air"</li>
                        <li>"Collecte et transformation de matériaux naturels"</li>
                        <li>"Installations créatives"</li>
                        <li>"Photographie créative en extérieur"</li>
                    </ul>
                </section>
                
                <section class="atelier-section">
                    <h2>"Détails"</h2>
                    <p>"Durée : 2 à 3 heures"</p>
                    <p>"Public : Enfants et adultes"</p>
                    <p>"Saison : Idéal au printemps et en été"</p>
                </section>
                
                <div class="atelier-nav">
                    <A href="/services">"← Retour aux services"</A>
                </div>
            </div>
        </div>
    }
}
