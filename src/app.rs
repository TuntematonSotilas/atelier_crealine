use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    StaticSegment, WildcardSegment, components::{Route, Router, Routes}, path
};

use crate::{components::hooks::use_theme_mode::ThemeMode, pages::mentions_legales::MentionsLegales};
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::qui_suis_je::QuiSuisJePage;
use crate::pages::newsletter::NewsletterPage;
use crate::pages::services::hors_les_murs::AteliersHorsLesMurs;
use crate::pages::services::en_institution::AteliersEnInstitution;
use crate::pages::services::aperos_creatifs::AperosCreatifs;
use crate::pages::services::ateliers_pour_tous::AteliersCreatifsPourTous;
use crate::pages::services::parents_enfants::AteliersParentsEnfants;
use crate::components::blocks::nav_menu::NavMenu;
use crate::components::blocks::footer_block::FooterBlock;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(ThemeMode::init());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/atelier_crealine.css"/>

        // sets the document title
        <Title text="Atelier Créaline"/>

        // content for this welcome page
        <Router>
            <NavMenu/>
            <main class="container mx-auto px-4 py-8 min-h-[72vh]">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/services/hors-les-murs") view=AteliersHorsLesMurs/>
                    <Route path=path!("/services/en-institution") view=AteliersEnInstitution/>
                    <Route path=path!("/services/parents-enfants") view=AteliersParentsEnfants/>
                    <Route path=path!("/services/creatifs-pour-tous") view=AteliersCreatifsPourTous/>
                    <Route path=path!("/services/aperos-creatifs") view=AperosCreatifs/>
                    <Route path=path!("/qui-suis-je") view=QuiSuisJePage/>
                    <Route path=path!("/mentions-legales") view=MentionsLegales/>
                    <Route path=path!("/newsletter") view=NewsletterPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
            <FooterBlock/>
        </Router>
    }
}
