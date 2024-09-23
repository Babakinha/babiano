mod piano;
// mod key;
mod key_audio;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    if let Some(path) = option_env!("SINGLEPAGE") {
        PerseusApp::new()
            .template(piano::get_template(path, Some(path.to_string())))

            .static_dir("")
            .index_view_str(INDEX_VIEW)
            .error_views(ErrorViews::unlocalized_development_default())
    } else {
        PerseusApp::new()
            .template(piano::get_template("index", None))

            .static_dir("./static")
            .static_alias("/favicon.ico", "./public/favicon.ico")
            .static_alias("/robots.txt", "./public/robots.txt")

            .index_view_str(INDEX_VIEW)
            .error_views(ErrorViews::unlocalized_development_default())
    }
}

// Just to add lang="en" :3
static INDEX_VIEW: &str = r#"
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    </head>
    <body>
        <div id="root"></div>
    </body>
</html>"#;
