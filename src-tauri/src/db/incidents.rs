use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incident {
    pub id: i64,
    pub case_number: Option<i64>,
    pub establishment_id: i64,
    pub location_id: Option<i64>,

    pub employee_name: String,
    pub employee_job_title: Option<String>,
    pub employee_address: Option<String>,
    pub employee_city: Option<String>,
    pub employee_state: Option<String>,
    pub employee_zip: Option<String>,
    pub employee_dob: Option<String>,
    pub employee_hire_date: Option<String>,
    pub employee_gender: Option<String>,
    pub is_privacy_case: bool,

    pub incident_date: String,
    pub incident_time: Option<String>,
    pub work_start_time: Option<String>,
    pub where_occurred: Option<String>,
    pub description: String,

    pub activity_before_incident: Option<String>,
    pub how_injury_occurred: Option<String>,
    pub injury_description: Option<String>,
    pub object_substance: Option<String>,

    pub physician_name: Option<String>,
    pub treatment_facility: Option<String>,
    pub facility_address: Option<String>,
    pub facility_city_state_zip: Option<String>,
    pub treated_in_er: Option<bool>,
    pub hospitalized_overnight: Option<bool>,

    pub outcome_severity: String,
    pub days_away_count: i64,
    pub days_restricted_count: i64,
    pub date_of_death: Option<String>,

    pub injury_illness_type: String,

    pub is_recordable: bool,
    pub status: String,
    pub completed_by: Option<String>,
    pub completed_by_title: Option<String>,
    pub completed_by_phone: Option<String>,
    pub completed_date: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateIncident {
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub employee_name: String,
    pub employee_job_title: Option<String>,
    pub employee_address: Option<String>,
    pub employee_city: Option<String>,
    pub employee_state: Option<String>,
    pub employee_zip: Option<String>,
    pub employee_dob: Option<String>,
    pub employee_hire_date: Option<String>,
    pub employee_gender: Option<String>,
    pub is_privacy_case: Option<bool>,
    pub incident_date: String,
    pub incident_time: Option<String>,
    pub work_start_time: Option<String>,
    pub where_occurred: Option<String>,
    pub description: String,
    pub activity_before_incident: Option<String>,
    pub how_injury_occurred: Option<String>,
    pub injury_description: Option<String>,
    pub object_substance: Option<String>,
    pub physician_name: Option<String>,
    pub treatment_facility: Option<String>,
    pub facility_address: Option<String>,
    pub facility_city_state_zip: Option<String>,
    pub treated_in_er: Option<bool>,
    pub hospitalized_overnight: Option<bool>,
    pub outcome_severity: Option<String>,
    pub days_away_count: Option<i64>,
    pub days_restricted_count: Option<i64>,
    pub date_of_death: Option<String>,
    pub injury_illness_type: Option<String>,
    pub is_recordable: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIncident {
    pub location_id: Option<i64>,
    pub employee_name: Option<String>,
    pub employee_job_title: Option<String>,
    pub employee_address: Option<String>,
    pub employee_city: Option<String>,
    pub employee_state: Option<String>,
    pub employee_zip: Option<String>,
    pub employee_dob: Option<String>,
    pub employee_hire_date: Option<String>,
    pub employee_gender: Option<String>,
    pub is_privacy_case: Option<bool>,
    pub incident_date: Option<String>,
    pub incident_time: Option<String>,
    pub work_start_time: Option<String>,
    pub where_occurred: Option<String>,
    pub description: Option<String>,
    pub activity_before_incident: Option<String>,
    pub how_injury_occurred: Option<String>,
    pub injury_description: Option<String>,
    pub object_substance: Option<String>,
    pub physician_name: Option<String>,
    pub treatment_facility: Option<String>,
    pub facility_address: Option<String>,
    pub facility_city_state_zip: Option<String>,
    pub treated_in_er: Option<bool>,
    pub hospitalized_overnight: Option<bool>,
    pub outcome_severity: Option<String>,
    pub days_away_count: Option<i64>,
    pub days_restricted_count: Option<i64>,
    pub date_of_death: Option<String>,
    pub injury_illness_type: Option<String>,
    pub is_recordable: Option<bool>,
    pub status: Option<String>,
    pub completed_by: Option<String>,
    pub completed_by_title: Option<String>,
    pub completed_by_phone: Option<String>,
    pub completed_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IncidentFilter {
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub status: Option<String>,
    pub outcome_severity: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
}

fn next_case_number(conn: &Connection, establishment_id: i64, year: &str) -> Result<i64> {
    let max: Option<i64> = conn
        .query_row(
            "SELECT MAX(case_number) FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2",
            params![establishment_id, format!("{year}%")],
            |row| row.get(0),
        )
        .context("Failed to query max case number")?;

    Ok(max.unwrap_or(0) + 1)
}

fn extract_incident_year(incident_date: &str) -> Result<&str> {
    let year = incident_date
        .get(0..4)
        .context("Incident date must start with a 4-digit year")?;

    if !year.chars().all(|c| c.is_ascii_digit()) {
        anyhow::bail!("Incident date must start with a numeric year: {incident_date}");
    }

    Ok(year)
}

fn row_to_incident(row: &rusqlite::Row<'_>) -> rusqlite::Result<Incident> {
    Ok(Incident {
        id: row.get(0)?,
        case_number: row.get(1)?,
        establishment_id: row.get(2)?,
        location_id: row.get(3)?,
        employee_name: row.get(4)?,
        employee_job_title: row.get(5)?,
        employee_address: row.get(6)?,
        employee_city: row.get(7)?,
        employee_state: row.get(8)?,
        employee_zip: row.get(9)?,
        employee_dob: row.get(10)?,
        employee_hire_date: row.get(11)?,
        employee_gender: row.get(12)?,
        is_privacy_case: row.get::<_, i32>(13)? != 0,
        incident_date: row.get(14)?,
        incident_time: row.get(15)?,
        work_start_time: row.get(16)?,
        where_occurred: row.get(17)?,
        description: row.get(18)?,
        activity_before_incident: row.get(19)?,
        how_injury_occurred: row.get(20)?,
        injury_description: row.get(21)?,
        object_substance: row.get(22)?,
        physician_name: row.get(23)?,
        treatment_facility: row.get(24)?,
        facility_address: row.get(25)?,
        facility_city_state_zip: row.get(26)?,
        treated_in_er: row.get::<_, Option<i32>>(27)?.map(|v| v != 0),
        hospitalized_overnight: row.get::<_, Option<i32>>(28)?.map(|v| v != 0),
        outcome_severity: row.get(29)?,
        days_away_count: row.get(30)?,
        days_restricted_count: row.get(31)?,
        date_of_death: row.get(32)?,
        injury_illness_type: row.get(33)?,
        is_recordable: row.get::<_, i32>(34)? != 0,
        status: row.get(35)?,
        completed_by: row.get(36)?,
        completed_by_title: row.get(37)?,
        completed_by_phone: row.get(38)?,
        completed_date: row.get(39)?,
        created_at: row.get(40)?,
        updated_at: row.get(41)?,
    })
}

const SELECT_COLS: &str = "id, case_number, establishment_id, location_id,
    employee_name, employee_job_title, employee_address, employee_city,
    employee_state, employee_zip, employee_dob, employee_hire_date,
    employee_gender, is_privacy_case,
    incident_date, incident_time, work_start_time, where_occurred, description,
    activity_before_incident, how_injury_occurred, injury_description, object_substance,
    physician_name, treatment_facility, facility_address, facility_city_state_zip,
    treated_in_er, hospitalized_overnight,
    outcome_severity, days_away_count, days_restricted_count, date_of_death,
    injury_illness_type, is_recordable, status,
    completed_by, completed_by_title, completed_by_phone, completed_date,
    created_at, updated_at";

pub fn create_incident(conn: &Connection, data: CreateIncident) -> Result<Incident> {
    let year = extract_incident_year(&data.incident_date)?;
    let case_num = next_case_number(conn, data.establishment_id, year)?;

    conn.execute(
        "INSERT INTO incidents (
            case_number, establishment_id, location_id,
            employee_name, employee_job_title, employee_address, employee_city,
            employee_state, employee_zip, employee_dob, employee_hire_date,
            employee_gender, is_privacy_case,
            incident_date, incident_time, work_start_time, where_occurred, description,
            activity_before_incident, how_injury_occurred, injury_description, object_substance,
            physician_name, treatment_facility, facility_address, facility_city_state_zip,
            treated_in_er, hospitalized_overnight,
            outcome_severity, days_away_count, days_restricted_count, date_of_death,
            injury_illness_type, is_recordable
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26,
            ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34
        )",
        params![
            case_num,
            data.establishment_id,
            data.location_id,
            data.employee_name,
            data.employee_job_title,
            data.employee_address,
            data.employee_city,
            data.employee_state,
            data.employee_zip,
            data.employee_dob,
            data.employee_hire_date,
            data.employee_gender,
            data.is_privacy_case.unwrap_or(false) as i32,
            data.incident_date,
            data.incident_time,
            data.work_start_time,
            data.where_occurred,
            data.description,
            data.activity_before_incident,
            data.how_injury_occurred,
            data.injury_description,
            data.object_substance,
            data.physician_name,
            data.treatment_facility,
            data.facility_address,
            data.facility_city_state_zip,
            data.treated_in_er.map(|v| v as i32),
            data.hospitalized_overnight.map(|v| v as i32),
            data.outcome_severity
                .as_deref()
                .unwrap_or("other_recordable"),
            data.days_away_count.unwrap_or(0),
            data.days_restricted_count.unwrap_or(0),
            data.date_of_death,
            data.injury_illness_type.as_deref().unwrap_or("injury"),
            data.is_recordable.unwrap_or(true) as i32,
        ],
    )
    .context("Failed to create incident")?;

    let id = conn.last_insert_rowid();
    get_incident(conn, id)
}

pub fn get_incident(conn: &Connection, id: i64) -> Result<Incident> {
    conn.query_row(
        &format!("SELECT {SELECT_COLS} FROM incidents WHERE id = ?1"),
        [id],
        row_to_incident,
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Incident {id} not found")).into()
        }
        _ => anyhow::Error::new(e),
    })
}

