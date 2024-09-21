use sycamore::prelude::*;

// inline_props for the win!
#[component(inline_props)]
fn BabianoKey<'a, G: Html>(cx: Scope<'a>, value: &'a ReadSignal<i32>) -> View<G> {
    view!  {cx,
        div(class="my-component") {
            "Value: " (value.get())
        }
    }
}
