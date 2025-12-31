use crate::key_audio::BabianoKeyAudio;
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_meta::{Link, Meta, Title};

#[component]
pub fn BabianoPage() -> impl IntoView {
    let (volume, set_volume) = signal(50.0);
    let (tune, set_tune) = signal(50.0);
    let (file_data, set_file_data) = signal(Vec::new());



    Effect::new(move |_| {
        console_log(&format!("Volume: {}; Tune: {}", volume.get(), tune.get()));
    });

    #[cfg(feature = "hydrate")]
    let on_file = move |ev: web_sys::Event| {
        console_log(&format!("ev: {}", event_target_value(&ev)));

        use web_sys::wasm_bindgen::JsCast;
        let binding = ev.target().unwrap();
        let input_node = binding.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        if let Some(file) = input_node.files().unwrap().get(0) {
            use leptos::task::spawn_local;

            console_log(&format!("{file:?}"));
            let promise = file.array_buffer();
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            spawn_local(async move {
                if let Ok(js_value) = future.await {
                    //TODO: There is prob a better way to do this, like passing the buffer directly
                    let test = web_sys::js_sys::Uint8Array::new(&js_value);
                    set_file_data.set(test.to_vec());
                }
            });
        }
    };

    #[cfg(feature = "export")]
    let on_file = move |_| {};

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
        <input type="range" id="tune" min="-1000" step="100" max="1000" prop:value=tune on:input=move |ev| set_tune.set(event_target_value(&ev).parse().unwrap()) class="slider" />
        <label for="sample"> "Sample:" </label>
        <input type="file" id="sample" on:change=on_file />
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
