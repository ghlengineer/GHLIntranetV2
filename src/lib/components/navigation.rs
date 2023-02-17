// navigation.rs

use sycamore::prelude::*;

#[component]
pub fn Navigation<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
    nav {
        ul {
            li {
                a(href="/") { "Home" }
            }
            li {
                a(href="/knowledge") { "Knowledge" }
            }
            li {
                a(href="/masstimber") { "Mass Timber" }
            }
            li {
                a(href="/certifiedprofessional") { "Certified Professional" }
            }
            li {
                a(href="/technicalreferences") { "Technical References" }
            }
            li {
                a(href="/workload") { "Work Load" }
            }
            li {
                a(href="/lunchandlearns") { "Lunch and Learns" }
            }
            li {
                a(href="/informationtech") { "Information Tech" }
            }
            }
        }
    }
}
