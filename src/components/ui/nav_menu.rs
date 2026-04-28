use leptos::prelude::*;

/// Navigation menu component using rust-ui NavigationMenu with dropdown for services
#[component]
pub fn NavMenu() -> impl IntoView {
        
    view! {
        <div>
            <a 
                href="/services/hors-les-murs"
            >
                "Ateliers Hors les Murs"
            </a>
            <a 
                href="/services/en-institution"
            >
                "Ateliers en Institution"
            </a>
            <a 
                href="/services/parents-enfants"
            >
                "Ateliers Parents-Enfants"
            </a>
            <a 
                href="/services/creatifs-pour-tous"
            >
                "Ateliers Créatifs pour Tous"
            </a>
            <a 
                href="/services/aperos-creatifs"
            >
                "Apéros Créatifs (adultes)"
            </a>
                                
            <a 
                href="/qui-suis-je"
            >
                "Qui suis-je"
            </a>

            <a 
                href="/newsletter"
            >
                "Newsletter"
            </a>
        </div>
    }
}

