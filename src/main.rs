mod piano;
mod key;
mod key_audio;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(piano::get_template("index", None))

        .error_views(ErrorViews::unlocalized_development_default())
        //TODO: Better localization
        .index_view_str(INDEX_VIEW)
        .static_dir("./static")
        // Public stuff
        //TODO: Automate this?
        .static_alias("/favicon.ico", "./public/favicon.ico")
        .static_alias("/robots.txt", "./public/robots.txt")
        // Github Pages stuff
        // Idk if this is needed for gh pages to work but... why not keep it
        .static_alias("/CNAME", "./CNAME")
        .static_alias("/.nojekyll", "./.nojekyll")
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
