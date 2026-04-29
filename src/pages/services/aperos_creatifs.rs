use leptos::prelude::*;

/// Renders the "Apéros Créatifs (Adultes)" page.
#[component]
pub fn AperosCreatifs() -> impl IntoView {
    view! {
        <div class="atelier-page">
            <h1>"Apéros Créatifs (Adultes)"</h1>
            
            <div class="atelier-content">
                <section class="atelier-section">
                    <h2>"Détente, Créativité et Convivialité"</h2>
                    <p>"Les apéros créatifs sont des moments privilégiés pour les adultes qui souhaitent créer dans une ambiance décontractée et conviviale. Entre discussions, rires et créativité, c'est l'occasion de rencontrer des personnes partageant les mêmes passions."</p>
                </section>
                
                <section class="atelier-section">
                    <h2>"Le Concept"</h2>
                    <ul>
                        <li>"Activités créatives simples et détendues"</li>
                        <li>"Petit verre et grignotage sur place"</li>
                        <li>"Ambiance conviviale et sans pression"</li>
                        <li>"Partage et échanges autour des créations"</li>
                        <li>"Rencontres enrichissantes avec d'autres créatifs"</li>
                    </ul>
                </section>
                
                <section class="atelier-section">
                    <h2>"Infos Pratiques"</h2>
                    <p>"Horaire : 18h30 à 20h30"</p>
                    <p>"Public : Adultes uniquement"</p>
                    <p>"Fréquence : Jeudis soirs (vérifier le calendrier)"</p>
                    <p>"Ambiance : Décontractée et accueillante"</p>
                    <p>"Participation : Tarif à définir par apéro"</p>
                </section>
                
            </div>
        </div>
    }
}
