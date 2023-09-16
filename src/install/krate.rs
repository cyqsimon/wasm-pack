use crate::install::Tool;
use anyhow::Result;
use log::debug;
use serde::Deserialize;
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
pub struct Krate {
    pub max_version: String,
}

#[derive(Debug, Deserialize)]
pub struct KrateResponse {
    #[serde(rename = "crate")]
    pub krate: Krate,
}

impl Krate {
    pub fn new(name: &Tool) -> Result<Krate> {
        let krate_address = format!("https://crates.io/api/v1/crates/{}", name);
        let res = ureq::get(&krate_address)
            .set(
                "user-agent",
                &format!("wasm-pack/{}", VERSION.unwrap_or("unknown")),
            )
            .call()?;

        let kr: KrateResponse = res.into_json()?;
        debug!(
            "Latest `{name}` version on crates.io is {}",
            kr.krate.max_version
        );
        Ok(kr.krate)
    }
}
