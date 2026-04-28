use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the "Ateliers en Institution" page.
#[component]
pub fn AteliersEnInstitution() -> impl IntoView {
    view! {
        <div class="atelier-page">
            <h1>"Ateliers en Institution"</h1>
            
            <div class="atelier-content">
                <section class="atelier-section">
                    <h2>"Créativité dans les Structures"</h2>
                    <p>"Nous proposons des ateliers adaptés aux écoles, musées, bibliothèques et autres institutions culturelles. Nos programmes sont personnalisés selon les besoins et les publics de chaque structure."</p>
                </section>
                
                <section class="atelier-section">
                    <h2>"Nos Propositions"</h2>
                    <ul>
                        <li>"Ateliers périscolaires"</li>
                        <li>"Projets artistiques avec les classes"</li>
                        <li>"Formations pour les personnels éducatifs"</li>
                        <li>"Événements culturels sur mesure"</li>
                        <li>"Activités pour les groupes spécialisés"</li>
                    </ul>
                </section>
                
                <section class="atelier-section">
                    <h2>"Conditions"</h2>
                    <p>"Durée adaptable : de 1h à 2h selon vos besoins"</p>
                    <p>"Public : Groupes scolaires, groupes d'âge mixte"</p>
                    <p>"Calendrier : Toute l'année sur rendez-vous"</p>
                    <p>"Lieu : Au sein de votre institution"</p>
                </section>
                
                <div class="atelier-nav">
                    <A href="/services">"← Retour aux services"</A>
                </div>
            </div>
        </div>
    }
}
