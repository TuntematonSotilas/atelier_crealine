use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    StaticSegment, WildcardSegment, components::{Route, Router, Routes}, path
};

use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::services::ServicesPage;
use crate::pages::qui_suis_je::QuiSuisJePage;
use crate::pages::newsletter::NewsletterPage;
use crate::pages::ateliers::{
    AteliersHorsLesAu, AteliersEnInstitution, AteliersParentsEnfants,
    AteliersCreaifsPourTous, AperosCreatifs,
};
use crate::components::ui::nav_menu::NavMenu;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/atelier_crealine.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <NavMenu/>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/services") view=ServicesPage/>
                    <Route path=path!("/services/hors-les-murs") view=AteliersHorsLesAu/>
                    <Route path=path!("/services/en-institution") view=AteliersEnInstitution/>
                    <Route path=path!("/services/parents-enfants") view=AteliersParentsEnfants/>
                    <Route path=path!("/services/creatifs-pour-tous") view=AteliersCreaifsPourTous/>
                    <Route path=path!("/services/aperos-creatifs") view=AperosCreatifs/>
                    <Route path=path!("/qui-suis-je") view=QuiSuisJePage/>
                    <Route path=path!("/newsletter") view=NewsletterPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
