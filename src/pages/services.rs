use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the services page with links to different atelier types.
#[component]
pub fn ServicesPage() -> impl IntoView {
    view! {
        <div class="services-page">
            <h1>"Nos Services"</h1>
            <p>"Découvrez nos différents types d'ateliers créatifs et d'activités"</p>
            
            <div class="services-grid">
                <div class="service-card">
                    <h2>"Ateliers Hors les Murs"</h2>
                    <p>"Des ateliers créatifs en plein air et hors des structures traditionnelles"</p>
                    <A href="/services/hors-les-murs">"En savoir plus"</A>
                </div>
                
                <div class="service-card">
                    <h2>"Ateliers en Institution"</h2>
                    <p>"Ateliers adaptés pour les écoles, musées et institutions culturelles"</p>
                    <A href="/services/en-institution">"En savoir plus"</A>
                </div>
                
                <div class="service-card">
                    <h2>"Ateliers Parents-Enfants"</h2>
                    <p>"Partager des moments créatifs en famille et renforcer les liens"</p>
                    <A href="/services/parents-enfants">"En savoir plus"</A>
                </div>
                
                <div class="service-card">
                    <h2>"Ateliers Créatifs pour Tous"</h2>
                    <p>"Découvrez la créativité sans limite, accessible à tous les niveaux"</p>
                    <A href="/services/creatifs-pour-tous">"En savoir plus"</A>
                </div>
                
                <div class="service-card">
                    <h2>"Apéros Créatifs (Adultes)"</h2>
                    <p>"Des moments conviviaux mélant créativité, détente et échanges entre adultes"</p>
                    <A href="/services/aperos-creatifs">"En savoir plus"</A>
                </div>
            </div>
        </div>
    }
}
