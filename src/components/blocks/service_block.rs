use icons::MapPin;
use leptos::prelude::*;

use crate::components::ui::button::Button;

pub struct Session {
    pub date: String,
    pub theme: String,
    pub price: String,
}

#[component]
pub fn ServiceBlock(
    #[prop(into, optional, default = "Title".to_string())] title: String,
    #[prop(into, optional, default = "Description".to_string())] description: String,
    #[prop(into, optional, default = "".to_string())] image: String,
    #[prop(into, optional, default = true)] is_register: bool,
    #[prop(into, optional, default = "".to_string())] schedule: String,
    #[prop(into, optional, default = "".to_string())] place: String,
    #[prop(into, optional, default = "".to_string())] age: String,
    #[prop(into, optional, default = "".to_string())] place_link: String,
    #[prop(optional, default = Vec::new())] steps: Vec<String>,
    #[prop(optional, default = Vec::new())] sessions: Vec<Session>,
) -> impl IntoView {

    let btn_txt = match is_register {
        true => "S'inscrire",
        false => "En savoir plus",
    };

    let steps_view = steps
        .into_iter()
        .enumerate()
        .map(|(idx, step)| view! { 
            <div class="flex gap-3 items-start">
                <div class="flex-shrink-0 w-6 h-6 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center text-white text-sm font-semibold">
                    {idx + 1}
                </div>
                <p class="text-sm text-[var(--gray-text)] pt-0.5">{step}</p>
            </div>
        }.into_any())
        .collect::<Vec<_>>();

    let sessions_view = sessions
        .into_iter()
        .map(|session| view! { 
            <div class="p-4 border-l-4 border-primary bg-gradient-to-r from-primary/10 dark:from-primary/20 to-transparent rounded-r-lg hover:shadow-md dark:hover:shadow-primary/20 transition-shadow">
                <div class="font-semibold text-gray-900 dark:text-white">{session.date}</div>
                <div class="text-sm text-[var(--gray-text)] mt-1">{"Thème : "}<span class="text-primary dark:text-secondary">{session.theme}</span></div>
                <div class="text-sm font-medium text-gray-900 dark:text-white mt-2">{session.price}{"€"}</div>
            </div> }.into_any())
        .collect::<Vec<_>>();

    view! {
        <article class="rounded-xl shadow-lg shadow-slate-500/40 bg-[var(--card)] text-[var(--card-foreground)] overflow-hidden">
            <div class="flex flex-col lg:flex-row gap-0">

                {/* Image Section */}
                <div class="lg:w-80 flex-shrink-0 bg-[var(--primary)]" 
                    style={format!("background-repeat: repeat; background-size: 5em; background-image: url('{}');", image)}>
                </div>

                {/* Content Section */}
                <div class="flex-1 p-6 lg:p-8 flex flex-col">
                    {/* Header */}
                    <div class="mb-6">
                        <h3 class="text-2xl lg:text-3xl font-bold mb-3">{title.clone()}</h3>
                        <p class="text-base leading-relaxed">{description}</p>
                    </div>

                    {/* Info Cards */}
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                        <div class="p-4 bg-[var(--primary)] rounded-lg border border-slate-200 dark:border-slate-700">
                            <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">Horaire</div>
                            <div class="text-slate-900 dark:text-white font-medium mt-1">{schedule}</div>
                        </div>
                        <div class="p-4 bg-[var(--primary)] rounded-lg border border-slate-200 dark:border-slate-700">
                            <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">Âge requis</div>
                            <div class="text-slate-900 dark:text-white font-medium mt-1">{age}</div>
                        </div>
                        <div class="p-4 bg-[var(--primary)] rounded-lg border border-slate-200 dark:border-slate-700 md:col-span-2">
                            <div class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wide">Lieu</div>
                            <a href={place_link} target="_blank" rel="noreferrer" class="text-slate-900 dark:text-white font-medium mt-1 inline-flex items-center gap-2 hover:text-primary dark:hover:text-secondary transition-colors">
                                <MapPin class="w-4 h-4 flex-shrink-0" />
                                <span class="underline">{place}</span>
                            </a>
                        </div>
                    </div>

                    {/* Steps */}
                    <div class="mb-6 pb-6 border-b border-gray-200 dark:border-slate-700">
                        <h4 class="text-lg font-bold mb-4">{"Déroulement de l'atelier"}</h4>
                        <div class="space-y-3">
                            {steps_view}
                        </div>
                    </div>
            
                    {/* Sessions */}
                    <div class="mb-6">
                        <h4 class="text-lg font-bold mb-4">{"Prochaines séances"}</h4>
                        <div class="space-y-3">
                            {sessions_view}
                        </div>
                    </div>
                        
                    {/* Register Button */}
                    <div class="mt-auto">
                        <Button class="w-full md:w-auto">
                            {btn_txt}
                        </Button>
                    </div>
                </div>
            </div>
        </article>
    }.into_any()
}