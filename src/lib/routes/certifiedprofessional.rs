// knowledge.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn CertifiedProfessional<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Certified Professional" }
            p { "This is the Certified Professional section..." }
        }
        Footer {}
    }
}
