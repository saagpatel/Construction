mod commands;
mod db;
mod errors;
mod validation;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_data = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to resolve app data directory: {}", e))?;

            std::fs::create_dir_all(&app_data)
                .map_err(|e| format!("Failed to create app data directory: {}", e))?;

            let db_path = app_data.join("safety_tracker.db");
            let conn = db::open_db(&db_path)
                .map_err(|e| format!("Failed to open database at {:?}: {}", db_path, e))?;

            app.manage(Mutex::new(conn));

            // Create attachments directory
            let attachments_dir = app_data.join("attachments");
            std::fs::create_dir_all(&attachments_dir)
                .map_err(|e| format!("Failed to create attachments directory: {}", e))?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Establishments
            commands::locations::create_establishment,
            commands::locations::get_establishment,
            commands::locations::list_establishments,
            commands::locations::update_establishment,
            commands::locations::delete_establishment,
            // Locations
            commands::locations::create_location,
            commands::locations::get_location,
            commands::locations::list_locations,
            commands::locations::update_location,
            commands::locations::delete_location,
            // Incidents
            commands::incidents::create_incident,
            commands::incidents::get_incident,
            commands::incidents::list_incidents,
            commands::incidents::update_incident,
            commands::incidents::delete_incident,
            // Attachments
            commands::attachments::add_attachment,
            commands::attachments::list_attachments,
            commands::attachments::delete_attachment,
            commands::attachments::upload_attachment,
            // RCA
            commands::rca::create_rca_session,
            commands::rca::get_rca_session,
            commands::rca::list_rca_sessions,
            commands::rca::complete_rca_session,
            commands::rca::delete_rca_session,
            commands::rca::add_five_whys_step,
            commands::rca::list_five_whys_steps,
            commands::rca::update_five_whys_step,
            commands::rca::add_fishbone_category,
            commands::rca::list_fishbone_categories,
            commands::rca::add_fishbone_cause,
            commands::rca::update_fishbone_cause,
            commands::rca::delete_fishbone_cause,
            commands::rca::create_corrective_action,
            commands::rca::list_corrective_actions,
            commands::rca::update_corrective_action,
            commands::rca::delete_corrective_action,
            // OSHA
            commands::osha::get_osha_300_log,
            commands::osha::get_osha_300a_summary,
            commands::osha::get_osha_301_report,
            commands::osha::export_osha_300_csv,
            commands::osha::export_osha_300a_csv,
            commands::osha::export_osha_301_csv,
            commands::osha::upsert_annual_stats,
            commands::osha::get_annual_stats,
            // Dashboard
            commands::dashboard::get_dashboard_summary,
            commands::dashboard::get_incidents_by_month,
            commands::dashboard::get_incidents_by_severity,
            commands::dashboard::get_incidents_by_location,
            commands::dashboard::get_incidents_by_type,
            commands::dashboard::get_corrective_action_summary,
            // Import
            commands::import::preview_csv,
            commands::import::import_csv,
            // Toolbox Talks
            commands::toolbox::list_toolbox_topics,
            commands::toolbox::get_toolbox_topic,
            commands::toolbox::create_toolbox_talk,
            commands::toolbox::get_toolbox_talk,
            commands::toolbox::list_toolbox_talks,
            commands::toolbox::complete_toolbox_talk,
            commands::toolbox::add_toolbox_attendee,
            commands::toolbox::list_toolbox_attendees,
            commands::toolbox::sign_toolbox_attendee,
            commands::toolbox::delete_toolbox_attendee,
            // JSA/JHA
            commands::jsa::list_jsa_templates,
            commands::jsa::create_jsa_instance,
            commands::jsa::get_jsa_instance,
            commands::jsa::list_jsa_instances,
            commands::jsa::update_jsa_status,
            commands::jsa::add_jsa_step,
            commands::jsa::list_jsa_steps,
            commands::jsa::toggle_jsa_step,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}
