// header.rs

use sycamore::prelude::*;

use crate::time::get_current_date;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    let today = get_current_date();
    view! { cx,
    header {
        img (src="GHL_Logo.jpg") {}
        h2 { "GHL Intranet" }
        h3 { "Today is: " (today) }
    }}
}
