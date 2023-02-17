// home.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Welcome" }
            h3 { "Today's News:"}
            p { "Public review on proposed changes to the NBC 2020 is live:" }
            a (href="https://ccbfc-cccbpi.ca/en/get-involved/public-review-on-proposed-changes-to-codes-canada-publications-2022/?utm_source=DialogInsight&utm_medium=email&utm_campaign=Coming%20soon!%20First%20public%20review%20of%20proposed%20changes%20to%20the%202020%20National%20Model%20Codes%20%2f%20%C3%80%20venir%20bient%C3%B4t%20%E2%80%93%20le%20p%20-%20copie_20221024") { "NBC 2020 Proposed Changes" }
        }
        Footer {}
    }
}
