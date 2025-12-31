use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::*;
use crate::piano::BabianoPage;
use leptos_router::static_routes::StaticRoute;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // <AutoReload options=options.clone()/>
                <HydrationScripts options islands=false/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found ><" }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/babiano.css"/>
        <Stylesheet id="baba" href="/css/baba.css"/>
        <Title text="~UwU What is this?~"/>
        <Meta name="color-scheme" content="dark light"/>
        <Router>
            <main>
                <FlatRoutes fallback>
                    <Route
                        path=path!("/")
                        view=BabianoPage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />

                </FlatRoutes>
            </main>
        </Router>
    }
}
