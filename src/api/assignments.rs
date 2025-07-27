use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::NoContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static ASSIGNMENTS_DB: LazyLock<Mutex<HashMap<UserId, HashMap<RoutineId, Assignment>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

type UserId = String;
type RoutineId = String;

#[derive(Clone, Serialize)]
pub struct Assignment {
    user_id: UserId,
    routine_id: RoutineId,
    status: Status,
}

#[derive(Clone, Serialize)]
pub enum Status {
    Assigned,
    Started,
    Completed,
}

#[derive(Deserialize)]
pub struct PutAssignmentsParams {
    user_id: UserId,
    routine_id: RoutineId,
}

#[derive(Serialize)]
pub struct UserAssignments {
    assignments: Vec<Assignment>,
}

pub async fn get_user_assignments(
    Path(user_id): Path<UserId>,
) -> Result<Json<UserAssignments>, StatusCode> {
    let db = ASSIGNMENTS_DB.lock().unwrap();

    let maybe_assigned_routines = db.get(&user_id);
    if maybe_assigned_routines.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let assigned_routines = maybe_assigned_routines.unwrap().values().cloned().collect();

    Ok(Json(UserAssignments {
        assignments: assigned_routines,
    }))
}

pub async fn assignment_accept(Path(params): Path<PutAssignmentsParams>) -> NoContent {
    let mut db = ASSIGNMENTS_DB.lock().unwrap();

    if let Some(assigned_routines) = db.get_mut(&params.user_id) {
        assigned_routines.insert(
            params.routine_id.clone(),
            Assignment {
                user_id: params.user_id,
                routine_id: params.routine_id,
                status: Status::Assigned,
            },
        );
    } else {
        let mut assigned_routines = HashMap::new();
        assigned_routines.insert(
            params.routine_id.clone(),
            Assignment {
                user_id: params.user_id.clone(),
                routine_id: params.routine_id,
                status: Status::Assigned,
            },
        );
        db.insert(params.user_id, assigned_routines);
    }

    NoContent
}

pub async fn assignment_start(
    Path(params): Path<PutAssignmentsParams>,
) -> Result<NoContent, StatusCode> {
    update_assignment_status(&params, Status::Started).unwrap_or_else(|value| value)
}

pub async fn assignment_complete(
    Path(params): Path<PutAssignmentsParams>,
) -> Result<NoContent, StatusCode> {
    update_assignment_status(&params, Status::Completed).unwrap_or_else(|value| value)
}

fn update_assignment_status(
    params: &PutAssignmentsParams,
    new_status: Status,
) -> Result<Result<NoContent, StatusCode>, Result<NoContent, StatusCode>> {
    let mut db = ASSIGNMENTS_DB.lock().unwrap();

    if let Some(assigned_routines) = db.get_mut(&params.user_id) {
        if let Some(assignment) = assigned_routines.get_mut(&params.routine_id) {
            let assignment = assignment.clone();
            assigned_routines.insert(
                params.routine_id.clone(),
                Assignment {
                    status: new_status,
                    ..assignment
                },
            );
        } else {
            return Err(Err(StatusCode::NOT_FOUND));
        }
    } else {
        return Err(Err(StatusCode::NOT_FOUND));
    }

    Ok(Ok(NoContent))
}
