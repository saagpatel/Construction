import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "../stores/settingsStore";
import { useToast } from "../hooks/useToast";

interface JsaTemplate {
  id: number;
  title: string;
}

interface JsaInstance {
  id: number;
  job_name: string;
  job_date: string;
  prepared_by: string;
  status: string;
}

export function JsaPage() {
  const toast = useToast();
  const { activeEstablishment } = useSettingsStore();
  const [templates, setTemplates] = useState<JsaTemplate[]>([]);
  const [instances, setInstances] = useState<JsaInstance[]>([]);
  const [jobName, setJobName] = useState("");
  const [jobDate, setJobDate] = useState(
    new Date().toISOString().split("T")[0],
  );
  const [preparedBy, setPreparedBy] = useState("");
  const [templateId, setTemplateId] = useState<number | "">("");
  const [updatingId, setUpdatingId] = useState<number | null>(null);

  const loadData = async () => {
    if (!activeEstablishment) return;

    try {
      const [templateData, instanceData] = await Promise.all([
        invoke<JsaTemplate[]>("list_jsa_templates"),
        invoke<JsaInstance[]>("list_jsa_instances", {
          establishmentId: activeEstablishment.id,
        }),
      ]);
      setTemplates(templateData);
      setInstances(instanceData);
    } catch (error) {
      toast.error(`Failed to load JSA data: ${error}`);
    }
  };

  useEffect(() => {
    loadData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeEstablishment?.id]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!activeEstablishment) {
      toast.error("No active establishment selected");
      return;
    }

    try {
      await invoke("create_jsa_instance", {
        data: {
          template_id: templateId || null,
          establishment_id: activeEstablishment.id,
          location_id: null,
          job_name: jobName,
          job_date: jobDate,
          prepared_by: preparedBy,
        },
      });
      setJobName("");
      setPreparedBy("");
      setTemplateId("");
      await loadData();
      toast.success("JSA created");
    } catch (error) {
      toast.error(`Failed to create JSA: ${error}`);
    }
  };

  const getNextTransition = (
    status: string,
  ): { label: string; value: string } | null => {
    switch (status) {
      case "draft":
        return { label: "Mark Reviewed", value: "reviewed" };
      case "reviewed":
        return { label: "Approve", value: "approved" };
      case "approved":
        return { label: "Start Work", value: "in_progress" };
      case "in_progress":
        return { label: "Mark Complete", value: "completed" };
      default:
        return null;
    }
  };

  const statusBadgeClass = (status: string) => {
    const colors: Record<string, string> = {
      draft: "bg-gray-100 text-gray-700",
      reviewed: "bg-blue-100 text-blue-800",
      approved: "bg-purple-100 text-purple-800",
      in_progress: "bg-orange-100 text-orange-800",
      completed: "bg-green-100 text-green-800",
    };
    return colors[status] ?? "bg-gray-100 text-gray-700";
  };

  const handleAdvanceStatus = async (instance: JsaInstance) => {
    const transition = getNextTransition(instance.status);
    if (!transition) return;

    setUpdatingId(instance.id);
    try {
      await invoke("update_jsa_status", {
        id: instance.id,
        status: transition.value,
      });
      await loadData();
      toast.success(`JSA moved to ${transition.value.replace("_", " ")}`);
    } catch (error) {
      toast.error(`Failed to update JSA status: ${error}`);
    } finally {
      setUpdatingId(null);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Job Safety Analysis (JSA)</h1>
        <p className="text-sm text-gray-600">
          Create, review, approve, and track JSAs for jobs and tasks.
        </p>
      </div>

      <form
        onSubmit={handleCreate}
        className="bg-white rounded-lg shadow p-4 grid grid-cols-1 md:grid-cols-4 gap-3"
      >
        <select
          value={templateId}
          onChange={(e) =>
            setTemplateId(e.target.value ? Number(e.target.value) : "")
          }
          className="border rounded px-3 py-2"
        >
          <option value="">No template</option>
          {templates.map((template) => (
            <option key={template.id} value={template.id}>
              {template.title}
            </option>
          ))}
        </select>
        <input
          className="border rounded px-3 py-2"
          placeholder="Job name"
          value={jobName}
          onChange={(e) => setJobName(e.target.value)}
          required
        />
        <input
          className="border rounded px-3 py-2"
          type="date"
          value={jobDate}
          onChange={(e) => setJobDate(e.target.value)}
          required
        />
        <input
          className="border rounded px-3 py-2"
          placeholder="Prepared by"
          value={preparedBy}
          onChange={(e) => setPreparedBy(e.target.value)}
          required
        />
        <div className="md:col-span-4">
          <button
            className="bg-safety-orange text-white px-4 py-2 rounded"
            type="submit"
          >
            Create JSA
          </button>
        </div>
      </form>

      <div className="bg-white rounded-lg shadow overflow-hidden">
        <div className="px-4 py-3 border-b">
          <h2 className="font-semibold">Recent JSAs ({instances.length})</h2>
        </div>
        {instances.length === 0 ? (
          <p className="p-4 text-sm text-gray-500">No JSAs yet.</p>
        ) : (
          <table className="w-full text-sm">
            <thead className="bg-gray-50">
              <tr>
                <th className="text-left px-4 py-2">Job</th>
                <th className="text-left px-4 py-2">Date</th>
                <th className="text-left px-4 py-2">Prepared By</th>
                <th className="text-left px-4 py-2">Status</th>
                <th className="text-left px-4 py-2">Workflow</th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {instances.map((instance) => (
                <tr key={instance.id}>
                  <td className="px-4 py-2">{instance.job_name}</td>
                  <td className="px-4 py-2">{instance.job_date}</td>
                  <td className="px-4 py-2">{instance.prepared_by}</td>
                  <td className="px-4 py-2">
                    <span
                      className={`px-2 py-0.5 rounded text-xs ${statusBadgeClass(instance.status)}`}
                    >
                      {instance.status.replace("_", " ")}
                    </span>
                  </td>
                  <td className="px-4 py-2">
                    {getNextTransition(instance.status) ? (
                      <button
                        onClick={() => handleAdvanceStatus(instance)}
                        className="text-xs px-2 py-1 rounded border border-safety-orange text-safety-orange hover:bg-orange-50 disabled:opacity-50"
                        disabled={updatingId === instance.id}
                      >
                        {updatingId === instance.id
                          ? "Updating..."
                          : getNextTransition(instance.status)?.label}
                      </button>
                    ) : (
                      <span className="text-xs text-gray-500">
                        No further action
                      </span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}
