use crate::db::rca::{
    self, CorrectiveAction, CreateCorrectiveAction, CreateFishboneCategory, CreateFishboneCause,
    CreateFiveWhysStep, CreateRcaSession, FishboneCategory, FishboneCause, FiveWhysStep,
    RcaSession, UpdateCorrectiveAction,
};
use crate::errors::AppError;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

// ── RCA Sessions ──

#[tauri::command]
pub fn create_rca_session(
    db: State<'_, DbState>,
    data: CreateRcaSession,
) -> Result<RcaSession, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::create_rca_session(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_rca_session(db: State<'_, DbState>, id: i64) -> Result<RcaSession, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::get_rca_session(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_rca_sessions(
    db: State<'_, DbState>,
    incident_id: i64,
) -> Result<Vec<RcaSession>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::list_rca_sessions(&conn, incident_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn complete_rca_session(
    db: State<'_, DbState>,
    id: i64,
    root_cause_summary: String,
) -> Result<RcaSession, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::complete_rca_session(&conn, id, &root_cause_summary)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_rca_session(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::delete_rca_session(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

// ── Five Whys ──

#[tauri::command]
pub fn add_five_whys_step(
    db: State<'_, DbState>,
    data: CreateFiveWhysStep,
) -> Result<FiveWhysStep, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::add_five_whys_step(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_five_whys_steps(
    db: State<'_, DbState>,
    rca_session_id: i64,
) -> Result<Vec<FiveWhysStep>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::list_five_whys_steps(&conn, rca_session_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_five_whys_step(
    db: State<'_, DbState>,
    id: i64,
    question: String,
    answer: String,
) -> Result<FiveWhysStep, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::update_five_whys_step(&conn, id, &question, &answer)
        .map_err(|e| AppError::Internal(e.to_string()))
}

// ── Fishbone ──

#[tauri::command]
pub fn add_fishbone_category(
    db: State<'_, DbState>,
    data: CreateFishboneCategory,
) -> Result<FishboneCategory, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::add_fishbone_category(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_fishbone_categories(
    db: State<'_, DbState>,
    rca_session_id: i64,
) -> Result<Vec<FishboneCategory>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::list_fishbone_categories(&conn, rca_session_id)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn add_fishbone_cause(
    db: State<'_, DbState>,
    data: CreateFishboneCause,
) -> Result<FishboneCause, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::add_fishbone_cause(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_fishbone_cause(
    db: State<'_, DbState>,
    id: i64,
    cause_text: Option<String>,
    is_root_cause: Option<bool>,
) -> Result<FishboneCause, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::update_fishbone_cause(&conn, id, cause_text.as_deref(), is_root_cause)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_fishbone_cause(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::delete_fishbone_cause(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

// ── Corrective Actions ──

#[tauri::command]
pub fn create_corrective_action(
    db: State<'_, DbState>,
    data: CreateCorrectiveAction,
) -> Result<CorrectiveAction, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::create_corrective_action(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_corrective_actions(
    db: State<'_, DbState>,
    incident_id: i64,
) -> Result<Vec<CorrectiveAction>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::list_corrective_actions(&conn, incident_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_corrective_action(
    db: State<'_, DbState>,
    id: i64,
    data: UpdateCorrectiveAction,
) -> Result<CorrectiveAction, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::update_corrective_action(&conn, id, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_corrective_action(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    rca::delete_corrective_action(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}
