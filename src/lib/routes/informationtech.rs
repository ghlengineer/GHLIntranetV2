// knowledge.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn InformationTech<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Information Technology" }
            p { "Here you will find resources related to GHL information technology systems" }
            br {}
            h3 { "Nucleus Networks" }
            p { "GHL's IT provider is Nucleus Networks. They can be contacted by phone, email, or through the support app." }
            p { "Phone: 604-682-3444" }
            p { "Email: helpdesk@ghl.ca" }
        }
        Footer {}
    }
}
