use leptos::prelude::*;

/// Renders the newsletter subscription page.
#[component]
pub fn NewsletterPage() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let submitted = RwSignal::new(false);
    
    view! {
        <div>
            <h1>"Notre Newsletter"</h1>
            
            <div class="newsletter-content">
                <section class="newsletter-section">
                    <h2>"Restez Informé"</h2>
                    <p>"Inscrivez-vous à notre newsletter pour recevoir :"</p>
                    <ul>
                        <li>"Les dates et horaires de nos nouveaux ateliers"</li>
                        <li>"Des conseils créatifs et des astuces DIY"</li>
                        <li>"Les actualités et les projets spéciaux"</li>
                        <li>"Des offres et promotions exclusives"</li>
                    </ul>
                </section>
                
                <section class="newsletter-form-section">
                    <h2>"S'inscrire"</h2>
                    <Show when=move || submitted.get()>
                        <div class="success-message">
                            <p>"Merci ! Votre inscription a bien été reçue."</p>
                        </div>
                    </Show>
                    <Show when=move || !submitted.get()>
                        <form on:submit=move |e: leptos::ev::SubmitEvent| {
                            e.prevent_default();
                            if !email.get().is_empty() {
                                *submitted.write() = true;
                                *email.write() = String::new();
                                
                                // Reset message after 5 seconds
                                let _ = set_timeout(
                                    move || {
                                        *submitted.write() = false;
                                    },
                                    std::time::Duration::from_secs(5)
                                );
                            }
                        }>
                            <div class="form-group">
                                <label for="email">"Votre adresse e-mail"</label>
                                <input
                                    id="email"
                                    type="email"
                                    placeholder="vous@example.com"
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        *email.write() = val;
                                    }
                                    prop:value=email
                                    required
                                />
                            </div>
                            <button type="submit" class="btn-submit">
                                "S'inscrire"
                            </button>
                        </form>
                    </Show>
                </section>
                
                <section class="newsletter-section">
                    <h2>"Votre Confidentialité"</h2>
                    <p>"Nous respectons votre vie privée. Vos données ne seront jamais partagées à des tiers. Vous pouvez vous désabonner à tout moment."</p>
                </section>
            </div>
        </div>
    }
}
