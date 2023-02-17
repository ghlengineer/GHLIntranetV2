use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use super::certifiedprofessional::CertifiedProfessional;
use super::home::Home;
use super::informationtech::InformationTech;
use super::knowledge::Knowledge;
use super::lunchandlearns::LunchandLearns;
use super::masstimber::MassTimber;
use super::technicalreferences::TechnicalReferences;
use super::workload::WorkLoad;

#[derive(Route)]
enum SiteRoutes {
    #[to("/")]
    Home,
    #[to("/knowledge")]
    Knowledge,
    #[to("/masstimber")]
    MassTimber,
    #[to("/certifiedprofessional")]
    CertifiedProfessional,
    #[to("/technicalreferences")]
    TechnicalReferences,
    #[to("/workload")]
    WorkLoad,
    #[to("/lunchandlearns")]
    LunchandLearns,
    #[to("/informationtech")]
    InformationTech,
    #[not_found]
    NotFound,
}

#[component]
pub fn SiteRouter<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<SiteRoutes>| {
                view! { cx,
                    div {
                        (match route.get().as_ref() {
                            SiteRoutes::Home => view! { cx,
                                Home {}
                            },
                            SiteRoutes::Knowledge => view! { cx,
                                Knowledge {}
                            },
                            SiteRoutes::MassTimber => view! { cx,
                                MassTimber {}
                            },
                            SiteRoutes::CertifiedProfessional => view! { cx,
                                CertifiedProfessional {}
                            },
                            SiteRoutes::TechnicalReferences => view! { cx,
                                TechnicalReferences {}
                            },
                            SiteRoutes::WorkLoad => view! { cx,
                                WorkLoad {}
                            },
                            SiteRoutes::LunchandLearns => view! { cx,
                                LunchandLearns {}
                            },
                            SiteRoutes::InformationTech => view! { cx,
                                InformationTech {}
                            },
                            SiteRoutes::NotFound => view! { cx,
                                "404 Not Found"
                            },
                        })
                    }
                }
            }
        )
    }
}
