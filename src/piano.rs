use crate::key::*;
use crate::key_audio::*;
use perseus::prelude::*;
use sycamore::prelude::*;

fn piano_page<G: Html>(cx: Scope) -> View<G> {
    let volume_slider = create_signal(cx, String::from("10"));
    let tune_input = create_signal(cx, String::from("10"));
    let tune = create_memo(cx, || (*tune_input.get()).parse::<f32>().unwrap_or(0.0));
    let sample = create_signal(cx, None);

    let on_change = move |event: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        let binding = event.target().unwrap();
        #[cfg(client)]
        let input_node = binding.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        #[cfg(client)]
        if let Some(file) = input_node.files().unwrap().get(0) {
            gloo::console::log!("File!", file.name());
            sample.set(Some(file));
        }
    };

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
        input(type="range", min="-1000", step="100", max="1000", bind:value=tune_input, class="slider")
        input(on:change=on_change, type="file")
        // div(style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;") {
        //     BabianoKey(frequency=261.63, volume=volume_normal, keyboard_key=String::from("a")) {"C"}
        //     BabianoKey(frequency=277.18, volume=volume_normal, keyboard_key=String::from("w")) {"C#"}
        //     BabianoKey(frequency=293.66, volume=volume_normal, keyboard_key=String::from("s")) {"D"}
        //     BabianoKey(frequency=311.13, volume=volume_normal, keyboard_key=String::from("e")) {"D#"}
        //     BabianoKey(frequency=329.63, volume=volume_normal, keyboard_key=String::from("d")) {"E"}
        //     BabianoKey(frequency=349.23, volume=volume_normal, keyboard_key=String::from("f")) {"F"}
        //     BabianoKey(frequency=369.99, volume=volume_normal, keyboard_key=String::from("t")) {"F#"}
        //     BabianoKey(frequency=392.00, volume=volume_normal, keyboard_key=String::from("g")) {"G"}
        //     BabianoKey(frequency=415.30, volume=volume_normal, keyboard_key=String::from("y")) {"G#"}
        //     BabianoKey(frequency=440.00, volume=volume_normal, keyboard_key=String::from("h")) {"A"}
        //     BabianoKey(frequency=466.16, volume=volume_normal, keyboard_key=String::from("u")) {"A#"}
        //     BabianoKey(frequency=493.88, volume=volume_normal, keyboard_key=String::from("j")) {"B"}
        //     BabianoKey(frequency=523.25, volume=volume_normal, keyboard_key=String::from("k")) {"C"}
        //     BabianoKey(frequency=587.33, volume=volume_normal, keyboard_key=String::from("l")) {"D"}
        // }
        div(style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;") {
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 100.0), volume=volume_normal, keyboard_key=String::from("a")) {"C"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 200.0), volume=volume_normal, keyboard_key=String::from("w")) {"C#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 300.0), volume=volume_normal, keyboard_key=String::from("s")) {"D"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 400.0), volume=volume_normal, keyboard_key=String::from("e")) {"D#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 500.0), volume=volume_normal, keyboard_key=String::from("d")) {"E"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 600.0), volume=volume_normal, keyboard_key=String::from("f")) {"F"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 700.0), volume=volume_normal, keyboard_key=String::from("t")) {"F#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 800.0), volume=volume_normal, keyboard_key=String::from("g")) {"G"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 900.0), volume=volume_normal, keyboard_key=String::from("y")) {"G#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1000.0), volume=volume_normal, keyboard_key=String::from("h")) {"A"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1100.0), volume=volume_normal, keyboard_key=String::from("u")) {"A#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1200.0), volume=volume_normal, keyboard_key=String::from("j")) {"B"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1300.0), volume=volume_normal, keyboard_key=String::from("k")) {"C"}
            BabianoKeyAudio(class="black-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1400.0), volume=volume_normal, keyboard_key=String::from("i")) {"C#"}
            BabianoKeyAudio(class="white-note".to_string(), file=sample, detune=create_memo(cx, || *tune.get() + 1500.0), volume=volume_normal, keyboard_key=String::from("l")) {"D"}
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
