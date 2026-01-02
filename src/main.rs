#[cfg(feature = "export")]
#[tokio::main]
async fn main() {
    use babiano::app::*;
    use leptos::prelude::*;
    use leptos_axum::generate_route_list_with_ssg;

    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;

    let (_, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;
}


#[cfg(not(feature = "export"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
