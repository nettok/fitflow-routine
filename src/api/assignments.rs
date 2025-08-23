use crate::AppState;
use crate::errors::AppError;
use axum::Json;
use axum::extract::{Path, State};
use axum::response::NoContent;
use redis::AsyncTypedCommands;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};

const DB_KEY_PREFIX: &str = "user-assignments:";

type UserId = String;
type RoutineId = String;

#[derive(Clone, Debug, Serialize)]
pub struct Assignment {
    user_id: UserId,
    routine_id: RoutineId,
    status: Status,
}

#[derive(Clone, Debug, Display, EnumString, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Assigned,
    Started,
    Completed,
}

#[derive(Debug, Deserialize)]
pub struct PutAssignmentsParams {
    user_id: UserId,
    routine_id: RoutineId,
}

#[derive(Debug, Serialize)]
pub struct UserAssignments {
    assignments: Vec<Assignment>,
}

#[tracing::instrument(skip(state))]
pub async fn get_user_assignments(
    State(state): State<AppState>,
    Path(user_id): Path<UserId>,
) -> Result<Json<UserAssignments>, AppError> {
    let routine_status_map = state
        .redis_pool
        .get()
        .await?
        .hgetall(DB_KEY_PREFIX.to_owned() + &*user_id)
        .await?;

    let assigned_routines = routine_status_map
        .iter()
        .map(|(routine_id, status)| Assignment {
            user_id: user_id.to_owned(),
            routine_id: routine_id.to_owned(),
            status: Status::from_str(status).unwrap(),
        })
        .collect();

    Ok(Json(UserAssignments {
        assignments: assigned_routines,
    }))
}

#[tracing::instrument(skip(state))]
pub async fn assignment_accept(
    State(state): State<AppState>,
    Path(params): Path<PutAssignmentsParams>,
) -> Result<NoContent, AppError> {
    update_assignment_status(state, &params, Status::Assigned).await
}

#[tracing::instrument(skip(state))]
pub async fn assignment_start(
    State(state): State<AppState>,
    Path(params): Path<PutAssignmentsParams>,
) -> Result<NoContent, AppError> {
    update_assignment_status(state, &params, Status::Started).await
}

#[tracing::instrument(skip(state))]
pub async fn assignment_complete(
    State(state): State<AppState>,
    Path(params): Path<PutAssignmentsParams>,
) -> Result<NoContent, AppError> {
    update_assignment_status(state, &params, Status::Completed).await
}

async fn update_assignment_status(
    state: AppState,
    params: &PutAssignmentsParams,
    new_status: Status,
) -> Result<NoContent, AppError> {
    let user_id = &params.user_id;
    let routine_id = &params.routine_id;
    let status = new_status.to_string();

    state
        .redis_pool
        .get()
        .await?
        .hset(DB_KEY_PREFIX.to_owned() + &*user_id, routine_id, status)
        .await?;

    Ok(NoContent)
}
