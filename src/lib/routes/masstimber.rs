// knowledge.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn MassTimber<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Mass Timber" }
            p { "This is the mass timber section..." }
        }
        Footer {}
    }
}
