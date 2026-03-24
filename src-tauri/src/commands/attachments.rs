use crate::db::incidents::{self, Attachment};
use crate::errors::AppError;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

type DbState = Mutex<Connection>;

#[tauri::command]
pub fn add_attachment(
    db: State<'_, DbState>,
    incident_id: i64,
    file_name: String,
    file_path: String,
    file_type: String,
    file_size: Option<i64>,
) -> Result<Attachment, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::add_attachment(
        &conn,
        incident_id,
        &file_name,
        &file_path,
        &file_type,
        file_size,
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn list_attachments(
    db: State<'_, DbState>,
    incident_id: i64,
) -> Result<Vec<Attachment>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::list_attachments(&conn, incident_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn delete_attachment(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::delete_attachment(&conn, id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub async fn upload_attachment(
    app: AppHandle,
    db: State<'_, DbState>,
    incident_id: i64,
    source_path: String,
    file_type: String,
) -> Result<Attachment, AppError> {
    use crate::validation;

    // Validate file type
    let allowed_types = ["photo", "audio", "document"];
    if !allowed_types.contains(&file_type.as_str()) {
        return Err(AppError::Validation(format!(
            "Invalid file type: {}. Must be one of: photo, audio, document",
            file_type
        )));
    }

    // Get source path and extract filename
    let source = PathBuf::from(&source_path);
    let file_name = source
        .file_name()
        .ok_or_else(|| AppError::Validation("Invalid file path".to_string()))?
        .to_string_lossy()
        .to_string();

    // Sanitize filename
    let safe_filename = validation::sanitize_filename(&file_name);
    if safe_filename.is_empty() {
        return Err(AppError::Validation(
            "Filename would be empty after sanitization".to_string(),
        ));
    }

    // Get file size
    let metadata = fs::metadata(&source)
        .map_err(|e| AppError::Internal(format!("Failed to read file metadata: {}", e)))?;
    let file_size = metadata.len() as i64;

    // Validate file size (max 50MB)
    const MAX_FILE_SIZE: i64 = 50 * 1024 * 1024;
    if file_size > MAX_FILE_SIZE {
        return Err(AppError::Validation(format!(
            "File size {} bytes exceeds maximum of {} bytes (50MB)",
            file_size, MAX_FILE_SIZE
        )));
    }

    // Create attachments directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(format!("Failed to get app data dir: {}", e)))?;
    let attachments_dir = app_data_dir.join("attachments");
    fs::create_dir_all(&attachments_dir).map_err(|e| {
        AppError::Internal(format!("Failed to create attachments directory: {}", e))
    })?;

    // Generate unique filename with incident ID
    let unique_filename = format!("{}_{}", incident_id, safe_filename);
    let dest_path = attachments_dir.join(&unique_filename);

    // Copy file to attachments directory
    fs::copy(&source, &dest_path)
        .map_err(|e| AppError::Internal(format!("Failed to copy file: {}", e)))?;

    // Store relative path in database
    let relative_path = format!("attachments/{}", unique_filename);

    // Add to database
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::add_attachment(
        &conn,
        incident_id,
        &file_name,
        &relative_path,
        &file_type,
        Some(file_size),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}
