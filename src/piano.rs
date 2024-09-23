use crate::key::*;
use crate::key_audio::*;
use perseus::prelude::*;
use sycamore::prelude::*;

fn piano_page<G: Html>(cx: Scope, static_path: &str) -> View<G> {
    let volume_slider = create_signal(cx, String::from("10"));
    let volume_normal = create_memo(cx, || {
        (*volume_slider.get()).parse::<f32>().unwrap() / 100.0
    });

    let tune_input = create_signal(cx, String::from("10"));
    let tune = create_memo(cx, || (*tune_input.get()).parse::<f32>().unwrap_or(0.0));

    let sample = create_signal(cx, None);

    // When a file is selected
    let on_change = move |event: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        let binding = event.target().unwrap();
        #[cfg(client)]
        let input_node = binding.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        #[cfg(client)]
        if let Some(file) = input_node.files().unwrap().get(0) {
            sample.set(Some(file));
        }
    };

    // Mute button actually
    let on_mute = move |_| {
        #[cfg(client)]
        if &*volume_slider.get() == "0" {
            volume_slider.set(String::from("10"));
        } else {
            volume_slider.set(String::from("0"));
        }
    };

    view! {cx,
        button(on:click=on_mute) { "Mute ><" }

        label(for="volume") { "Volume:" }
        input(type="range", id="volume", min="0", max="100", bind:value=volume_slider, class="slider")
        label(for="tune") { "Tune:" }
        input(type="range", id="tune", min="-1000", step="100", max="1000", bind:value=tune_input, class="slider")
        label(for="sample") { "Sample:" }
        input(type="file", id="sample", on:change=on_change)
        /*
        div(style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;") {
            BabianoKey(frequency=261.63, volume=volume_normal, keyboard_key=String::from("a")) {"C"}
            BabianoKey(frequency=277.18, volume=volume_normal, keyboard_key=String::from("w")) {"C#"}
            BabianoKey(frequency=293.66, volume=volume_normal, keyboard_key=String::from("s")) {"D"}
            BabianoKey(frequency=311.13, volume=volume_normal, keyboard_key=String::from("e")) {"D#"}
            BabianoKey(frequency=329.63, volume=volume_normal, keyboard_key=String::from("d")) {"E"}
            BabianoKey(frequency=349.23, volume=volume_normal, keyboard_key=String::from("f")) {"F"}
            BabianoKey(frequency=369.99, volume=volume_normal, keyboard_key=String::from("t")) {"F#"}
            BabianoKey(frequency=392.00, volume=volume_normal, keyboard_key=String::from("g")) {"G"}
            BabianoKey(frequency=415.30, volume=volume_normal, keyboard_key=String::from("y")) {"G#"}
            BabianoKey(frequency=440.00, volume=volume_normal, keyboard_key=String::from("h")) {"A"}
            BabianoKey(frequency=466.16, volume=volume_normal, keyboard_key=String::from("u")) {"A#"}
            BabianoKey(frequency=493.88, volume=volume_normal, keyboard_key=String::from("j")) {"B"}
            BabianoKey(frequency=523.25, volume=volume_normal, keyboard_key=String::from("k")) {"C"}
            BabianoKey(frequency=587.33, volume=volume_normal, keyboard_key=String::from("l")) {"D"}
        }
        */
        div(style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;") {
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 100.0), volume=volume_normal, keyboard_key=String::from("a")) {"C"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 200.0), volume=volume_normal, keyboard_key=String::from("w")) {"C#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 300.0), volume=volume_normal, keyboard_key=String::from("s")) {"D"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 400.0), volume=volume_normal, keyboard_key=String::from("e")) {"D#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 500.0), volume=volume_normal, keyboard_key=String::from("d")) {"E"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 600.0), volume=volume_normal, keyboard_key=String::from("f")) {"F"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 700.0), volume=volume_normal, keyboard_key=String::from("t")) {"F#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 800.0), volume=volume_normal, keyboard_key=String::from("g")) {"G"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 900.0), volume=volume_normal, keyboard_key=String::from("y")) {"G#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1000.0), volume=volume_normal, keyboard_key=String::from("h")) {"A"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1100.0), volume=volume_normal, keyboard_key=String::from("u")) {"A#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1200.0), volume=volume_normal, keyboard_key=String::from("j")) {"B"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1300.0), volume=volume_normal, keyboard_key=String::from("k")) {"C"}
            BabianoKeyAudio(class="black-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1400.0), volume=volume_normal, keyboard_key=String::from("i")) {"C#"}
            BabianoKeyAudio(class="white-note".to_string(), fallback_href=format!("{static_path}/meow.wav"), file=sample, detune=create_memo(cx, || *tune.get() + 1500.0), volume=volume_normal, keyboard_key=String::from("l")) {"D"}
        }
        p { "I love you ❤️" }
    }
}

#[engine_only_fn]
fn head(cx: Scope, static_path: String) -> View<SsrNode> {
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

        link(href=format!("{static_path}/css/baba.css"), rel="stylesheet")
    }
}

pub fn get_template<G: Html>(page_path: &str, custom_static: Option<String>) -> Template<G> {
    let static_path = custom_static.unwrap_or_else(|| String::from("/.perseus/static"));

    let template = Template::build(page_path).view({
        let static_path = static_path.clone();
        move |cx| piano_page(cx, &static_path)
    });

    #[cfg(engine)]
    let template = template.head(move |cx| head(cx, static_path.clone()));

    template.build()
}
