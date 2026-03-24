import { useEffect, useState } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { useIncidentStore } from "../../stores/incidentStore";
import {
  OUTCOME_SEVERITY_LABELS,
  INJURY_TYPE_LABELS,
  INCIDENT_STATUS_LABELS,
} from "../../lib/constants";
import type { IncidentStatus } from "../../lib/types";
import { AttachmentUpload } from "./AttachmentUpload";
import { AttachmentGallery } from "./AttachmentGallery";

export function IncidentDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const {
    currentIncident: incident,
    attachments,
    loading,
    loadIncident,
    loadAttachments,
    updateIncident,
    deleteIncident,
    clearCurrent,
  } = useIncidentStore();
  const [confirmDelete, setConfirmDelete] = useState(false);

  useEffect(() => {
    if (id) loadIncident(Number(id));
    return () => clearCurrent();
  }, [id, loadIncident, clearCurrent]);

  if (loading || !incident) {
    return (
      <p className="text-gray-500">
        {loading ? "Loading..." : "Incident not found"}
      </p>
    );
  }

  const handleStatusChange = async (status: IncidentStatus) => {
    await updateIncident(incident.id, { status });
  };

  const handleDelete = async () => {
    await deleteIncident(incident.id);
    navigate("/incidents");
  };

  const field = (
    label: string,
    value: string | number | null | undefined | boolean,
  ) => {
    if (value === null || value === undefined || value === "") return null;
    const display =
      typeof value === "boolean" ? (value ? "Yes" : "No") : String(value);
    return (
      <div>
        <dt className="text-xs text-gray-500 uppercase tracking-wide">
          {label}
        </dt>
        <dd className="text-sm mt-0.5">{display}</dd>
      </div>
    );
  };

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-3 mb-1">
            <h1 className="text-2xl font-bold">
              Case #{incident.case_number ?? "N/A"}
            </h1>
            <span
              className={`text-xs px-2 py-1 rounded-full font-medium ${
                incident.status === "open"
                  ? "bg-orange-100 text-orange-800"
                  : incident.status === "in_review"
                    ? "bg-blue-100 text-blue-800"
                    : "bg-green-100 text-green-800"
              }`}
            >
              {INCIDENT_STATUS_LABELS[incident.status]}
            </span>
          </div>
          <p className="text-gray-500 text-sm">
            {incident.incident_date} -{" "}
            {incident.is_privacy_case ? "Privacy Case" : incident.employee_name}
          </p>
        </div>
        <div className="flex gap-2">
          <Link
            to={`/incidents/${incident.id}/edit`}
            className="border border-safety-orange text-safety-orange px-3 py-1.5 rounded text-sm hover:bg-orange-50"
          >
            Edit
          </Link>
          <Link
            to={`/incidents/${incident.id}/rca`}
            className="bg-safety-blue text-white px-3 py-1.5 rounded text-sm hover:bg-blue-600"
          >
            Root Cause Analysis
          </Link>
          <select
            value={incident.status}
            onChange={(e) =>
              handleStatusChange(e.target.value as IncidentStatus)
            }
            className="border rounded px-3 py-1.5 text-sm"
          >
            <option value="open">Open</option>
            <option value="in_review">In Review</option>
            <option value="closed">Closed</option>
          </select>
        </div>
      </div>

      {/* Incident Details */}
      <section className="bg-white rounded-lg shadow p-6">
        <h2 className="font-semibold text-lg mb-4">Incident Details</h2>
        <dl className="grid grid-cols-2 md:grid-cols-3 gap-4">
          {field(
            "Employee",
            incident.is_privacy_case ? "Privacy Case" : incident.employee_name,
          )}
          {field("Job Title", incident.employee_job_title)}
          {field("Date", incident.incident_date)}
          {field("Time", incident.incident_time)}
          {field("Work Start", incident.work_start_time)}
          {field("Location", incident.where_occurred)}
          {field("Gender", incident.employee_gender)}
          {field("DOB", incident.employee_dob)}
          {field("Hire Date", incident.employee_hire_date)}
        </dl>
        <div className="mt-4">
          <dt className="text-xs text-gray-500 uppercase tracking-wide">
            Description
          </dt>
          <dd className="text-sm mt-1 whitespace-pre-wrap">
            {incident.description}
          </dd>
        </div>
      </section>

      {/* 301 Detail Fields */}
      <section className="bg-white rounded-lg shadow p-6">
        <h2 className="font-semibold text-lg mb-4">OSHA 301 Details</h2>
        <dl className="space-y-3">
          {field("Activity Before Incident", incident.activity_before_incident)}
          {field("How Injury Occurred", incident.how_injury_occurred)}
          {field("Injury Description", incident.injury_description)}
          {field("Object/Substance", incident.object_substance)}
        </dl>
      </section>

      {/* Classification */}
      <section className="bg-white rounded-lg shadow p-6">
        <h2 className="font-semibold text-lg mb-4">Classification</h2>
        <dl className="grid grid-cols-2 md:grid-cols-3 gap-4">
          {field(
            "Severity",
            OUTCOME_SEVERITY_LABELS[incident.outcome_severity],
          )}
          {field(
            "Injury Type",
            INJURY_TYPE_LABELS[incident.injury_illness_type],
          )}
          {field("Days Away", incident.days_away_count)}
          {field("Days Restricted", incident.days_restricted_count)}
          {field("Recordable", incident.is_recordable)}
          {field("Date of Death", incident.date_of_death)}
        </dl>
      </section>

      {/* Healthcare */}
      <section className="bg-white rounded-lg shadow p-6">
        <h2 className="font-semibold text-lg mb-4">Healthcare</h2>
        <dl className="grid grid-cols-2 gap-4">
          {field("Physician", incident.physician_name)}
          {field("Facility", incident.treatment_facility)}
          {field("Facility Address", incident.facility_address)}
          {field("Treated in ER", incident.treated_in_er)}
          {field("Hospitalized Overnight", incident.hospitalized_overnight)}
        </dl>
      </section>

      {/* Attachments */}
      <section className="bg-white rounded-lg shadow p-6">
        <h2 className="font-semibold text-lg mb-4">
          Attachments ({attachments.length})
        </h2>
        <p className="text-sm text-gray-500 mb-4">
          Upload field evidence (photos, audio notes, and documents) for this
          incident.
        </p>
        <AttachmentUpload
          incidentId={incident.id}
          onUploadComplete={() => {
            loadAttachments(incident.id);
          }}
        />
        <div className="mt-6">
          <AttachmentGallery
            attachments={attachments}
            onDelete={() => {
              loadAttachments(incident.id);
            }}
          />
        </div>
      </section>

      {/* Danger Zone */}
      <section className="border border-red-200 rounded-lg p-6">
        <h2 className="font-semibold text-lg text-red-700 mb-2">Danger Zone</h2>
        {confirmDelete ? (
          <div className="flex items-center gap-3">
            <p className="text-sm text-red-600">
              Are you sure? This cannot be undone.
            </p>
            <button
              onClick={handleDelete}
              className="bg-red-600 text-white px-3 py-1.5 rounded text-sm"
            >
              Yes, Delete
            </button>
            <button
              onClick={() => setConfirmDelete(false)}
              className="border px-3 py-1.5 rounded text-sm"
            >
              Cancel
            </button>
          </div>
        ) : (
          <button
            onClick={() => setConfirmDelete(true)}
            className="text-red-600 text-sm hover:underline"
          >
            Delete this incident
          </button>
        )}
      </section>
    </div>
  );
}
