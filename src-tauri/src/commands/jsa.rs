use crate::db::jsa::{self, AddJsaStep, CreateJsaInstance, JsaInstance, JsaStep, JsaTemplate};
use crate::errors::AppError;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

#[tauri::command]
pub fn list_jsa_templates(db: State<'_, DbState>) -> Result<Vec<JsaTemplate>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::list_templates(&conn).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn create_jsa_instance(
    db: State<'_, DbState>,
    data: CreateJsaInstance,
) -> Result<JsaInstance, AppError> {
    use crate::validation;

    validation::validate_not_empty(&data.job_name, "Job name")?;
    validation::validate_not_empty(&data.prepared_by, "Prepared by")?;
    validation::validate_date_format(&data.job_date, "Job date")?;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::create_instance(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_jsa_instance(db: State<'_, DbState>, id: i64) -> Result<JsaInstance, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::get_instance(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_jsa_instances(
    db: State<'_, DbState>,
    establishment_id: i64,
) -> Result<Vec<JsaInstance>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::list_instances(&conn, establishment_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_jsa_status(
    db: State<'_, DbState>,
    id: i64,
    status: String,
) -> Result<JsaInstance, AppError> {
    let valid_statuses = ["draft", "reviewed", "approved", "in_progress", "completed"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid status: {}. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        )));
    }

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::update_instance_status(&conn, id, &status).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn add_jsa_step(db: State<'_, DbState>, data: AddJsaStep) -> Result<JsaStep, AppError> {
    use crate::validation;

    validation::validate_not_empty(&data.task_description, "Task description")?;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::add_step(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_jsa_steps(
    db: State<'_, DbState>,
    jsa_instance_id: i64,
) -> Result<Vec<JsaStep>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::list_steps(&conn, jsa_instance_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn toggle_jsa_step(db: State<'_, DbState>, id: i64) -> Result<JsaStep, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    jsa::toggle_step_completion(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}
