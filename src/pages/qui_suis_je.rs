use leptos::prelude::*;

/// Renders the "About Me" page.
#[component]
pub fn QuiSuisJePage() -> impl IntoView {
    view! {
        <div class="about-page">
            <h1>"Qui suis-je ?"</h1>
            
            <div class="about-content">
                <section class="about-section">
                    <h2>"Ma Passion pour la Création"</h2>
                    <p>"Depuis plusieurs années, j'accompagne les enfants, les familles et les adultes dans leur parcours créatif. Mon objectif est de créer des espaces bienveillants où chacun peut exprimer sa créativité, sans jugement ni comparaison."</p>
                </section>
                
                <section class="about-section">
                    <h2>"Mon Approche"</h2>
                    <p>"Je crois que la créativité est en chacun de nous. À travers mes ateliers, j'essaie de dépoussiérer ce potentiel créatif et d'offrir des outils pour que chacun puisse créer à son rythme, en fonction de ses envies et de ses capacités."</p>
                </section>
                
                <section class="about-section">
                    <h2>"Mon Parcours"</h2>
                    <p>"Formée aux arts plastiques et à l'animation, j'ai développé une expertise dans l'organisation d'ateliers inclusifs adaptés à tous les publics. Que ce soit en institution, en plein air ou en famille, je mets ma passion au service de la créativité collective."</p>
                </section>
                
                <section class="about-section">
                    <h2>"Mes Valeurs"</h2>
                    <ul>
                        <li>"Bienveillance et respect"</li>
                        <li>"Accessibilité pour tous"</li>
                        <li>"Créativité sans limite"</li>
                        <li>"Partage et convivialité"</li>
                        <li>"Développement personnel"</li>
                    </ul>
                </section>
            </div>
        </div>
    }
}
