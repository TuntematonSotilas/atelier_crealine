use leptos::prelude::*;

/// Renders the "Ateliers Parents-Enfants" page.
#[component]
pub fn AteliersParentsEnfants() -> impl IntoView {
    view! {
        <div class="atelier-page">
            <h1>"Ateliers Parents-Enfants"</h1>
            
            <div class="atelier-content">
                <section class="atelier-section">
                    <h2>"Partager des Moments Créatifs en Famille"</h2>
                    <p>"Ces ateliers sont conçus pour renforcer les liens parent-enfant à travers des activités créatives ludiques et enrichissantes. Créer ensemble devient une source de complicité et de souvenirs partagés."</p>
                </section>
                
                <section class="atelier-section">
                    <h2>"Activités Proposées"</h2>
                    <ul>
                        <li>"Création de projets artistiques ensemble"</li>
                        <li>"Ateliers de peinture et dessin"</li>
                        <li>"Travaux manuels et bricolage créatif"</li>
                        <li>"Création de jeux et jouets DIY"</li>
                        <li>"Ateliers de collage et assemblage"</li>
                    </ul>
                </section>
                
                <section class="atelier-section">
                    <h2>"Infos Pratiques"</h2>
                    <p>"Durée : 2 heures"</p>
                    <p>"Public : Parents et enfants (à partir de 4 ans)"</p>
                    <p>"Fréquence : Samedis et mercredis"</p>
                    <p>"Nombre limité de places pour une ambiance conviviale"</p>
                </section>
                
            </div>
        </div>
    }
}
