use perseus::prelude::*;
use sycamore::prelude::*;
use crate::key;

fn piano_page<G: Html>(cx: Scope) -> View<G> {
    let volume_slider = create_signal(cx, String::from("20.0"));
    let frequency_slider = create_signal(cx, String::from("220"));
    let playing = create_signal(cx, false);

    #[cfg(client)]
    let context = web_sys::AudioContext::new().expect("Unable to create AudioContext");
    #[cfg(client)]
    let master_volume = context.create_gain().expect("Unable to create gain node");
    #[cfg(client)]
    master_volume
        .connect_with_audio_node(&context.destination())
        .expect("Unable to connect master");

    #[cfg(client)]
    let oscillator = context
        .create_oscillator()
        .expect("Unable to create oscillator");
    // 'A' Note (i think)
    #[cfg(client)]
    oscillator
        .frequency()
        .set_value_at_time(220.0, 0.0)
        .expect("Unable to set tone");
    #[cfg(client)]
    oscillator
        .connect_with_audio_node(&master_volume)
        .expect("Unable to connect oscillator");

    #[cfg(client)]
    gloo::console::log!("Hello :3");

    #[cfg(client)]
    let oscillator_clone = oscillator.clone();
    let on_play = move |_| {
        if *playing.get() {
            #[cfg(client)]
            if &*volume_slider.get() == "0" {
                volume_slider.set(String::from("20"));
            } else {
                volume_slider.set(String::from("0"));
            }
            return;
        }
        #[cfg(client)]
        oscillator_clone.start().unwrap();
        playing.set(true)
    };

    #[cfg(client)]
    let context_clone = context.clone();
    create_effect(cx, move || {
        volume_slider.track();
        #[cfg(client)]
        gloo::console::log!("Vol", volume_slider.get().to_string());
        #[cfg(client)]
        master_volume
            .gain()
            .set_value_at_time(
                volume_slider.get().parse::<f32>().unwrap() / 100.0,
                context_clone.current_time(),
            )
            .unwrap();
    });

    create_effect(cx, move || {
        frequency_slider.track();
        #[cfg(client)]
        gloo::console::log!("Freq", frequency_slider.get().to_string());
        #[cfg(client)]
        oscillator
            .frequency()
            .set_value_at_time(
                frequency_slider.get().parse::<f32>().unwrap(),
                context.current_time(),
            )
            .unwrap();
    });

    view! {cx,
        button(on:click=on_play) { "Playyy :3" }
        input(type="range", min="0", max="100", bind:value=volume_slider, class="slider")
        input(type="range", min="0", max="1000", bind:value=frequency_slider, class="slider")
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
