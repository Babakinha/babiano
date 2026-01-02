use leptos::{ev, prelude::*, task::spawn_local};

#[component]
pub fn BabianoKeyAudio(
    #[prop(optional, default = "")] class: &'static str,
    #[prop(optional, default = "")] keyboard_key: &'static str,
    #[prop(into)] file: Signal<Vec<u8>>,
    #[prop(into, optional)] detune: Signal<f32>,
    #[prop(into)] volume: Signal<f32>,
    children: Children,
) -> impl IntoView {
    let (pressed, set_pressed) = signal(false);
    let pressed_memo = Memo::new(move |_| pressed.get());

    #[cfg(feature = "hydrate")]
    use std::cell::RefCell;
    #[cfg(feature = "hydrate")]
    use std::rc::Rc;

    #[cfg(feature = "hydrate")]
    let context = Rc::new(web_sys::AudioContext::new().expect("AudioContext"));

    #[cfg(feature = "hydrate")]
    let master_gain = Rc::new(context.create_gain().unwrap());

    #[cfg(feature = "hydrate")]
    master_gain
        .connect_with_audio_node(&context.destination())
        .unwrap();

    #[cfg(feature = "hydrate")]
    let audio_node: Rc<RefCell<Option<web_sys::AudioBufferSourceNode>>> =
        Rc::new(RefCell::new(None));

    #[cfg(feature = "hydrate")]
    let (audio_buffer, set_audio_buffer) = arc_signal(None);

    // Play sound
    #[cfg(feature = "hydrate")]
    let play_sound = {
        let context = context.clone();
        let master_gain = master_gain.clone();
        let audio_node = audio_node.clone();
        let audio_buffer = audio_buffer.clone();

        move || {
            // TODO: Depracated?
            if let Some(node) = audio_node.borrow_mut().take() {
                let _ = node.stop();
                let _ = node.disconnect();
            }

            let node = context.create_buffer_source().unwrap();
            node.connect_with_audio_node(&master_gain).unwrap();
            node.set_buffer(audio_buffer.get().as_ref());
            node.detune().set_value(detune.get());
            let _ = node.start_with_when(0.0);

            *audio_node.borrow_mut() = Some(node);
        }
    };

    #[cfg(feature = "hydrate")]
    let stop_sound = {
        let audio_node = audio_node.clone();

        move || {
            // TODO: Depracated?
            if let Some(node) = audio_node.borrow_mut().take() {
                let _ = node.stop();
                let _ = node.disconnect();
            }
        }
    };

    // When pressed
    Effect::new(move || {
        pressed_memo.track();

        #[cfg(feature = "hydrate")]
        if pressed_memo.get() {
            play_sound()
        }
    });

    // File change
    {
        #[cfg(feature = "hydrate")]
        let context = context.clone();

        Effect::new(move || {
            file.track();

            #[cfg(feature = "hydrate")]
            {
                let u8_array = web_sys::js_sys::Uint8Array::new_from_slice(&file.get());
                let buffer = u8_array.buffer();

                let promise = context.decode_audio_data(&buffer).unwrap();
                let future = wasm_bindgen_futures::JsFuture::from(promise);
                let set_audio_buffer = set_audio_buffer.clone();

                spawn_local(async move {
                    if let Ok(value) = future.await {
                        use wasm_bindgen::JsCast;
                        let audio_buffer = value.unchecked_into::<web_sys::AudioBuffer>();
                        set_audio_buffer.set(Some(audio_buffer));
                    }
                });
            }
        });
    }
    // Change volume
    {
        #[cfg(feature = "hydrate")]
        let master_gain = master_gain.clone();

        Effect::new(move || {
            volume.track();
            #[cfg(feature = "hydrate")]
            master_gain.gain().set_value(volume.get());
        });
    }

    // Keyboard events
    window_event_listener(ev::keydown, move |ev| {
        if !ev.repeat() && ev.key() == keyboard_key {
            set_pressed.set(true);
        }
    });

    window_event_listener(ev::keyup, move |ev| {
        if ev.key() == keyboard_key {
            set_pressed.set(false);
        }
    });

    // Mouse enter
    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |ev: web_sys::MouseEvent| {
        if ev.buttons() == 1 {
            set_pressed.set(true);
        }
    };
    #[cfg(feature = "export")]
    let on_mouseenter = move |_| {};

    view! {
          <button class=move || {format!("{class} {}", if pressed_memo.get() {"pressed"} else {""})} on:mouseenter=on_mouseenter on:mousedown=move |_| set_pressed.set(true) on:mouseup=move |_| set_pressed.set(false) on:mouseleave=move |_| set_pressed.set(false) >
            {children()}
          </button>
    }
}
