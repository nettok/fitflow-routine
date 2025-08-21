use axum::BoxError;
use config::{Case, Config};
use serde::Deserialize;
use std::str::FromStr;
use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RunProfile {
    Dev,
    Prod,
}

pub fn load_app_config<'de, T: Clone + Deserialize<'de>>() -> Result<T, BoxError> {
    let default_run_profile = RunProfile::Dev;

    let profile = std::env::var("RUN_PROFILE")
        .map(|env_profile| {
            RunProfile::from_str(&env_profile).unwrap_or(default_run_profile.clone())
        })
        .unwrap_or(default_run_profile)
        .to_string();

    let conf = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{}", profile)).required(false))
        .add_source(config::Environment::default().convert_case(Case::Snake))
        .build()?;

    conf.try_deserialize::<T>().map_err(|e| e.into())
}
