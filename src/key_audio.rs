use sycamore::prelude::*;

#[derive(Prop)]
pub struct BabianoKeyAudioProps<'a, G: Html> {
    #[builder(default)]
    class: String,
    #[builder(default)]
    children: Children<'a, G>,

    #[builder(default)]
    keyboard_key: String,

    #[builder(default)]
    fallback_href: String,

    #[cfg(client)]
    file: &'a ReadSignal<Option<web_sys::File>>,
    #[cfg(engine)]
    file: &'a ReadSignal<Option<()>>,

    detune: &'a ReadSignal<f32>,
    volume: &'a ReadSignal<f32>,
}

#[component]
pub fn BabianoKeyAudio<'a, G: Html>(cx: Scope<'a>, props: BabianoKeyAudioProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let playing = create_rc_signal(false);
    let allowed_to_play = create_rc_signal(false);

    // Setup audio context
    #[cfg(client)]
    let context = web_sys::AudioContext::new().expect("Unable to create AudioContext");

    // Gain node (connects to speakers)
    #[cfg(client)]
    let master_volume = context.create_gain().expect("Unable to create gain node");
    #[cfg(client)]
    master_volume
        .connect_with_audio_node(&context.destination())
        .expect("Unable to connect master");

    // Audio node (connects to gain node)
    #[cfg(client)]
    let mut audio_node = create_rc_signal(context.create_buffer_source().unwrap());
    #[cfg(client)]
    audio_node
        .get()
        .connect_with_audio_node(&master_volume)
        .unwrap();

    // Audio buffer (aka sample) (connects to audio node)
    #[cfg(client)]
    let audio_buffer = create_rc_signal(None);
    create_effect(cx, {
        #[cfg(client)]
        let context = context.clone();
        #[cfg(client)]
        let audio_buffer = audio_buffer.clone();
        move || {
            props.file.track();

            #[cfg(client)]
            use sycamore::futures::spawn_local;
            #[cfg(client)]
            spawn_local({
                use web_sys::wasm_bindgen::JsCast;
                let context = context.clone();
                let audio_buffer = audio_buffer.clone();
                let fallback_href = props.fallback_href.clone();

                let binding = props.file.get();
                let audio_file = binding.as_ref().clone();
                async move {
                    // If we have a file use it, else fetch and use fallback
                    if let Some(file) = audio_file {
                        // Turn into ArrayBuffer
                        let audio_file_promise = file.array_buffer();
                        let binding = sycamore::futures::JsFuture::from(audio_file_promise)
                            .await
                            .unwrap();
                        let audio_file_value =
                            binding.dyn_ref::<web_sys::js_sys::ArrayBuffer>().unwrap();

                        // Decode it into actual audio data
                        let audio_buffer_promise =
                            context.decode_audio_data(&audio_file_value).unwrap();
                        let audio_buffer_value =
                            sycamore::futures::JsFuture::from(audio_buffer_promise)
                                .await
                                .unwrap();

                        // Set it :3
                        audio_buffer.set(Some(web_sys::AudioBuffer::from(audio_buffer_value)));
                    } else {
                        // Get base of the url if it doesnt have one (maybe not check like this)
                        let fallback_href = if fallback_href.starts_with("http") {
                            fallback_href
                        } else {
                            format!("{}/{fallback_href}", gloo::utils::window().origin())
                        };

                        // Fetch audio
                        gloo::console::log!(fallback_href.clone());
                        let bytes = reqwest::get(fallback_href)
                            .await
                            .unwrap()
                            .bytes()
                            .await
                            .unwrap();

                        // Turn into ArrayBuffer and decode it into actual audio data
                        let array_buffer = web_sys::js_sys::Uint8Array::from(&bytes[..]).buffer();
                        let audio_buffer_promise =
                            context.decode_audio_data(&array_buffer).unwrap();
                        let audio_buffer_value =
                            sycamore::futures::JsFuture::from(audio_buffer_promise)
                                .await
                                .unwrap();

                        // Set it :3
                        audio_buffer.set(Some(web_sys::AudioBuffer::from(audio_buffer_value)));
                    }
                }
            });
        }
    });

    // Function to play the sample (starting from offset)
    #[cfg(client)]
    let mut play_buffer = {
        #[cfg(client)]
        let context = context.clone();
        #[cfg(client)]
        let audio_node = audio_node.clone();
        #[cfg(client)]
        let master_volume = master_volume.clone();
        #[cfg(client)]
        let allowed_to_play = allowed_to_play.clone();
        move |offset| {
            // There is no nice way to stop and start the sound after the first time for some reason,
            // se we have to remove it and put it back again X3

            // Disconnect old buffer
            if *allowed_to_play.get() {
                //TODO: depracated?
                audio_node.get().stop().unwrap();
            }
            audio_node.get().disconnect().unwrap();

            // Set new buffer
            audio_node.set(context.create_buffer_source().unwrap());
            audio_node
                .get()
                .connect_with_audio_node(&master_volume)
                .unwrap();
            audio_node.get().set_buffer((*audio_buffer.get()).as_ref());
            audio_node.get().detune().set_value(*props.detune.get());
            audio_node
                .get()
                .start_with_when_and_grain_offset(0.0, offset);
        }
    };

    // Handle mouse
    let on_mousedown = {
        let playing = playing.clone();
        move |_| {
            playing.set(true);
        }
    };

    let on_mouseup = {
        let playing = playing.clone();
        move |_| {
            playing.set(false);
        }
    };

    // Handle keyboard
    #[cfg(client)]
    {
        use std::rc::Rc;
        use web_sys::wasm_bindgen::prelude::Closure;
        use web_sys::wasm_bindgen::JsCast;

        //NOTE: Maybe we should use "keypress"?

        // keydown event (When the key is down or held down)
        let on_keydown = Rc::new({
            let playing = playing.clone();
            let keyboard_key = props.keyboard_key.clone();
            move |event: web_sys::Event| {
                let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                // We have to check if its already playing bcs in firefox repeat doesnt work
                if keyboard_event.repeat() || *playing.get() {
                    return;
                }

                if keyboard_event.key() == keyboard_key {
                    playing.set(true);
                };
            }
        });

        // keyup event (When the key is lifted)
        let on_keyup = Rc::new({
            let playing = playing.clone();
            let keyboard_key = props.keyboard_key.clone();
            move |event: web_sys::Event| {
                let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                if keyboard_event.key() == keyboard_key {
                    playing.set(false);
                };
            }
        });

        // Rust to js :3
        let on_keydown_closure =
            Closure::new(
                Box::new(move |event: web_sys::Event| on_keydown(event)) as Box<dyn FnMut(_)>
            );
        let on_keyup_closure = Closure::new(
            Box::new(move |event: web_sys::Event| on_keyup(event)) as Box<dyn FnMut(_)>
        );

        // Register events
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

        //TODO: Clean this up

        // And finally, forget to clean them up so they stay forever :p
        on_keydown_closure.forget();
        on_keyup_closure.forget();
    }

    // Play the sound and Turn up the volume
    create_effect(cx, {
        #[cfg(client)]
        let context = context.clone();
        let playing = playing.clone();
        let allowed_to_play = allowed_to_play.clone();
        move || {
            props.volume.track();
            if *playing.get() {
                if !*allowed_to_play.get() {
                    #[cfg(client)]
                    play_buffer(0.0);

                    allowed_to_play.set(true);
                }
                #[cfg(client)]
                play_buffer(0.0);
                #[cfg(client)]
                gloo::console::log!("P");

                #[cfg(client)]
                master_volume.gain().set_value(*props.volume.get());
            } else {
                //TODO: Make stopping the sound when key is lifted an option
                // #[cfg(client)]
                // master_volume
                //     .gain()
                //     .set_value(0.0);
            }
        }
    });

    // Button!
    view! {cx,
        button(class=format!("{}{}", props.class, if *playing.get() { " playing" } else { "" }), on:mousedown=on_mousedown, on:mouseup=on_mouseup.clone(), on:mouseleave=on_mouseup) {
            (children)
        }
    }
}
