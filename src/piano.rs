use std::collections::HashMap;

use crate::key_audio::BabianoKeyAudio;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

const MEOW_SOUND_URL: &'static str = "assets/meow.wav";
const MEOW_2_SOUND_URL: &'static str = "assets/meow_2_b4.wav";
const PIANO_SOUND_URL: &'static str = "assets/piano_b3.wav";
const FISH_SOUND_URL: &'static str = "assets/meow_2.wav";

#[component]
pub fn BabianoPage() -> impl IntoView {
    let (volume, set_volume) = signal(50.0);
    let (tune, set_tune) = signal(50.0);
    let (file_data, set_file_data) = signal(Vec::new());
    let sample_list = RwSignal::new(HashMap::<String, (String, Vec<u8>)>::new());
    let (selected_sample, set_selected_sample) = signal(String::from(" 0_meow"));

    // Use meow.wav by deafault
    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;

        let load_sound = move |sample_id: &'static str, sample_name: &'static str, url: &'static str| {
            spawn_local(async move {
                sample_list
                    .write()
                    .insert(sample_id.to_string(), (sample_name.to_string(), Vec::new()));
                let data = gloo_net::http::Request::get(&url).send().await.expect("Unable to fetch sound");
                let bytes = data
                    .binary()
                    .await
                    .expect("Unable to extract bytes from sound");
                sample_list.write().get_mut(sample_id).unwrap().1 = bytes.to_vec();
            });
        };

        // The weird start is so it appears in the right order X3
        load_sound(" 0_meow", "Meow :3", MEOW_SOUND_URL);
        load_sound(" 1_meow", "Meow 2 (2026 edition) >:3", MEOW_2_SOUND_URL);
        load_sound(" 2_piano", "Piano", PIANO_SOUND_URL);
        load_sound(" 3_fish", "FISH", FISH_SOUND_URL);
    }

    Effect::new(move || {
        let binding = sample_list.read();
        let data = binding.get(&selected_sample.get());

        match data {
            Some(data) => set_file_data.set(data.1.clone()),
            None => {/* TODO: Handle this */},
        }
    });

    // Read file bytes
    #[cfg(feature = "hydrate")]
    let on_file = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        let binding = ev.target().unwrap();
        let input_node = binding.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        if let Some(file) = input_node.files().unwrap().get(0) {
            use leptos::task::spawn_local;
            sample_list
                .write()
                .insert(file.name(), (file.name(), Vec::new()));

            let promise = file.array_buffer();
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            spawn_local(async move {
                if let Ok(js_value) = future.await {
                    //TODO: There is prob a better way to do this, like passing the buffer directly
                    let bytes = web_sys::js_sys::Uint8Array::new(&js_value);
                    sample_list.write().get_mut(&file.name()).unwrap().1 = bytes.to_vec();
                    set_selected_sample.set(file.name());
                }
            });
        }
    };

    #[cfg(feature = "export")]
    let on_file = move |_| {};

    #[cfg(feature = "hydrate")]
    let on_sample = move |ev: web_sys::Event| {
        set_selected_sample.set(event_target_value(&ev));
    };

    #[cfg(feature = "export")]
    let on_sample = move |_| {};

    let volume_memo = Memo::new(move |_| volume.get() as f32 / 100.0);

    view! {
        // Title and description
        <Title text="~UwU What is this~" />
        <Meta name="description" content="Cool piano -w-" />

        // Font
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" attr:crossorigin />
        <Link rel="preload" attr:r#as="style" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        <Link rel="stylesheet" media="print" attr:onload="this.media='all'" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        <noscript>
            <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Kanit:ital,wght@0,400;0,700;1,400&display=swap" />
        </noscript>

        // Main
        <h1> "Babiano Page! :3" </h1>

        <label for="volume"> "Volume:" </label>
        <input type="range" id="volume" min="0" max="100" prop:value=volume on:input=move |ev| set_volume.set(event_target_value(&ev).parse().unwrap()) class="slider" />
        <label for="tune"> "Tune:" </label>
        <input type="range" id="tune" min="-1200" step="100" max="1200" prop:value=tune on:input=move |ev| set_tune.set(event_target_value(&ev).parse().unwrap()) class="slider" />
        <select on:change=on_sample name="sample" id="sample_list">
            {move || {
                let binding = sample_list.read();
                let mut items: Vec<_> = binding.iter().collect();

                // TODO: There is prob a better way to do this
                if items.is_empty() {
                    return vec![view! {
                        <option selected=true value="".to_string()>
                            {"Loading...".to_string()}
                        </option>
                    }];
                }

                items.sort_by(|a, b| a.0.cmp(&b.0));

                items
                    .into_iter()
                    .map(|(key, value)| {
                        let selected = &selected_sample.get() == key;
                        view! {
                            <option selected=selected value={key.clone()}>
                                {value.0.clone()}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()

             }}
        </select>
        <label for="sample"> "Upload" </label>
        <input type="file" style="display:none;" id="sample" on:change=on_file value="Upload?" />
        <div style="padding-top: 300px; padding-bottom: 300px; justify-content: center; height: 120px; display:flex;">
            <BabianoKeyAudio class="white-note" keyboard_key="a" file=file_data detune=Memo::new(move |_| tune.get() + 100.0) volume=volume_memo > "a" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="w" file=file_data detune=Memo::new(move |_| tune.get() + 200.0) volume=volume_memo > "w" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="s" file=file_data detune=Memo::new(move |_| tune.get() + 300.0) volume=volume_memo > "s" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="e" file=file_data detune=Memo::new(move |_| tune.get() + 400.0) volume=volume_memo > "e" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="d" file=file_data detune=Memo::new(move |_| tune.get() + 500.0) volume=volume_memo > "d" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="f" file=file_data detune=Memo::new(move |_| tune.get() + 600.0) volume=volume_memo > "f" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="t" file=file_data detune=Memo::new(move |_| tune.get() + 700.0) volume=volume_memo > "t" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="g" file=file_data detune=Memo::new(move |_| tune.get() + 800.0) volume=volume_memo > "g" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="y" file=file_data detune=Memo::new(move |_| tune.get() + 900.0) volume=volume_memo > "y" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="h" file=file_data detune=Memo::new(move |_| tune.get() + 1000.0) volume=volume_memo > "h" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="u" file=file_data detune=Memo::new(move |_| tune.get() + 1100.0) volume=volume_memo > "u" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="j" file=file_data detune=Memo::new(move |_| tune.get() + 1200.0) volume=volume_memo > "j" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="k" file=file_data detune=Memo::new(move |_| tune.get() + 1300.0) volume=volume_memo > "k" </BabianoKeyAudio>
            <BabianoKeyAudio class="black-note" keyboard_key="i" file=file_data detune=Memo::new(move |_| tune.get() + 1400.0) volume=volume_memo > "i" </BabianoKeyAudio>
            <BabianoKeyAudio class="white-note" keyboard_key="l" file=file_data detune=Memo::new(move |_| tune.get() + 1500.0) volume=volume_memo > "l" </BabianoKeyAudio>
        </div>

        // Footer
        <p> "I love you ❤️" </p>
    }
}
