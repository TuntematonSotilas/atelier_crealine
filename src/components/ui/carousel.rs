use icons::{ChevronLeft, ChevronRight};
use leptos::context::Provider;
use leptos::prelude::*;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id_for;

/* ========================================================== */
/*                     CAROUSEL CONTEXT                       */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct CarouselContext {
    carousel_id: String,
    orientation: CarouselOrientation,
}

impl CarouselContext {
    #[must_use]
    /// Initializes a new CarouselContext instance.
    pub fn init() -> Self {
        let carousel_id = use_random_id_for("carousel");
        let orientation = CarouselOrientation::Horizontal;
        CarouselContext { carousel_id, orientation }
    }
}

/* ========================================================== */
/*                     CAROUSEL ROOT                          */
/* ========================================================== */

#[component]
pub fn Carousel(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: CarouselOrientation,
    #[prop(optional)] looping: bool,
) -> impl IntoView {
    let carousel_id = use_random_id_for("carousel");
    let ctx = CarouselContext { carousel_id: carousel_id.clone(), orientation };

    let orientation_str = match orientation {
        CarouselOrientation::Horizontal => "horizontal",
        CarouselOrientation::Vertical => "vertical",
    };

    let class = tw_merge!("relative", class);

    view! {
        <Provider value=ctx>

            <div
                data-name="Carousel"
                data-carousel-id=carousel_id.clone()
                data-carousel-orientation=orientation_str
                data-carousel-loop=looping.to_string()
                class=class
                role="region"
                aria-roledescription="carousel"
                tabindex="0"
            >
                {children()}
            </div>

            <script>
                {format!(
                    r#"
                    (function() {{
                        const setup = () => {{
                            const root = document.querySelector('[data-carousel-id="{0}"]');
                            const trackEl = root && root.querySelector('[data-carousel-track="{0}"]');
                            const prevBtn = root && root.querySelector('[data-carousel-prev="{0}"]');
                            const nextBtn = root && root.querySelector('[data-carousel-next="{0}"]');

                            if (!root || !trackEl) {{ setTimeout(setup, 50); return; }}
                            if (root.hasAttribute('data-carousel-initialized')) return;
                            root.setAttribute('data-carousel-initialized', 'true');

                            const isHorizontal = root.getAttribute('data-carousel-orientation') !== 'vertical';
                            const isLoop = root.getAttribute('data-carousel-loop') === 'true';

                            const getTrackPos  = () => isHorizontal ? trackEl.scrollLeft : trackEl.scrollTop;
                            const getTrackSize = () => isHorizontal ? trackEl.scrollWidth : trackEl.scrollHeight;
                            const getClientSize = () => isHorizontal ? trackEl.clientWidth  : trackEl.clientHeight;
                            const getMaxTrackPos = () => Math.max(0, getTrackSize() - getClientSize());

                            const slides = () => Array.from(trackEl.querySelectorAll('[role="group"]'));
                            const countSlides = () => slides().length;
                            const currentSlide = () => {{
                                const currentSlides = slides();
                                if (currentSlides.length === 0) return 1;
                                const pos = getTrackPos();
                                const viewportCenter = pos + getClientSize() / 2;
                                let closest = 0;
                                let minDiff = Infinity;
                                currentSlides.forEach((slide, index) => {{
                                    const offset = isHorizontal ? slide.offsetLeft : slide.offsetTop;
                                    const size = isHorizontal ? slide.offsetWidth : slide.offsetHeight;
                                    const center = offset + size / 2;
                                    const diff = Math.abs(viewportCenter - center);
                                    if (diff < minDiff) {{
                                        minDiff = diff;
                                        closest = index;
                                    }}
                                }});
                                return Math.min(closest + 1, currentSlides.length);
                            }};

                            const canPrev = () => currentSlide() > 1;
                            const canNext = () => currentSlide() < countSlides();

                            const updateIndicator = () => {{
                                const indicator = root.querySelector('[data-carousel-indicator="{0}"]');
                                if (!indicator) return;
                                indicator.textContent = `${{currentSlide()}} / ${{countSlides()}}`;
                            }};

                            const updateButtons = () => {{
                                if (prevBtn) prevBtn.disabled = !isLoop && !canPrev();
                                if (nextBtn) nextBtn.disabled = !isLoop && !canNext();
                                updateIndicator();
                            }};

                            const rafUpdateButtons = () => {{
                                window.requestAnimationFrame(updateButtons);
                            }};

                            const trackPrev = () => {{
                                if (isLoop && !canPrev()) {{
                                    if (isHorizontal) trackEl.scrollLeft = trackEl.scrollWidth;
                                    else trackEl.scrollTop = trackEl.scrollHeight;
                                    updateButtons();
                                }} else {{
                                    const size = getClientSize();
                                    if (isHorizontal) trackEl.scrollBy({{ left: -size, behavior: 'smooth' }});
                                    else trackEl.scrollBy({{ top: -size, behavior: 'smooth' }});
                                }}
                            }};

                            const trackNext = () => {{
                                if (isLoop && !canNext()) {{
                                    if (isHorizontal) trackEl.scrollLeft = 0;
                                    else trackEl.scrollTop = 0;
                                    updateButtons();
                                }} else {{
                                    const size = getClientSize();
                                    if (isHorizontal) trackEl.scrollBy({{ left: size, behavior: 'smooth' }});
                                    else trackEl.scrollBy({{ top: size, behavior: 'smooth' }});
                                }}
                            }};

                            if (prevBtn) prevBtn.addEventListener('click', trackPrev);
                            if (nextBtn) nextBtn.addEventListener('click', trackNext);
                            trackEl.addEventListener('scroll', rafUpdateButtons);
                            window.addEventListener('resize', rafUpdateButtons);

                            root.addEventListener('keydown', (e) => {{
                                if (isHorizontal) {{
                                    if (e.key === 'ArrowLeft')  {{ e.preventDefault(); trackPrev(); }}
                                    else if (e.key === 'ArrowRight') {{ e.preventDefault(); trackNext(); }}
                                }} else {{
                                    if (e.key === 'ArrowUp')   {{ e.preventDefault(); trackPrev(); }}
                                    else if (e.key === 'ArrowDown') {{ e.preventDefault(); trackNext(); }}
                                }}
                            }});

                            updateButtons();
                        }};

                        if (document.readyState === 'loading') {{
                            document.addEventListener('DOMContentLoaded', setup);
                        }} else {{
                            setup();
                        }}
                    }})();
                    "#,
                    carousel_id,
                )}
            </script>
        </Provider>
    }
}

