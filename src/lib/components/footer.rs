// footer.rs

use sycamore::prelude::*;

use crate::time::get_current_year;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    let year = get_current_year();
    view! { cx,
    footer {
        p { "800 - 700 W Pender St, Vancouver, BC V6C 168" }
        p { "Copyright " (year) " GHL Consultants Ltd | All Rights Reserved"}
        a (href="https://www.ghl.ca", target="_blank") { " GHL Web Site " }
        a (href="https://www.google.com", target="_blank") { " Google Search" }
        a (href="https://ghl.egnyte.com", target="_blank") { " GHL Egnyte Web Portal " }
        a (href="https://portal.office.com", target="_blank") { " Microsoft 365 Portal " }
        a (href="https://security.microsoft.com/quarantine", target="_blank") { " Email Quarantine Viewer " }
    }}
}