pub fn list_incidents(conn: &Connection, filter: IncidentFilter) -> Result<Vec<Incident>> {
    let mut sql = format!("SELECT {SELECT_COLS} FROM incidents WHERE establishment_id = ?");
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(filter.establishment_id)];

    if let Some(ref loc_id) = filter.location_id {
        sql.push_str(" AND location_id = ?");
        values.push(Box::new(*loc_id));
    }
    if let Some(ref status) = filter.status {
        sql.push_str(" AND status = ?");
        values.push(Box::new(status.clone()));
    }
    if let Some(ref sev) = filter.outcome_severity {
        sql.push_str(" AND outcome_severity = ?");
        values.push(Box::new(sev.clone()));
    }
    if let Some(ref from) = filter.date_from {
        sql.push_str(" AND incident_date >= ?");
        values.push(Box::new(from.clone()));
    }
    if let Some(ref to) = filter.date_to {
        sql.push_str(" AND incident_date <= ?");
        values.push(Box::new(to.clone()));
    }
    if let Some(ref search) = filter.search {
        sql.push_str(" AND (employee_name LIKE ? OR description LIKE ?)");
        let pat = format!("%{search}%");
        values.push(Box::new(pat.clone()));
        values.push(Box::new(pat));
    }

    sql.push_str(" ORDER BY incident_date DESC, id DESC");

    let mut stmt = conn.prepare(&sql)?;
    let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let rows = stmt
        .query_map(params.as_slice(), row_to_incident)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn update_incident(conn: &Connection, id: i64, data: UpdateIncident) -> Result<Incident> {
    let _existing = get_incident(conn, id)?;

    // Build dynamic SET clause
    macro_rules! push_field {
        ($sets:ident, $vals:ident, $field:expr, $col:literal) => {
            if let Some(ref v) = $field {
                $sets.push(concat!($col, " = ?"));
                $vals.push(Box::new(v.clone()) as Box<dyn rusqlite::types::ToSql>);
            }
        };
    }

    macro_rules! push_bool_field {
        ($sets:ident, $vals:ident, $field:expr, $col:literal) => {
            if let Some(v) = $field {
                $sets.push(concat!($col, " = ?"));
                $vals.push(Box::new(v as i32) as Box<dyn rusqlite::types::ToSql>);
            }
        };
    }

    macro_rules! push_i64_field {
        ($sets:ident, $vals:ident, $field:expr, $col:literal) => {
            if let Some(v) = $field {
                $sets.push(concat!($col, " = ?"));
                $vals.push(Box::new(v) as Box<dyn rusqlite::types::ToSql>);
            }
        };
    }

    let mut sets: Vec<&str> = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    push_i64_field!(sets, values, data.location_id, "location_id");
    push_field!(sets, values, data.employee_name, "employee_name");
    push_field!(sets, values, data.employee_job_title, "employee_job_title");
    push_field!(sets, values, data.employee_address, "employee_address");
    push_field!(sets, values, data.employee_city, "employee_city");
    push_field!(sets, values, data.employee_state, "employee_state");
    push_field!(sets, values, data.employee_zip, "employee_zip");
    push_field!(sets, values, data.employee_dob, "employee_dob");
    push_field!(sets, values, data.employee_hire_date, "employee_hire_date");
    push_field!(sets, values, data.employee_gender, "employee_gender");
    push_bool_field!(sets, values, data.is_privacy_case, "is_privacy_case");
    push_field!(sets, values, data.incident_date, "incident_date");
    push_field!(sets, values, data.incident_time, "incident_time");
    push_field!(sets, values, data.work_start_time, "work_start_time");
    push_field!(sets, values, data.where_occurred, "where_occurred");
    push_field!(sets, values, data.description, "description");
    push_field!(
        sets,
        values,
        data.activity_before_incident,
        "activity_before_incident"
    );
    push_field!(
        sets,
        values,
        data.how_injury_occurred,
        "how_injury_occurred"
    );
    push_field!(sets, values, data.injury_description, "injury_description");
    push_field!(sets, values, data.object_substance, "object_substance");
    push_field!(sets, values, data.physician_name, "physician_name");
    push_field!(sets, values, data.treatment_facility, "treatment_facility");
    push_field!(sets, values, data.facility_address, "facility_address");
    push_field!(
        sets,
        values,
        data.facility_city_state_zip,
        "facility_city_state_zip"
    );
    push_bool_field!(sets, values, data.treated_in_er, "treated_in_er");
    push_bool_field!(
        sets,
        values,
        data.hospitalized_overnight,
        "hospitalized_overnight"
    );
    push_field!(sets, values, data.outcome_severity, "outcome_severity");
    push_i64_field!(sets, values, data.days_away_count, "days_away_count");
    push_i64_field!(
        sets,
        values,
        data.days_restricted_count,
        "days_restricted_count"
    );
    push_field!(sets, values, data.date_of_death, "date_of_death");
    push_field!(
        sets,
        values,
        data.injury_illness_type,
        "injury_illness_type"
    );
    push_bool_field!(sets, values, data.is_recordable, "is_recordable");
    push_field!(sets, values, data.status, "status");
    push_field!(sets, values, data.completed_by, "completed_by");
    push_field!(sets, values, data.completed_by_title, "completed_by_title");
    push_field!(sets, values, data.completed_by_phone, "completed_by_phone");
    push_field!(sets, values, data.completed_date, "completed_date");

    if !sets.is_empty() {
        sets.push("updated_at = datetime('now')");
        let sql = format!("UPDATE incidents SET {} WHERE id = ?", sets.join(", "));
        values.push(Box::new(id));
        let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        conn.execute(&sql, params.as_slice())
            .context("Failed to update incident")?;
    }

    get_incident(conn, id)
}

