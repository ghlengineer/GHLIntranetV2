// src/lib/components/altsolnsearch.rs

use crate::domain::altsolns::AltSolns;
use reqwasm::http::Request;
use sycamore::prelude::*;
use sycamore::builder::prelude::*;

// API location for the alternative solutions endpoint
const API_BASE_URL: &str = "https://ghl-van-app03:5050/altsolns";

async fn fetch_altsolns() -> Result<AltSolns, reqwasm::Error> {
    let url = format!("{}", API_BASE_URL);
    let resp = Request::get(&url).send().await?;

    resp.json::<AltSolns>().await
}

#[component]
pub async fn DisplayAltSolns<G: Html>(cx: Scope<'_>) -> View<G> {
    let _altsolns = fetch_altsolns().await.unwrap_or_default();
    ul()
        .c(li().t("test"))
    .view(cx)
}