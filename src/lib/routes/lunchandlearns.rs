// knowledge.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::navigation::Navigation;

#[component]
pub fn LunchandLearns<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header {}
        Navigation {}
        article {
            h2 { "Lunch and Learns" }
            p { "This is the lunch and learns section. Here you will find access to recent GHL Lunch and Learn Sessions." }
            br {}
            table {
                thead {
                    tr {
                        td { "Date"}
                        td { "Topic" }
                        td { "Presenters" }
                    }
                }
                tbody {
                    tr {
                        td { "2021-09-23"}
                        td { "Mass Timber Fire Test Program Pilot Scale Demonstration (Preliminary Summary)" }
                        td { "Claire Yuan and Andrew Harmsworth" }
                    }
                    tr {
                        td { "2021-10-21"}
                        td { "Ideas for Alternative Solutions for Building Size and Area and Use of Mass Timber" }
                        td { "Andrew Harmsworth and Luke Kong" }
                    }
                    tr {
                        td { "2022-04-13"}
                        td { "Use of Fire Modelling Tools and Fire Alarm Cause and Effect Matrix" }
                        td { "Gary Chen, Dilip Rathinakumar, and Claire Yuan" }
                    }
                    tr {
                        td { "2022-04-27"}
                        td { "Fire Alarm Sequence of Operation Alternative Solutions" }
                        td { "Adam Nadem and Khash Vorell" }
                    }
                    tr {
                        td { "2022-04-04"}
                        td { "CAN/ULC-S1001" }
                        td { "WSP" }
                    }
                    tr {
                        td { "2022-06-01"}
                        td { "Distilleries" }
                        td { "Matt Turco" }
                    }
                }
            }
        }
        Footer {}
    }
}
