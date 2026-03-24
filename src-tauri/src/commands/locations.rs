use crate::db::locations::{
    self, CreateEstablishment, CreateLocation, Establishment, Location, UpdateEstablishment,
    UpdateLocation,
};
use crate::errors::AppError;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

#[tauri::command]
pub fn create_establishment(
    db: State<'_, DbState>,
    data: CreateEstablishment,
) -> Result<Establishment, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::create_establishment(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_establishment(db: State<'_, DbState>, id: i64) -> Result<Establishment, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::get_establishment(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_establishments(db: State<'_, DbState>) -> Result<Vec<Establishment>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::list_establishments(&conn).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_establishment(
    db: State<'_, DbState>,
    id: i64,
    data: UpdateEstablishment,
) -> Result<Establishment, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::update_establishment(&conn, id, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_establishment(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::delete_establishment(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn create_location(db: State<'_, DbState>, data: CreateLocation) -> Result<Location, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::create_location(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_location(db: State<'_, DbState>, id: i64) -> Result<Location, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::get_location(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_locations(
    db: State<'_, DbState>,
    establishment_id: i64,
) -> Result<Vec<Location>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::list_locations(&conn, establishment_id)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn update_location(
    db: State<'_, DbState>,
    id: i64,
    data: UpdateLocation,
) -> Result<Location, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::update_location(&conn, id, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_location(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    locations::delete_location(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}
