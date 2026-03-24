use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

// ── OSHA 300 Log Row ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Osha300Row {
    pub case_number: i64,
    pub employee_name: String,
    pub job_title: String,
    pub incident_date: String,
    pub where_occurred: String,
    pub description: String,
    pub outcome_death: bool,
    pub outcome_days_away: bool,
    pub outcome_job_transfer: bool,
    pub outcome_other_recordable: bool,
    pub days_away_count: i64,
    pub days_restricted_count: i64,
    pub type_injury: bool,
    pub type_skin_disorder: bool,
    pub type_respiratory: bool,
    pub type_poisoning: bool,
    pub type_hearing_loss: bool,
    pub type_other_illness: bool,
}

// ── OSHA 300A Summary ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Osha300ASummary {
    pub year: i64,
    pub establishment_name: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub industry_description: String,
    pub naics_code: String,

    pub total_deaths: i64,
    pub total_days_away_cases: i64,
    pub total_transfer_restriction_cases: i64,
    pub total_other_recordable_cases: i64,
    pub total_days_away: i64,
    pub total_days_restricted: i64,

    pub total_injuries: i64,
    pub total_skin_disorders: i64,
    pub total_respiratory: i64,
    pub total_poisonings: i64,
    pub total_hearing_loss: i64,
    pub total_other_illnesses: i64,

    pub avg_employees: Option<i64>,
    pub total_hours_worked: Option<i64>,

    pub certifier_name: Option<String>,
    pub certifier_title: Option<String>,
    pub certifier_phone: Option<String>,
    pub certification_date: Option<String>,
}

// ── OSHA 301 Individual Report ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Osha301Report {
    pub case_number: i64,
    // Section A - Employee
    pub employee_name: String,
    pub employee_address: String,
    pub employee_city: String,
    pub employee_state: String,
    pub employee_zip: String,
    pub employee_dob: String,
    pub employee_hire_date: String,
    pub employee_gender: String,
    // Section B - Healthcare
    pub physician_name: String,
    pub treatment_facility: String,
    pub facility_address: String,
    pub facility_city_state_zip: String,
    pub treated_in_er: bool,
    pub hospitalized_overnight: bool,
    // Section C - Incident details
    pub incident_date: String,
    pub incident_time: String,
    pub work_start_time: String,
    pub where_occurred: String,
    pub activity_before_incident: String,
    pub how_injury_occurred: String,
    pub injury_description: String,
    pub object_substance: String,
    pub date_of_death: String,
    // Section D - Completed by
    pub completed_by: String,
    pub completed_by_title: String,
    pub completed_by_phone: String,
    pub completed_date: String,
}

// ── Annual Stats ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnnualStats {
    pub id: i64,
    pub establishment_id: i64,
    pub year: i64,
    pub avg_employees: i64,
    pub total_hours_worked: i64,
    pub certifier_name: Option<String>,
    pub certifier_title: Option<String>,
    pub certifier_phone: Option<String>,
    pub certification_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertAnnualStats {
    pub establishment_id: i64,
    pub year: i64,
    pub avg_employees: i64,
    pub total_hours_worked: i64,
    pub certifier_name: Option<String>,
    pub certifier_title: Option<String>,
    pub certifier_phone: Option<String>,
    pub certification_date: Option<String>,
}

// ── Queries ──

