import { useEffect, useState } from "react";
import { useParams, Link } from "react-router-dom";
import { useRcaStore } from "../../stores/rcaStore";
import { useIncidentStore } from "../../stores/incidentStore";
import { useToast } from "../../hooks/useToast";
import type { IncidentStatus } from "../../lib/types";
import { FiveWhysWizard } from "./FiveWhysWizard";
import { FishboneDiagram } from "./FishboneDiagram";
import { CorrectiveActions } from "./CorrectiveActions";

export function RcaPage() {
  const { id } = useParams<{ id: string }>();
  const incidentId = Number(id);
  const toast = useToast();
  const { currentIncident, loadIncident, updateIncident } = useIncidentStore();
  const {
    sessions,
    currentSession,
    loading,
    loadSessions,
    createSession,
    setCurrentSession,
    completeSession,
  } = useRcaStore();

  const [showCreate, setShowCreate] = useState(false);
  const [summaryText, setSummaryText] = useState("");

  useEffect(() => {
    loadIncident(incidentId);
    loadSessions(incidentId);
  }, [incidentId, loadIncident, loadSessions]);

  const handleCreate = async (method: "five_whys" | "fishbone") => {
    const session = await createSession(incidentId, method);
    setCurrentSession(session);
    setShowCreate(false);
  };

  const handleComplete = async () => {
    if (!currentSession || !summaryText.trim()) return;
    await completeSession(currentSession.id, summaryText.trim());
    setSummaryText("");
    toast.success("Analysis completed");
  };

  const handleIncidentHandoff = async (status: IncidentStatus) => {
    if (!currentIncident) return;
    try {
      await updateIncident(currentIncident.id, { status });
      await loadIncident(currentIncident.id);
      toast.success(`Incident moved to ${status.replace("_", " ")}`);
    } catch (error) {
      toast.error(`Failed to update incident status: ${error}`);
    }
  };

  return (
    <div className="max-w-5xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <Link
            to={`/incidents/${incidentId}`}
            className="text-sm text-safety-orange hover:underline"
          >
            Back to Incident
          </Link>
          <h1 className="text-2xl font-bold mt-1">
            Root Cause Analysis - Case #{currentIncident?.case_number ?? "N/A"}
          </h1>
        </div>
        <button
          onClick={() => setShowCreate(true)}
          className="bg-safety-orange text-white px-4 py-2 rounded text-sm"
        >
          New Analysis
        </button>
      </div>

      {showCreate && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="font-semibold mb-4">Choose Analysis Method</h2>
          <div className="grid grid-cols-2 gap-4">
            <button
              onClick={() => handleCreate("five_whys")}
              className="border-2 rounded-lg p-6 hover:border-safety-orange text-left"
            >
              <h3 className="font-semibold text-lg">5 Whys</h3>
              <p className="text-sm text-gray-500 mt-1">
                Ask "Why?" iteratively to drill down to the root cause
              </p>
            </button>
            <button
              onClick={() => handleCreate("fishbone")}
              className="border-2 rounded-lg p-6 hover:border-safety-orange text-left"
            >
              <h3 className="font-semibold text-lg">Fishbone Diagram</h3>
              <p className="text-sm text-gray-500 mt-1">
                Categorize causes across 6 categories (Ishikawa method)
              </p>
            </button>
          </div>
          <button
            onClick={() => setShowCreate(false)}
            className="text-sm text-gray-500 mt-3 hover:underline"
          >
            Cancel
          </button>
        </div>
      )}

      {/* Session List */}
      {sessions.length > 0 && (
        <div className="bg-white rounded-lg shadow">
          <div className="border-b px-4 py-3">
            <h2 className="font-semibold">Analysis Sessions</h2>
          </div>
          <div className="divide-y">
            {sessions.map((session) => (
              <button
                key={session.id}
                onClick={() => setCurrentSession(session)}
                className={`w-full text-left px-4 py-3 hover:bg-gray-50 flex items-center justify-between ${
                  currentSession?.id === session.id ? "bg-orange-50" : ""
                }`}
              >
                <div>
                  <span className="font-medium text-sm capitalize">
                    {session.method === "five_whys" ? "5 Whys" : "Fishbone"}
                  </span>
                  <span className="text-xs text-gray-500 ml-3">
                    {session.created_at}
                  </span>
                </div>
                <span
                  className={`text-xs px-2 py-0.5 rounded ${
                    session.status === "completed"
                      ? "bg-green-100 text-green-800"
                      : "bg-yellow-100 text-yellow-800"
                  }`}
                >
                  {session.status}
                </span>
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Active Session */}
      {currentSession && (
        <div className="space-y-6">
          {currentSession.method === "five_whys" ? (
            <FiveWhysWizard />
          ) : (
            <FishboneDiagram />
          )}

          {/* Complete Session */}
          {currentSession.status === "in_progress" && (
            <div className="bg-white rounded-lg shadow p-6">
              <h3 className="font-semibold mb-3">Complete Analysis</h3>
              <textarea
                value={summaryText}
                onChange={(e) => setSummaryText(e.target.value)}
                className="w-full border rounded px-3 py-2 text-sm"
                rows={3}
                placeholder="Summarize the root cause..."
              />
              <button
                onClick={handleComplete}
                className="mt-3 bg-safety-green text-white px-4 py-2 rounded text-sm"
                disabled={!summaryText.trim()}
              >
                Mark as Complete
              </button>
            </div>
          )}

          {currentSession.root_cause_summary && (
            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <h3 className="font-semibold text-green-800">
                Root Cause Summary
              </h3>
              <p className="text-sm mt-1 text-green-700">
                {currentSession.root_cause_summary}
              </p>
              <div className="mt-4 pt-3 border-t border-green-200 flex flex-wrap items-center gap-2">
                <span className="text-xs text-green-700">
                  Incident status:{" "}
                  <strong>
                    {currentIncident?.status?.replace("_", " ") ?? "unknown"}
                  </strong>
                </span>
                {currentIncident?.status === "open" && (
                  <button
                    onClick={() => handleIncidentHandoff("in_review")}
                    className="text-xs px-2 py-1 rounded bg-blue-600 text-white hover:bg-blue-700"
                  >
                    Move Incident to In Review
                  </button>
                )}
                {currentIncident?.status === "in_review" && (
                  <button
                    onClick={() => handleIncidentHandoff("closed")}
                    className="text-xs px-2 py-1 rounded bg-safety-green text-white hover:bg-green-700"
                  >
                    Close Incident
                  </button>
                )}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Corrective Actions */}
      {!loading && <CorrectiveActions incidentId={incidentId} />}
    </div>
  );
}