pub fn delete_incident(conn: &Connection, id: i64) -> Result<()> {
    let changes = conn
        .execute("DELETE FROM incidents WHERE id = ?1", [id])
        .context("Failed to delete incident")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("Incident {id} not found")).into());
    }
    Ok(())
}

// ── Attachments ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: i64,
    pub incident_id: i64,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    pub file_size: Option<i64>,
    pub created_at: String,
}

pub fn add_attachment(
    conn: &Connection,
    incident_id: i64,
    file_name: &str,
    file_path: &str,
    file_type: &str,
    file_size: Option<i64>,
) -> Result<Attachment> {
    conn.execute(
        "INSERT INTO attachments (incident_id, file_name, file_path, file_type, file_size)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![incident_id, file_name, file_path, file_type, file_size],
    )
    .context("Failed to add attachment")?;

    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, incident_id, file_name, file_path, file_type, file_size, created_at
         FROM attachments WHERE id = ?1",
        [id],
        |row| {
            Ok(Attachment {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                file_name: row.get(2)?,
                file_path: row.get(3)?,
                file_type: row.get(4)?,
                file_size: row.get(5)?,
                created_at: row.get(6)?,
            })
        },
    )
    .map_err(anyhow::Error::new)
}

pub fn list_attachments(conn: &Connection, incident_id: i64) -> Result<Vec<Attachment>> {
    let mut stmt = conn.prepare(
        "SELECT id, incident_id, file_name, file_path, file_type, file_size, created_at
         FROM attachments WHERE incident_id = ?1 ORDER BY created_at",
    )?;

    let rows = stmt
        .query_map([incident_id], |row| {
            Ok(Attachment {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                file_name: row.get(2)?,
                file_path: row.get(3)?,
                file_type: row.get(4)?,
                file_size: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn delete_attachment(conn: &Connection, id: i64) -> Result<()> {
    let changes = conn
        .execute("DELETE FROM attachments WHERE id = ?1", [id])
        .context("Failed to delete attachment")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("Attachment {id} not found")).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::locations::{
        create_establishment, create_location, CreateEstablishment, CreateLocation,
    };
    use crate::db::open_test_db;

    fn setup_test_data(conn: &Connection) -> (i64, i64) {
        let est = create_establishment(
            conn,
            CreateEstablishment {
                name: "Test Co".into(),
                street_address: None,
                city: None,
                state: None,
                zip_code: None,
                industry_description: None,
                naics_code: None,
            },
        )
        .unwrap();
        let loc = create_location(
            conn,
            CreateLocation {
                establishment_id: est.id,
                name: "Site A".into(),
                address: None,
                city: None,
                state: None,
            },
        )
        .unwrap();
        (est.id, loc.id)
    }

    fn make_incident(est_id: i64, loc_id: i64) -> CreateIncident {
        CreateIncident {
            establishment_id: est_id,
            location_id: Some(loc_id),
            employee_name: "John Doe".into(),
            employee_job_title: Some("Carpenter".into()),
            employee_address: None,
            employee_city: None,
            employee_state: None,
            employee_zip: None,
            employee_dob: None,
            employee_hire_date: None,
            employee_gender: Some("male".into()),
            is_privacy_case: None,
            incident_date: "2026-01-15".into(),
            incident_time: Some("09:30".into()),
            work_start_time: Some("07:00".into()),
            where_occurred: Some("Building B, 2nd floor".into()),
            description: "Worker fell from scaffolding".into(),
            activity_before_incident: Some("Installing drywall".into()),
            how_injury_occurred: Some("Lost balance on scaffold".into()),
            injury_description: Some("Fractured left wrist".into()),
            object_substance: Some("Scaffold platform".into()),
            physician_name: None,
            treatment_facility: None,
            facility_address: None,
            facility_city_state_zip: None,
            treated_in_er: Some(true),
            hospitalized_overnight: Some(false),
            outcome_severity: Some("days_away".into()),
            days_away_count: Some(14),
            days_restricted_count: Some(0),
            date_of_death: None,
            injury_illness_type: Some("injury".into()),
            is_recordable: Some(true),
        }
    }

    #[test]
    fn test_incident_crud() {
        let conn = open_test_db();
        let (est_id, loc_id) = setup_test_data(&conn);

        let inc = create_incident(&conn, make_incident(est_id, loc_id)).unwrap();
        assert_eq!(inc.case_number, Some(1));
        assert_eq!(inc.employee_name, "John Doe");
        assert_eq!(inc.outcome_severity, "days_away");

        // Second incident gets case_number 2
        let inc2 = create_incident(&conn, make_incident(est_id, loc_id)).unwrap();
        assert_eq!(inc2.case_number, Some(2));

        let fetched = get_incident(&conn, inc.id).unwrap();
        assert_eq!(fetched.description, "Worker fell from scaffolding");

        let updated = update_incident(
            &conn,
            inc.id,
            UpdateIncident {
                status: Some("closed".into()),
                description: Some("Updated description".into()),
                location_id: None,
                employee_name: None,
                employee_job_title: None,
                employee_address: None,
                employee_city: None,
                employee_state: None,
                employee_zip: None,
                employee_dob: None,
                employee_hire_date: None,
                employee_gender: None,
                is_privacy_case: None,
                incident_date: None,
                incident_time: None,
                work_start_time: None,
                where_occurred: None,
                activity_before_incident: None,
                how_injury_occurred: None,
                injury_description: None,
                object_substance: None,
                physician_name: None,
                treatment_facility: None,
                facility_address: None,
                facility_city_state_zip: None,
                treated_in_er: None,
                hospitalized_overnight: None,
                outcome_severity: None,
                days_away_count: None,
                days_restricted_count: None,
                date_of_death: None,
                injury_illness_type: None,
                is_recordable: None,
                completed_by: None,
                completed_by_title: None,
                completed_by_phone: None,
                completed_date: None,
            },
        )
        .unwrap();
        assert_eq!(updated.status, "closed");
        assert_eq!(updated.description, "Updated description");

        delete_incident(&conn, inc.id).unwrap();
        assert!(get_incident(&conn, inc.id).is_err());
    }

    #[test]
    fn test_incident_filtering() {
        let conn = open_test_db();
        let (est_id, loc_id) = setup_test_data(&conn);

        create_incident(&conn, make_incident(est_id, loc_id)).unwrap();

        let all = list_incidents(
            &conn,
            IncidentFilter {
                establishment_id: est_id,
                location_id: None,
                status: None,
                outcome_severity: None,
                date_from: None,
                date_to: None,
                search: None,
            },
        )
        .unwrap();
        assert_eq!(all.len(), 1);

        let filtered = list_incidents(
            &conn,
            IncidentFilter {
                establishment_id: est_id,
                location_id: None,
                status: Some("closed".into()),
                outcome_severity: None,
                date_from: None,
                date_to: None,
                search: None,
            },
        )
        .unwrap();
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_create_incident_rejects_invalid_date_prefix() {
        let conn = open_test_db();
        let (est_id, loc_id) = setup_test_data(&conn);

        let mut incident = make_incident(est_id, loc_id);
        incident.incident_date = "bad-date".to_string();

        let result = create_incident(&conn, incident);
        assert!(result.is_err());
    }
}
