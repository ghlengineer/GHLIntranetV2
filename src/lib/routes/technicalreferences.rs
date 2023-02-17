// knowledge.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn TechnicalReferences<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
            Header {}
            Navigation {}
            article {
                h2 { "Technical References" }
                p { "This is a collection of fire science, code, and other technical references..." }
            }
            table {
                thead {
                    tr {
                        td { "Organization" }
                        td { "Resource" }
                        td { "Link" }
                        td { "Username" }
                        td { "Password" }
                    }
                }
                tbody {
                    tr {
                        td { "US Nuclear Regulatory Commission" }
                        td { "NRC Fire Dynamics Tools" }
                        td { a (href="https://www.nrc.gov/reading-rm/doc-collections/nuregs/staff/sr1805/s1/index.html", target="_blank") { "NRC Fire Dynamics Tools" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "National Institute of Standards and Technology" }
                        td { "Fire Cone Calorimetry Database" }
                        td { a (href="https://www.nist.gov/el/fcd", target="_blank") { "Fire Cone Calorimetry Database" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "National Fire Protection Association" }
                        td { "NFPA Codes Online" }
                        td { a (href="https://codesonline.nfpa.org/", target="_blank") { "NFPA Codes Onine" } }
                        td { "ghlengineer@ghl.ca" }
                        td { "gc$11056" }
                    }
                    tr {
                        td { "Underwriters Laboratories" }
                        td { "UL Online Directory" }
                        td { a (href="https://iq2.ulprospector.com/", target="_blank") { "NFPA Codes Online" } }
                        td { "Create your own" }
                        td { "Create your own" }
                    }
                }
            Footer {}
        }
    }
}
