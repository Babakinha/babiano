// OLD, DONT USE >.<
use sycamore::prelude::*;

#[derive(Prop)]
pub struct BabianoKeyProps<'a, G: Html> {
    #[builder(default)]
    class: String,
    #[builder(default)]
    children: Children<'a, G>,

    #[builder(default)]
    keyboard_key: String,

    frequency: f32,
    volume: &'a ReadSignal<f32>,
}

#[component]
pub fn BabianoKey<'a, G: Html>(cx: Scope<'a>, props: BabianoKeyProps<'a, G>) -> View<G> {
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
    let oscillator = context
        .create_oscillator()
        .expect("Unable to create oscillator");
    #[cfg(client)]
    oscillator
        .frequency()
        .set_value_at_time(props.frequency, 0.0)
        .expect("Unable to set tone");
    #[cfg(client)]
    oscillator
        .connect_with_audio_node(&master_volume)
        .expect("Unable to connect oscillator");

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
        use web_sys::wasm_bindgen::JsCast;
        use web_sys::wasm_bindgen::prelude::Closure;
        use std::rc::Rc;
        let playing_clone = playing.clone();
        let keyboard_key_clone = props.keyboard_key.clone();
        let on_keydown = Rc::new(move |event: web_sys::Event| {
            let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
            if keyboard_event.repeat() {
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
        let on_keydown_closure = Closure::new(Box::new(move |event: web_sys::Event| on_keydown(event)) as Box<dyn FnMut(_)>);
        let on_keyup_closure = Closure::new(Box::new(move |event: web_sys::Event| on_keyup(event)) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback("keydown", on_keydown_closure.as_ref().unchecked_ref()).unwrap();
        web_sys::window().unwrap().add_event_listener_with_callback("keyup", on_keyup_closure.as_ref().unchecked_ref()).unwrap();
        on_keydown_closure.forget();
        on_keyup_closure.forget();
    }

    #[cfg(client)]
    let context_clone = context.clone();
    let allowed_to_play_clone = allowed_to_play.clone();
    #[cfg(client)]
    let oscillator_clone = oscillator.clone();
    let playing_clone = playing.clone();
    create_effect(cx, move || {
        props.volume.track();
        if *playing_clone.get() {
            if !*allowed_to_play_clone.get() {
                #[cfg(client)]
                oscillator_clone.start().unwrap();
                allowed_to_play_clone.set(true);
            }
            #[cfg(client)]
            master_volume
                .gain()
                .set_value_at_time(*props.volume.get(), context_clone.current_time())
                .unwrap();
        } else {
            #[cfg(client)]
            master_volume
                .gain()
                .set_value_at_time(0.0, context_clone.current_time())
                .unwrap();
        }
    });

    view! {cx,
        button(class=props.class, on:mousedown=on_mousedown, on:mouseup=on_mouseup.clone(), on:mouseleave=on_mouseup) {
            (children)
        }
    }
}
