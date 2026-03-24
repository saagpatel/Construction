import { useEffect, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { useIncidentStore } from "../../stores/incidentStore";
import {
  OUTCOME_SEVERITY_LABELS,
  INCIDENT_STATUS_LABELS,
} from "../../lib/constants";
import type {
  IncidentStatus,
  OutcomeSeverity,
  UpdateIncident,
} from "../../lib/types";

export function IncidentEditPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const {
    currentIncident,
    loading,
    loadIncident,
    updateIncident,
    clearCurrent,
  } = useIncidentStore();

  const [form, setForm] = useState<UpdateIncident>({});
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    if (id) {
      loadIncident(Number(id));
    }
    return () => clearCurrent();
  }, [id, loadIncident, clearCurrent]);

  useEffect(() => {
    if (!currentIncident) return;

    setForm({
      employee_name: currentIncident.employee_name,
      incident_date: currentIncident.incident_date,
      description: currentIncident.description,
      where_occurred: currentIncident.where_occurred ?? undefined,
      status: currentIncident.status,
      outcome_severity: currentIncident.outcome_severity,
      days_away_count: currentIncident.days_away_count,
      days_restricted_count: currentIncident.days_restricted_count,
    });
  }, [currentIncident]);

  const setField = <K extends keyof UpdateIncident>(
    field: K,
    value: UpdateIncident[K],
  ) => {
    setForm((prev) => ({ ...prev, [field]: value }));
  };

  const handleSave = async () => {
    if (!currentIncident) return;
    if (
      !form.employee_name?.trim() ||
      !form.incident_date ||
      !form.description?.trim()
    ) {
      setError("Employee name, incident date, and description are required");
      return;
    }

    setSaving(true);
    setError("");
    try {
      await updateIncident(currentIncident.id, form);
      navigate(`/incidents/${currentIncident.id}`);
    } catch (e) {
      setError(String(e));
      setSaving(false);
    }
  };

  if (loading || !currentIncident) {
    return (
      <p className="text-gray-500">
        {loading ? "Loading..." : "Incident not found"}
      </p>
    );
  }

  const inputClass = "w-full border rounded-md px-3 py-2 text-sm";
  const labelClass = "block text-sm font-medium text-gray-700 mb-1";

  return (
    <div className="max-w-3xl mx-auto space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Edit Incident</h1>
        <Link
          to={`/incidents/${currentIncident.id}`}
          className="text-sm text-safety-orange hover:underline"
        >
          Back to incident
        </Link>
      </div>

      <div className="bg-white rounded-lg shadow p-6 space-y-4">
        <div>
          <label className={labelClass}>Employee Name *</label>
          <input
            type="text"
            value={form.employee_name ?? ""}
            onChange={(e) => setField("employee_name", e.target.value)}
            className={inputClass}
          />
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className={labelClass}>Incident Date *</label>
            <input
              type="date"
              value={form.incident_date ?? ""}
              onChange={(e) => setField("incident_date", e.target.value)}
              className={inputClass}
            />
          </div>
          <div>
            <label className={labelClass}>Status</label>
            <select
              value={form.status ?? "open"}
              onChange={(e) =>
                setField("status", e.target.value as IncidentStatus)
              }
              className={inputClass}
            >
              {Object.entries(INCIDENT_STATUS_LABELS).map(([value, label]) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div>
          <label className={labelClass}>Where did the event occur?</label>
          <input
            type="text"
            value={form.where_occurred ?? ""}
            onChange={(e) =>
              setField("where_occurred", e.target.value || undefined)
            }
            className={inputClass}
          />
        </div>

        <div>
          <label className={labelClass}>Description *</label>
          <textarea
            value={form.description ?? ""}
            onChange={(e) => setField("description", e.target.value)}
            className={inputClass}
            rows={4}
          />
        </div>

        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className={labelClass}>Severity</label>
            <select
              value={form.outcome_severity ?? "other_recordable"}
              onChange={(e) =>
                setField("outcome_severity", e.target.value as OutcomeSeverity)
              }
              className={inputClass}
            >
              {Object.entries(OUTCOME_SEVERITY_LABELS).map(([value, label]) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className={labelClass}>Days Away</label>
            <input
              type="number"
              min={0}
              max={180}
              value={form.days_away_count ?? 0}
              onChange={(e) =>
                setField("days_away_count", Number(e.target.value))
              }
              className={inputClass}
            />
          </div>
          <div>
            <label className={labelClass}>Days Restricted</label>
            <input
              type="number"
              min={0}
              max={180}
              value={form.days_restricted_count ?? 0}
              onChange={(e) =>
                setField("days_restricted_count", Number(e.target.value))
              }
              className={inputClass}
            />
          </div>
        </div>

        {error && <p className="text-sm text-red-600">{error}</p>}

        <div className="flex justify-end gap-3 pt-2">
          <Link
            to={`/incidents/${currentIncident.id}`}
            className="border px-4 py-2 rounded text-sm hover:bg-gray-50"
          >
            Cancel
          </Link>
          <button
            onClick={handleSave}
            disabled={saving}
            className="bg-safety-green text-white px-4 py-2 rounded text-sm hover:bg-green-600 disabled:opacity-50"
          >
            {saving ? "Saving..." : "Save changes"}
          </button>
        </div>
      </div>
    </div>
  );
}
