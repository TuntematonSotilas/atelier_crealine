use leptos::prelude::*;
use crate::components::ui::navigation_menu::*;


#[component]
fn ListItem(#[prop(into)] href: String, #[prop(into)] title: String) -> impl IntoView {
    view! {
        <li>
            <a href=href class="block p-3 space-y-1 leading-none no-underline rounded-md transition-colors outline-none select-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground">
                <div class="text-sm font-medium leading-none">{title}</div>
            </a>
        </li>
    }
}

#[component]
pub fn NavMenu() -> impl IntoView {
        
    view! {
        <div class="flex justify-center items-start py-8 min-h-[350px]">
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <NavigationMenuTrigger>"Services"</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <ul class="grid gap-3 p-0 md:grid-cols-2 w-[400px] md:w-[500px] lg:w-[600px]">
                                <ListItem href="/services/hors-les-murs" title="Ateliers hors les murs"/>
                                <ListItem href="/services/en-institution" title="Ateliers en institution"/>
                                <ListItem href="/services/parents-enfants" title="Ateliers parents-enfants"/>
                                <ListItem href="/services/creatifs-pour-tous" title="Ateliers créatifs pour tous"/>
                                <ListItem href="/services/aperos-creatifs" title="Apéros Créatifs (adultes)"/>
                            </ul>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink class=navigation_menu_trigger_style() href="/qui-suis-je">
                            "Qui suis-je"
                        </NavigationMenuLink>
                    </NavigationMenuItem>

                     <NavigationMenuItem>
                        <NavigationMenuLink class=navigation_menu_trigger_style() href="/newsletter">
                            "Newsletter"
                        </NavigationMenuLink>
                    </NavigationMenuItem>

                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}

