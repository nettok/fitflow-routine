use crate::api::routines::Intensity::Set;
use crate::api::routines::TrainingGoal::Strength;
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::LazyLock;
use strum::VariantArray;

static ROUTINES_DB: LazyLock<HashMap<TrainingGoal, Routines>> = LazyLock::new(|| {
    let strength_routines = Routines {
        routines: vec![Routine {
            id: "str001".to_owned(),
            exercises: vec![Exercise {
                name: "Squat".to_owned(),
                intensity: Set(SetDetails {
                    sets: 3,
                    reps: 10,
                    load: Load::High,
                }),
            }],
        }],
    };

    let mut routines = HashMap::new();
    routines.insert(Strength, strength_routines);
    routines
});

#[derive(Clone, Eq, PartialEq, Hash, Serialize, VariantArray)]
pub enum TrainingGoal {
    Strength,
    Hypertrophy,
    Endurance,
    Power,
    FatLoss,
}

#[derive(Clone, Serialize)]
pub struct Routines {
    routines: Vec<Routine>,
}

#[derive(Clone, Serialize)]
pub struct Routine {
    id: String,
    exercises: Vec<Exercise>,
}

#[derive(Clone, Serialize)]
pub struct Exercise {
    name: String,
    intensity: Intensity,
}

#[derive(Clone, Serialize)]
pub enum Intensity {
    Set(SetDetails),
}

#[derive(Clone, Serialize)]
pub struct SetDetails {
    sets: u16,
    reps: u16,
    load: Load,
}

#[derive(Clone, Serialize)]
pub enum Load {
    Low,
    Medium,
    High,
}

#[derive(Clone, Serialize)]
pub struct Goals {
    goals: Vec<TrainingGoal>,
}

#[tracing::instrument]
pub async fn get_routines_by_goal() -> Json<HashMap<TrainingGoal, Routines>> {
    Json(ROUTINES_DB.clone())
}

#[tracing::instrument]
pub async fn get_goals() -> Json<Goals> {
    Json(Goals {
        goals: TrainingGoal::VARIANTS.to_vec(),
    })
}
