// main.rs

use sycamore::prelude::*;

use ghlintranetv2::routes::router::SiteRouter;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
        SiteRouter {} }
    });
}