/* ========================================================== */
/*                     CAROUSEL CONTENT                       */
/* ========================================================== */

#[component]
pub fn CarouselContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();
    let carousel_id = ctx.carousel_id;

    let (scroll_class, inner_class) = match ctx.orientation {
        CarouselOrientation::Horizontal => {
            ("overflow-x-auto snap-x snap-mandatory scroll-smooth", tw_merge!("flex -ml-4", class))
        }
        CarouselOrientation::Vertical => {
            ("overflow-y-auto snap-y snap-mandatory scroll-smooth", tw_merge!("flex flex-col -mt-4", class))
        }
    };

    view! {
        <div
            data-carousel-track=carousel_id
            class=scroll_class
            style="scrollbar-width: none; -ms-overflow-style: none;"
        >
            <div class=inner_class>{children()}</div>
        </div>
    }
}

/* ========================================================== */
/*                     CAROUSEL ITEM                          */
/* ========================================================== */

#[component]
pub fn CarouselItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let padding = match ctx.orientation {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };

    let class = tw_merge!("min-w-0 shrink-0 grow-0 basis-full snap-start", padding, class);

    view! {
        <div data-name="CarouselItem" role="group" aria-roledescription="slide" class=class>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     CAROUSEL PREVIOUS                      */
/* ========================================================== */

#[component]
pub fn CarouselPrevious(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -left-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-top-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class
    );

    view! {
        <button data-name="CarouselPrevious" data-carousel-prev=ctx.carousel_id class=class aria-label="Previous slide">
            <ChevronLeft class="size-4 no-tooltips" />
            <span class="sr-only">"Previous slide"</span>
        </button>
    }
}

/* ========================================================== */
/*                     CAROUSEL NEXT                          */
/* ========================================================== */

#[component]
pub fn CarouselNext(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -right-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-bottom-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class
    );

    view! {
        <button data-name="CarouselNext" data-carousel-next=ctx.carousel_id class=class aria-label="Next slide">
            <ChevronRight class="size-4 no-tooltips" />
            <span class="sr-only">"Next slide"</span>
        </button>
    }
}

/* ========================================================== */
/*                     CAROUSEL INDICATOR                     */
/* ========================================================== */

/// Displays "current / total" slide count, updated automatically by JS.
#[component]
pub fn CarouselIndicator(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();
    let class = tw_merge!("py-2 text-center text-sm text-muted-foreground", class);

    view! { <div data-name="CarouselIndicator" data-carousel-indicator=ctx.carousel_id class=class /> }
}