pub fn get_osha_300_log(
    conn: &Connection,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<Osha300Row>> {
    let year_str = format!("{year}%");
    let mut stmt = conn.prepare(
        "SELECT case_number, employee_name, employee_job_title,
                incident_date, where_occurred, description,
                outcome_severity, days_away_count, days_restricted_count,
                injury_illness_type, is_privacy_case
         FROM incidents
         WHERE establishment_id = ?1
           AND incident_date LIKE ?2
           AND is_recordable = 1
         ORDER BY case_number",
    )?;

    let rows = stmt
        .query_map(params![establishment_id, year_str], |row| {
            let severity: String = row.get(6)?;
            let illness_type: String = row.get(9)?;
            let is_privacy: bool = row.get::<_, i32>(10)? != 0;
            let name: String = row.get(1)?;

            Ok(Osha300Row {
                case_number: row.get::<_, Option<i64>>(0)?.unwrap_or(0),
                employee_name: if is_privacy {
                    "Privacy Case".to_string()
                } else {
                    name
                },
                job_title: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                incident_date: row.get(3)?,
                where_occurred: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                description: row.get(5)?,
                outcome_death: severity == "death",
                outcome_days_away: severity == "days_away",
                outcome_job_transfer: severity == "job_transfer_restriction",
                outcome_other_recordable: severity == "other_recordable",
                days_away_count: row.get(7)?,
                days_restricted_count: row.get(8)?,
                type_injury: illness_type == "injury",
                type_skin_disorder: illness_type == "skin_disorder",
                type_respiratory: illness_type == "respiratory",
                type_poisoning: illness_type == "poisoning",
                type_hearing_loss: illness_type == "hearing_loss",
                type_other_illness: illness_type == "other_illness",
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn get_osha_300a_summary(
    conn: &Connection,
    establishment_id: i64,
    year: i64,
) -> Result<Osha300ASummary> {
    let year_str = format!("{year}%");

    // Get establishment info
    let (est_name, street, city, state, zip, industry, naics): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = conn
        .query_row(
            "SELECT name, COALESCE(street_address,''), COALESCE(city,''),
                    COALESCE(state,''), COALESCE(zip_code,''),
                    COALESCE(industry_description,''), COALESCE(naics_code,'')
             FROM establishments WHERE id = ?1",
            [establishment_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::NotFound(format!("Establishment {establishment_id} not found"))
            }
            _ => AppError::Database(e),
        })?;

    // Aggregate incident data
    let (deaths, days_away_cases, transfer_cases, other_cases, total_days_away, total_days_restricted): (i64, i64, i64, i64, i64, i64) = conn
        .query_row(
            "SELECT
                COALESCE(SUM(CASE WHEN outcome_severity = 'death' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN outcome_severity = 'days_away' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN outcome_severity = 'job_transfer_restriction' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN outcome_severity = 'other_recordable' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(days_away_count), 0),
                COALESCE(SUM(days_restricted_count), 0)
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2 AND is_recordable = 1",
            params![establishment_id, year_str],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )?;

    let (injuries, skin, resp, poison, hearing, other_ill): (i64, i64, i64, i64, i64, i64) = conn
        .query_row(
        "SELECT
                COALESCE(SUM(CASE WHEN injury_illness_type = 'injury' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN injury_illness_type = 'skin_disorder' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN injury_illness_type = 'respiratory' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN injury_illness_type = 'poisoning' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN injury_illness_type = 'hearing_loss' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN injury_illness_type = 'other_illness' THEN 1 ELSE 0 END), 0)
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2 AND is_recordable = 1",
        params![establishment_id, year_str],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        },
    )?;

    // Get annual stats
    let stats: Option<AnnualStats> = conn
        .query_row(
            "SELECT id, establishment_id, year, avg_employees, total_hours_worked,
                    certifier_name, certifier_title, certifier_phone, certification_date
             FROM annual_stats WHERE establishment_id = ?1 AND year = ?2",
            params![establishment_id, year],
            |row| {
                Ok(AnnualStats {
                    id: row.get(0)?,
                    establishment_id: row.get(1)?,
                    year: row.get(2)?,
                    avg_employees: row.get(3)?,
                    total_hours_worked: row.get(4)?,
                    certifier_name: row.get(5)?,
                    certifier_title: row.get(6)?,
                    certifier_phone: row.get(7)?,
                    certification_date: row.get(8)?,
                })
            },
        )
        .ok();

    Ok(Osha300ASummary {
        year,
        establishment_name: est_name,
        street_address: street,
        city,
        state,
        zip_code: zip,
        industry_description: industry,
        naics_code: naics,

        total_deaths: deaths,
        total_days_away_cases: days_away_cases,
        total_transfer_restriction_cases: transfer_cases,
        total_other_recordable_cases: other_cases,
        total_days_away,
        total_days_restricted,

        total_injuries: injuries,
        total_skin_disorders: skin,
        total_respiratory: resp,
        total_poisonings: poison,
        total_hearing_loss: hearing,
        total_other_illnesses: other_ill,

        avg_employees: stats.as_ref().map(|s| s.avg_employees),
        total_hours_worked: stats.as_ref().map(|s| s.total_hours_worked),
        certifier_name: stats.as_ref().and_then(|s| s.certifier_name.clone()),
        certifier_title: stats.as_ref().and_then(|s| s.certifier_title.clone()),
        certifier_phone: stats.as_ref().and_then(|s| s.certifier_phone.clone()),
        certification_date: stats.as_ref().and_then(|s| s.certification_date.clone()),
    })
}

pub fn get_osha_301_report(conn: &Connection, incident_id: i64) -> Result<Osha301Report> {
    conn.query_row(
        "SELECT case_number, employee_name, COALESCE(employee_address,''),
                COALESCE(employee_city,''), COALESCE(employee_state,''), COALESCE(employee_zip,''),
                COALESCE(employee_dob,''), COALESCE(employee_hire_date,''), COALESCE(employee_gender,''),
                COALESCE(physician_name,''), COALESCE(treatment_facility,''),
                COALESCE(facility_address,''), COALESCE(facility_city_state_zip,''),
                COALESCE(treated_in_er,0), COALESCE(hospitalized_overnight,0),
                incident_date, COALESCE(incident_time,''), COALESCE(work_start_time,''),
                COALESCE(where_occurred,''),
                COALESCE(activity_before_incident,''), COALESCE(how_injury_occurred,''),
                COALESCE(injury_description,''), COALESCE(object_substance,''),
                COALESCE(date_of_death,''),
                COALESCE(completed_by,''), COALESCE(completed_by_title,''),
                COALESCE(completed_by_phone,''), COALESCE(completed_date,''),
                is_privacy_case
         FROM incidents WHERE id = ?1",
        [incident_id],
        |row| {
            let is_privacy: bool = row.get::<_, i32>(28)? != 0;
            let name: String = row.get(1)?;

            Ok(Osha301Report {
                case_number: row.get::<_, Option<i64>>(0)?.unwrap_or(0),
                employee_name: if is_privacy { "Privacy Case".to_string() } else { name },
                employee_address: row.get(2)?,
                employee_city: row.get(3)?,
                employee_state: row.get(4)?,
                employee_zip: row.get(5)?,
                employee_dob: row.get(6)?,
                employee_hire_date: row.get(7)?,
                employee_gender: row.get(8)?,
                physician_name: row.get(9)?,
                treatment_facility: row.get(10)?,
                facility_address: row.get(11)?,
                facility_city_state_zip: row.get(12)?,
                treated_in_er: row.get::<_, i32>(13)? != 0,
                hospitalized_overnight: row.get::<_, i32>(14)? != 0,
                incident_date: row.get(15)?,
                incident_time: row.get(16)?,
                work_start_time: row.get(17)?,
                where_occurred: row.get(18)?,
                activity_before_incident: row.get(19)?,
                how_injury_occurred: row.get(20)?,
                injury_description: row.get(21)?,
                object_substance: row.get(22)?,
                date_of_death: row.get(23)?,
                completed_by: row.get(24)?,
                completed_by_title: row.get(25)?,
                completed_by_phone: row.get(26)?,
                completed_date: row.get(27)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Incident {incident_id} not found")).into()
        }
        _ => anyhow::Error::new(e),
    })
}

pub fn upsert_annual_stats(conn: &Connection, data: UpsertAnnualStats) -> Result<AnnualStats> {
    conn.execute(
        "INSERT INTO annual_stats (establishment_id, year, avg_employees, total_hours_worked,
                                    certifier_name, certifier_title, certifier_phone, certification_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(establishment_id, year) DO UPDATE SET
            avg_employees = excluded.avg_employees,
            total_hours_worked = excluded.total_hours_worked,
            certifier_name = excluded.certifier_name,
            certifier_title = excluded.certifier_title,
            certifier_phone = excluded.certifier_phone,
            certification_date = excluded.certification_date",
        params![
            data.establishment_id,
            data.year,
            data.avg_employees,
            data.total_hours_worked,
            data.certifier_name,
            data.certifier_title,
            data.certifier_phone,
            data.certification_date,
        ],
    )
    .context("Failed to upsert annual stats")?;

    conn.query_row(
        "SELECT id, establishment_id, year, avg_employees, total_hours_worked,
                certifier_name, certifier_title, certifier_phone, certification_date
         FROM annual_stats WHERE establishment_id = ?1 AND year = ?2",
        params![data.establishment_id, data.year],
        |row| {
            Ok(AnnualStats {
                id: row.get(0)?,
                establishment_id: row.get(1)?,
                year: row.get(2)?,
                avg_employees: row.get(3)?,
                total_hours_worked: row.get(4)?,
                certifier_name: row.get(5)?,
                certifier_title: row.get(6)?,
                certifier_phone: row.get(7)?,
                certification_date: row.get(8)?,
            })
        },
    )
    .map_err(anyhow::Error::new)
}

pub fn get_annual_stats(
    conn: &Connection,
    establishment_id: i64,
    year: i64,
) -> Result<Option<AnnualStats>> {
    match conn.query_row(
        "SELECT id, establishment_id, year, avg_employees, total_hours_worked,
                certifier_name, certifier_title, certifier_phone, certification_date
         FROM annual_stats WHERE establishment_id = ?1 AND year = ?2",
        params![establishment_id, year],
        |row| {
            Ok(AnnualStats {
                id: row.get(0)?,
                establishment_id: row.get(1)?,
                year: row.get(2)?,
                avg_employees: row.get(3)?,
                total_hours_worked: row.get(4)?,
                certifier_name: row.get(5)?,
                certifier_title: row.get(6)?,
                certifier_phone: row.get(7)?,
                certification_date: row.get(8)?,
            })
        },
    ) {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::incidents::{create_incident, CreateIncident};
    use crate::db::locations::{
        create_establishment, create_location, CreateEstablishment, CreateLocation,
    };
    use crate::db::open_test_db;

    fn setup(conn: &Connection) -> (i64, i64) {
        let est = create_establishment(
            conn,
            CreateEstablishment {
                name: "ABC Construction".into(),
                street_address: Some("100 Main St".into()),
                city: Some("Chicago".into()),
                state: Some("IL".into()),
                zip_code: Some("60601".into()),
                industry_description: Some("General Construction".into()),
                naics_code: Some("236220".into()),
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

        let inc = create_incident(
            conn,
            CreateIncident {
                establishment_id: est.id,
                location_id: Some(loc.id),
                employee_name: "John Doe".into(),
                incident_date: "2026-03-15".into(),
                description: "Fell from ladder".into(),
                employee_job_title: Some("Laborer".into()),
                outcome_severity: Some("days_away".into()),
                days_away_count: Some(10),
                days_restricted_count: Some(5),
                injury_illness_type: Some("injury".into()),
                is_recordable: Some(true),
                employee_address: None,
                employee_city: None,
                employee_state: None,
                employee_zip: None,
                employee_dob: None,
                employee_hire_date: None,
                employee_gender: None,
                is_privacy_case: None,
                incident_time: None,
                work_start_time: None,
                where_occurred: Some("Building A".into()),
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
                date_of_death: None,
            },
        )
        .unwrap();

        (est.id, inc.id)
    }

    #[test]
    fn test_osha_300_log() {
        let conn = open_test_db();
        let (est_id, _inc_id) = setup(&conn);

        let log = get_osha_300_log(&conn, est_id, 2026).unwrap();
        assert_eq!(log.len(), 1);
        assert!(log[0].outcome_days_away);
        assert!(log[0].type_injury);
        assert_eq!(log[0].days_away_count, 10);
    }

    #[test]
    fn test_osha_300a_summary() {
        let conn = open_test_db();
        let (est_id, _) = setup(&conn);

        upsert_annual_stats(
            &conn,
            UpsertAnnualStats {
                establishment_id: est_id,
                year: 2026,
                avg_employees: 50,
                total_hours_worked: 100000,
                certifier_name: Some("Jane Smith".into()),
                certifier_title: Some("Safety Director".into()),
                certifier_phone: None,
                certification_date: None,
            },
        )
        .unwrap();

        let summary = get_osha_300a_summary(&conn, est_id, 2026).unwrap();
        assert_eq!(summary.total_days_away_cases, 1);
        assert_eq!(summary.total_days_away, 10);
        assert_eq!(summary.total_injuries, 1);
        assert_eq!(summary.avg_employees, Some(50));
    }

    #[test]
    fn test_osha_301_report() {
        let conn = open_test_db();
        let (_est_id, inc_id) = setup(&conn);

        let report = get_osha_301_report(&conn, inc_id).unwrap();
        assert_eq!(report.employee_name, "John Doe");
        assert_eq!(report.incident_date, "2026-03-15");
    }
}
