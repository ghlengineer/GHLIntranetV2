// main.rs

use sycamore::prelude::*;

use ghl_intranet_sycamore_v3_lib::routes::router::SiteRouter;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
        SiteRouter {} }
    });
}
