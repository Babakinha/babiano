use crate::key::*;
use perseus::prelude::*;
use sycamore::prelude::*;

fn piano_page<G: Html>(cx: Scope) -> View<G> {
    let volume_slider = create_signal(cx, String::from("10"));
    let volume_normal = create_memo(cx, || {
        (*volume_slider.get()).parse::<f32>().unwrap() / 100.0
    });

    let on_play = move |_| {
        #[cfg(client)]
        if &*volume_slider.get() == "0" {
            volume_slider.set(String::from("10"));
        } else {
            volume_slider.set(String::from("0"));
        }
    };

    view! {cx,
        button(on:click=on_play) { "Mute ><" }
        input(type="range", min="0", max="100", bind:value=volume_slider, class="slider")
        div(style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;") {
            BabianoKey(frequency=261.63, volume=volume_normal) {"C"}
            BabianoKey(frequency=293.66, volume=volume_normal) {"D"}
            BabianoKey(frequency=329.63, volume=volume_normal) {"E"}
            BabianoKey(frequency=349.23, volume=volume_normal) {"F"}
            BabianoKey(frequency=392.00, volume=volume_normal) {"G"}
            BabianoKey(frequency=440.00, volume=volume_normal) {"A"}
            BabianoKey(frequency=493.88, volume=volume_normal) {"B"}
        }
        p { "I love you ❤️" }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "~UwU What is this~" }

        meta(name="description", content="Cool piano -w-")

        link(rel="preconnect", href="https://fonts.googleapis.com")
        link(rel="preconnect", href="https://fonts.gstatic.com", crossorigin=true)
        link(rel="preload", as="style", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        link(rel="stylesheet", media="print", onload="this.media='all'", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        noscript {
            link(rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap")
        }

        //TODO: maybe change the .perseus thing somehow
        link(href="/.perseus/static/css/baba.css", rel="stylesheet")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(piano_page).head(head).build()
}
