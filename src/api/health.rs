use axum::Json;
use serde::Serialize;
use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Healthy,
}

#[derive(Clone, Debug, Serialize)]
pub struct HealthZ {
    status: Status,
}

#[tracing::instrument]
pub async fn get_healthz() -> Json<HealthZ> {
    Json(HealthZ {
        status: Status::Healthy,
    })
}
