import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../hooks/useToast";
import { SignaturePad } from "../components/toolbox/SignaturePad";

interface ToolboxTalk {
  id: number;
  title: string;
  date: string;
  conducted_by: string;
  notes?: string;
  status: string;
}

interface ToolboxTalkAttendee {
  id: number;
  talk_id: number;
  employee_name: string;
  signature_data?: string;
  signed_at?: string;
}

export function ToolboxTalkDetailPage() {
  const { id } = useParams();
  const toast = useToast();
  const talkId = Number(id);

  const [talk, setTalk] = useState<ToolboxTalk | null>(null);
  const [attendees, setAttendees] = useState<ToolboxTalkAttendee[]>([]);
  const [employeeName, setEmployeeName] = useState("");
  const [loading, setLoading] = useState(true);
  const [signingAttendee, setSigningAttendee] =
    useState<ToolboxTalkAttendee | null>(null);

  const loadData = async () => {
    if (!talkId) return;

    try {
      const [talkData, attendeeData] = await Promise.all([
        invoke<ToolboxTalk>("get_toolbox_talk", { id: talkId }),
        invoke<ToolboxTalkAttendee[]>("list_toolbox_attendees", { talkId }),
      ]);
      setTalk(talkData);
      setAttendees(attendeeData);
    } catch (error) {
      toast.error(`Failed to load talk details: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [talkId]);

  const handleAddAttendee = async () => {
    if (!employeeName.trim()) return;

    try {
      await invoke("add_toolbox_attendee", {
        data: {
          talk_id: talkId,
          employee_name: employeeName,
          employee_id: null,
        },
      });
      setEmployeeName("");
      await loadData();
      toast.success("Attendee added");
    } catch (error) {
      toast.error(`Failed to add attendee: ${error}`);
    }
  };

  const handleComplete = async () => {
    try {
      const updated = await invoke<ToolboxTalk>("complete_toolbox_talk", {
        talkId,
      });
      setTalk(updated);
      toast.success("Talk marked complete");
    } catch (error) {
      toast.error(`Failed to complete talk: ${error}`);
    }
  };

  const handleDeleteAttendee = async (attendeeId: number) => {
    if (!confirm("Remove this attendee from the talk?")) return;
    try {
      await invoke("delete_toolbox_attendee", { id: attendeeId });
      await loadData();
      toast.success("Attendee removed");
    } catch (error) {
      toast.error(`Failed to remove attendee: ${error}`);
    }
  };

  const handleSaveSignature = async (signatureData: string) => {
    if (!signingAttendee) return;
    try {
      await invoke("sign_toolbox_attendee", {
        data: {
          attendee_id: signingAttendee.id,
          signature_data: signatureData,
        },
      });
      setSigningAttendee(null);
      await loadData();
      toast.success("Signature captured");
    } catch (error) {
      toast.error(`Failed to save signature: ${error}`);
    }
  };

  if (loading) {
    return <p className="text-gray-500">Loading toolbox talk...</p>;
  }

  if (!talk) {
    return (
      <div>
        <p className="text-gray-500">Toolbox talk not found.</p>
        <Link to="/toolbox" className="text-safety-orange hover:underline">
          Back to talks
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">{talk.title}</h1>
          <p className="text-sm text-gray-600">
            {talk.date} • Conducted by {talk.conducted_by}
          </p>
        </div>
        {talk.status !== "completed" && (
          <button
            onClick={handleComplete}
            className="bg-green-600 text-white px-4 py-2 rounded"
          >
            Mark Complete
          </button>
        )}
      </div>

      {talk.notes && (
        <div className="bg-white rounded-lg shadow p-4">
          <h2 className="font-semibold mb-2">Notes</h2>
          <p className="text-sm text-gray-700 whitespace-pre-wrap">
            {talk.notes}
          </p>
        </div>
      )}

      <div className="bg-white rounded-lg shadow p-4 space-y-3">
        <h2 className="font-semibold">Attendees ({attendees.length})</h2>
        <div className="flex gap-2">
          <input
            value={employeeName}
            onChange={(e) => setEmployeeName(e.target.value)}
            placeholder="Employee name"
            className="border rounded px-3 py-2 flex-1"
          />
          <button
            onClick={handleAddAttendee}
            className="bg-safety-orange text-white px-4 py-2 rounded"
          >
            Add
          </button>
        </div>
        {attendees.length === 0 ? (
          <p className="text-sm text-gray-500">No attendees yet.</p>
        ) : (
          <ul className="divide-y">
            {attendees.map((attendee) => (
              <li
                key={attendee.id}
                className="py-2 text-sm flex items-center justify-between"
              >
                <div>
                  <p>{attendee.employee_name}</p>
                  {attendee.signed_at && (
                    <p className="text-xs text-gray-500">
                      Signed at {new Date(attendee.signed_at).toLocaleString()}
                    </p>
                  )}
                </div>
                <div className="flex items-center gap-3">
                  {attendee.signature_data ? (
                    <span className="text-xs text-green-600">Signed</span>
                  ) : (
                    <button
                      onClick={() => setSigningAttendee(attendee)}
                      className="text-xs text-safety-orange hover:underline"
                    >
                      Capture Signature
                    </button>
                  )}
                  <button
                    onClick={() => handleDeleteAttendee(attendee.id)}
                    className="text-xs text-red-500 hover:underline"
                  >
                    Remove
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </div>

      {signingAttendee && (
        <SignaturePad
          onCancel={() => setSigningAttendee(null)}
          onSave={handleSaveSignature}
        />
      )}
    </div>
  );
}
