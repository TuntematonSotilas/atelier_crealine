use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {Footer, footer, ""}
    clx! {FooterBrandLink, a, "block size-fit"}
    clx! {FooterLinksSection, div, "space-y-4 text-sm"}
    clx! {FooterTitle, span, "block font-medium"}
    clx! {FooterLink, a, "block duration-150 text-foreground/70 hover:text-primary"}
    clx! {FooterLinks, div, "flex flex-wrap gap-4 sm:flex-col"}
    clx! {FooterDescription, p, "text-sm text-foreground/70 text-balance"}
    clx! {FooterGrid, div, "grid gap-12 md:grid-cols-5"}
    clx! {FooterContainer, div, "px-6 mx-auto max-w-5xl"}
    clx! {FooterSection, section,
        "w-full max-w-5xl mx-auto py-6 flex flex-wrap gap-4 justify-between items-center",
        "[.border-b]:mb-14",
        "[.border-t]:mt-14"
    }
    clx! {FooterSocialContainer, div, "flex flex-wrap gap-6 text-sm"}
    clx! {FooterBrand, div, "md:col-span-2"}
    clx! {FooterSectionsGrid, div, "grid gap-6"}
    clx! {FooterCopyright, small, "text-sm text-foreground/70"}
    clx! {FooterNavContainer, div, "flex flex-wrap gap-6 justify-center my-4 text-sm"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FooterExternalLink(
    children: Children,
    href: &'static str,
    #[prop(into, optional, default = "Social media link".to_string())] aria_label: String,
) -> impl IntoView {
    view! {
        <a
            target="_blank"
            rel="noopener noreferrer"
            class="block text-foreground/70 hover:text-primary"
            href=href
            aria-label=aria_label
        >
            {children()}
        </a>
    }
}