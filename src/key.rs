use sycamore::prelude::*;

#[derive(Prop)]
pub struct BabianoKeyProps<'a, G: Html> {
    #[builder(default)]
    class: String,
    #[builder(default)]
    children: Children<'a, G>,

    frequency: f32,
    volume: &'a ReadSignal<f32>,
}

// inline_props for the win!
#[component()]
pub fn BabianoKey<'a, G: Html>(cx: Scope<'a>, props: BabianoKeyProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let playing = create_signal(cx, false);
    let allowed_to_play = create_signal(cx, false);


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
        .set_value_at_time(props.frequency, 0.0)
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
        if !*allowed_to_play.get() {
            #[cfg(client)]
            oscillator_clone.start().unwrap();
            allowed_to_play.set(true);
        }
        playing.set(!*playing.get());
    };

    #[cfg(client)]
    let context_clone = context.clone();
    create_effect(cx, move || {
        props.volume.track();
        if *playing.get() {
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
        button(class=props.class, on:click=on_play) {
            (children)
        }
    }
}
