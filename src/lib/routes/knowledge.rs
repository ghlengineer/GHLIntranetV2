// knowledge.rs

use sycamore::prelude::*;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::altsolnsearch::DisplayAltSolns;

#[component]
pub fn Knowledge<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Code Knowledge Base" }
            p { "This is a collection of Code appeals, interpretations, and opinions, both internal and external" }
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
                        td { "National Research Council" }
                        td { "National Model Codes" }
                        td { a (href="https://nrc-publications.canada.ca/eng/search/?q=NRCCode&q=&q=&ps=50&s=dtp&m=1", target="_blank") { "National Model Codes" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "National Research Council" }
                        td { "NBC 2015 Intent Statements" }
                        td { a (href="https://codes-guides.nrc.ca/IA/15NBC/intentframe.html", target="_blank") { "NBC 2015 Intent Statements" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "National Research Council" }
                        td { "NBC 1995 User's Guide" }
                        td { a (href="https://nrc-publications.canada.ca/eng/view/ft/?id=f4c59aa5-16df-4b40-b9db-d6c65cae71de", target="_blank") { "NBC 1995 User's Guide" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "Province of British Columbia" }
                        td { "BC Building Code Resources" }
                        td { a (href="https://www2.gov.bc.ca/gov/content/industry/construction-industry/building-codes-standards", target="_blank") { "BC Building Code Resources" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "Province of British Columbia" }
                        td { "Building Access Handbook 2020" }
                        td { a (href="https://www2.gov.bc.ca/assets/gov/farming-natural-resources-and-industry/construction-industry/building-codes-and-standards/guides/2020_building_accessibility_handbook.pdf", target="_blank") { "Building Accessibility Handbook 2020" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "Province of British Columbia" }
                        td { "BC Building Code Appeals" }
                        td { a (href="https://www2.gov.bc.ca/gov/content/industry/construction-industry/building-codes-standards/building-code-appeal-board/building-code-appeal-board-decisions", target="_blank") { "BC Building Code Appeals" } }
                        td { "None" }
                        td { "None" }
                    }
                    tr {
                        td { "Building Officials Association of BC" }
                        td { "BC Building Code Interpretations" }
                        td { a (href="https://boabc.org/interpretations-committee/#:~:text=The%20purpose%20of%20the%20BC%20Building%20Code%20Interpretation,To%20disseminate%20the%20completed%20interpretations%20to%20code%20users", target="_blank") { "BC Building Code Appeals" } }
                        td { "None" }
                        td { "None" }
                    }
                }
            }
        }
        DisplayAltSolns {}
        Footer {}
    }
}
