use sycamore::{futures::spawn_local, prelude::*};
use web_sys::wasm_bindgen::JsCast;

#[derive(Prop)]
pub struct BabianoKeyAudioProps<'a, G: Html> {
    #[builder(default)]
    class: String,
    #[builder(default)]
    children: Children<'a, G>,

    #[builder(default)]
    keyboard_key: String,

    #[cfg(client)]
    file: &'a ReadSignal<Option<web_sys::File>>,
    #[cfg(engine)]
    file: &'a ReadSignal<Option<()>>,

    detune: &'a ReadSignal<f32>,
    volume: &'a ReadSignal<f32>,
}

// inline_props for the win!
#[component()]
pub fn BabianoKeyAudio<'a, G: Html>(cx: Scope<'a>, props: BabianoKeyAudioProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let playing = create_rc_signal(false);
    let allowed_to_play = create_rc_signal(false);

    #[cfg(client)]
    let context = web_sys::AudioContext::new().expect("Unable to create AudioContext");
    #[cfg(client)]
    let master_volume = context.create_gain().expect("Unable to create gain node");
    #[cfg(client)]
    master_volume
        .connect_with_audio_node(&context.destination())
        .expect("Unable to connect master");

    #[cfg(client)]
    let mut audio_node = create_rc_signal(context.create_buffer_source().unwrap());
    #[cfg(client)]
    audio_node.get().set_loop(true);
    #[cfg(client)]
    audio_node
        .get()
        .connect_with_audio_node(&master_volume)
        .unwrap();

    // Handle file
    let file = create_rc_signal(None);
    create_effect(cx, {
        let file = file.clone();
        move || {
            props.file.track();
            let binding = props.file.get();
            let value = binding.as_ref().clone();
            file.set(value);
        }
    });

    #[cfg(client)]
    let context_clone = context.clone();
    #[cfg(client)]
    let audio_buffer = create_rc_signal(None);

    #[cfg(client)]
    let audio_buffer_clone = audio_buffer.clone();
    create_effect(cx, move || {
        file.track();
        let binding = props.file.get();
        let value = binding.as_ref().clone();

        #[cfg(client)]
        let audio_buffer_clone = audio_buffer_clone.clone();
        #[cfg(client)]
        let context_clone = context_clone.clone();

        const NOTE_URL: &'static str = ".perseus/static/tada.mp3";
        #[cfg(client)]
        let note_url = format!("{}{NOTE_URL}", gloo::utils::window().location().to_string());
        #[cfg(client)]
        spawn_local(async move {
            if let Some(file) = value {
                let audio_file_promise = file.array_buffer();
                let binding = sycamore::futures::JsFuture::from(audio_file_promise)
                    .await
                    .unwrap();
                let audio_file_value = binding
                    .dyn_ref::<web_sys::js_sys::ArrayBuffer>().unwrap();
                let audio_buffer_promise =
                    context_clone.decode_audio_data(&audio_file_value).unwrap();
                let audio_buffer_value = sycamore::futures::JsFuture::from(audio_buffer_promise)
                    .await
                    .unwrap();
                audio_buffer_clone.set(Some(web_sys::AudioBuffer::from(audio_buffer_value)));
            } else {
                let bytes = reqwest::get(note_url).await.unwrap().bytes().await.unwrap();
                let array_buffer = web_sys::js_sys::Uint8Array::from(&bytes[..]).buffer();
                let audio_buffer_promise = context_clone.decode_audio_data(&array_buffer).unwrap();
                let audio_buffer_value = sycamore::futures::JsFuture::from(audio_buffer_promise)
                    .await
                    .unwrap();

                audio_buffer_clone.set(Some(web_sys::AudioBuffer::from(audio_buffer_value)));
            }
        });
    });

    #[cfg(client)]
    let context_clone = context.clone();
    #[cfg(client)]
    let audio_node_clone = audio_node.clone();
    #[cfg(client)]
    let master_volume_clone = master_volume.clone();
    #[cfg(client)]
    let allowed_to_play_clone = allowed_to_play.clone();
    #[cfg(client)]
    let mut play_buffer = move |offset| {
        if *allowed_to_play_clone.get() {
            audio_node_clone.get().stop().unwrap();
        }
        audio_node_clone.get().disconnect().unwrap();
        audio_node_clone.set(context_clone.create_buffer_source().unwrap());
        audio_node_clone
            .get()
            .connect_with_audio_node(&master_volume_clone)
            .unwrap();
        audio_node_clone
            .get()
            .set_buffer((*audio_buffer.get()).as_ref());
        audio_node_clone
            .get()
            .detune()
            .set_value(*props.detune.get());
        audio_node_clone
            .get()
            .start_with_when_and_grain_offset(0.0, offset);
    };

    // Handle mouse
    let playing_clone = playing.clone();
    let on_mousedown = move |_| {
        playing_clone.set(true);
    };

    let playing_clone = playing.clone();
    let on_mouseup = move |_| {
        playing_clone.set(false);
    };

    // Handle keyboard
    #[cfg(client)]
    {
        use std::rc::Rc;
        use web_sys::wasm_bindgen::prelude::Closure;
        use web_sys::wasm_bindgen::JsCast;
        let playing_clone = playing.clone();
        let keyboard_key_clone = props.keyboard_key.clone();
        // Maybe we should use keypress?
        let on_keydown = Rc::new(move |event: web_sys::Event| {
            let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            // We have to check if its already playing bcs firefox repeat doesnt work
            if keyboard_event.repeat() || *playing_clone.get() {
                return;
            }

            if keyboard_event.key() == keyboard_key_clone {
                playing_clone.set(true);
            };
        });

        let playing_clone = playing.clone();
        let keyboard_key_clone = props.keyboard_key.clone();
        let on_keyup = Rc::new(move |event: web_sys::Event| {
            let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            if keyboard_event.key() == keyboard_key_clone {
                playing_clone.set(false);
            };
        });
        let on_keydown_closure =
            Closure::new(
                Box::new(move |event: web_sys::Event| on_keydown(event)) as Box<dyn FnMut(_)>
            );
        let on_keyup_closure = Closure::new(
            Box::new(move |event: web_sys::Event| on_keyup(event)) as Box<dyn FnMut(_)>
        );
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback(
                "keydown",
                on_keydown_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keyup", on_keyup_closure.as_ref().unchecked_ref())
            .unwrap();
        on_keydown_closure.forget();
        on_keyup_closure.forget();
    }

    #[cfg(client)]
    let context_clone = context.clone();
    let allowed_to_play_clone = allowed_to_play.clone();
    let playing_clone = playing.clone();
    create_effect(cx, move || {
        props.volume.track();
        if *playing_clone.get() {
            if !*allowed_to_play_clone.get() {
                #[cfg(client)]
                play_buffer(0.0);

                allowed_to_play_clone.set(true);
            }
            #[cfg(client)]
            play_buffer(0.0);
            #[cfg(client)]
            gloo::console::log!("P");

            #[cfg(client)]
            master_volume.gain().set_value(*props.volume.get());
        } else {
            // #[cfg(client)]
            // master_volume
            //     .gain()
            //     .set_value(0.0);
        }
    });

    view! {cx,
        button(class=format!("{}{}", props.class, if *playing.get() { " playing" } else { "" }), on:mousedown=on_mousedown, on:mouseup=on_mouseup.clone(), on:mouseleave=on_mouseup) {
            (children)
        }
    }
}
