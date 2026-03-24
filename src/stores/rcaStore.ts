import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type {
  RcaSession,
  FiveWhysStep,
  FishboneCategory,
  FishboneCause,
  CorrectiveAction,
} from "../lib/types";

interface RcaState {
  sessions: RcaSession[];
  currentSession: RcaSession | null;
  fiveWhysSteps: FiveWhysStep[];
  fishboneCategories: FishboneCategory[];
  correctiveActions: CorrectiveAction[];
  correctiveActionsIncidentId: number | null;
  loading: boolean;
  error: string | null;

  loadSessions: (incidentId: number) => Promise<void>;
  createSession: (
    incidentId: number,
    method: "five_whys" | "fishbone",
  ) => Promise<RcaSession>;
  completeSession: (id: number, summary: string) => Promise<void>;
  deleteSession: (id: number) => Promise<void>;
  setCurrentSession: (session: RcaSession) => void;

  loadFiveWhysSteps: (sessionId: number) => Promise<void>;
  addFiveWhysStep: (
    sessionId: number,
    stepNumber: number,
    question: string,
    answer: string,
  ) => Promise<void>;
  updateFiveWhysStep: (
    id: number,
    question: string,
    answer: string,
  ) => Promise<void>;

  loadFishboneCategories: (sessionId: number) => Promise<void>;
  addFishboneCategory: (
    sessionId: number,
    category: string,
    sortOrder?: number,
  ) => Promise<FishboneCategory>;
  addFishboneCause: (
    categoryId: number,
    causeText: string,
    isRootCause?: boolean,
  ) => Promise<FishboneCause>;
  updateFishboneCause: (
    id: number,
    causeText?: string,
    isRootCause?: boolean,
  ) => Promise<void>;
  deleteFishboneCause: (id: number) => Promise<void>;

  loadCorrectiveActions: (incidentId: number) => Promise<void>;
  createCorrectiveAction: (
    incidentId: number,
    description: string,
    assignedTo?: string,
    dueDate?: string,
    rcaSessionId?: number,
  ) => Promise<void>;
  updateCorrectiveAction: (
    id: number,
    data: Record<string, unknown>,
  ) => Promise<void>;
  deleteCorrectiveAction: (id: number) => Promise<void>;
}

export const useRcaStore = create<RcaState>((set, get) => ({
  sessions: [],
  currentSession: null,
  fiveWhysSteps: [],
  fishboneCategories: [],
  correctiveActions: [],
  correctiveActionsIncidentId: null,
  loading: false,
  error: null,

  loadSessions: async (incidentId) => {
    set({ loading: true });
    try {
      const sessions = await invoke<RcaSession[]>("list_rca_sessions", {
        incidentId,
      });
      set({ sessions, loading: false });
    } catch (e) {
      set({ error: String(e), loading: false });
    }
  },

  createSession: async (incidentId, method) => {
    const session = await invoke<RcaSession>("create_rca_session", {
      data: { incident_id: incidentId, method },
    });
    get().loadSessions(incidentId);
    return session;
  },

  completeSession: async (id, summary) => {
    await invoke("complete_rca_session", { id, rootCauseSummary: summary });
    const session = get().currentSession;
    if (session) get().loadSessions(session.incident_id);
  },

  deleteSession: async (id) => {
    const session = get().currentSession;
    await invoke("delete_rca_session", { id });
    if (session) get().loadSessions(session.incident_id);
  },

  setCurrentSession: (session) => {
    set({ currentSession: session });
    if (session.method === "five_whys") {
      get().loadFiveWhysSteps(session.id);
    } else {
      get().loadFishboneCategories(session.id);
    }
  },

  loadFiveWhysSteps: async (sessionId) => {
    const steps = await invoke<FiveWhysStep[]>("list_five_whys_steps", {
      rcaSessionId: sessionId,
    });
    set({ fiveWhysSteps: steps });
  },

  addFiveWhysStep: async (sessionId, stepNumber, question, answer) => {
    await invoke("add_five_whys_step", {
      data: {
        rca_session_id: sessionId,
        step_number: stepNumber,
        question,
        answer,
      },
    });
    get().loadFiveWhysSteps(sessionId);
  },

  updateFiveWhysStep: async (id, question, answer) => {
    await invoke("update_five_whys_step", { id, question, answer });
    const session = get().currentSession;
    if (session) get().loadFiveWhysSteps(session.id);
  },

  loadFishboneCategories: async (sessionId) => {
    const categories = await invoke<FishboneCategory[]>(
      "list_fishbone_categories",
      { rcaSessionId: sessionId },
    );
    set({ fishboneCategories: categories });
  },

  addFishboneCategory: async (sessionId, category, sortOrder) => {
    const cat = await invoke<FishboneCategory>("add_fishbone_category", {
      data: { rca_session_id: sessionId, category, sort_order: sortOrder },
    });
    get().loadFishboneCategories(sessionId);
    return cat;
  },

  addFishboneCause: async (categoryId, causeText, isRootCause) => {
    const cause = await invoke<FishboneCause>("add_fishbone_cause", {
      data: {
        category_id: categoryId,
        cause_text: causeText,
        is_root_cause: isRootCause,
      },
    });
    const session = get().currentSession;
    if (session) get().loadFishboneCategories(session.id);
    return cause;
  },

  updateFishboneCause: async (id, causeText, isRootCause) => {
    await invoke("update_fishbone_cause", { id, causeText, isRootCause });
    const session = get().currentSession;
    if (session) get().loadFishboneCategories(session.id);
  },

  deleteFishboneCause: async (id) => {
    await invoke("delete_fishbone_cause", { id });
    const session = get().currentSession;
    if (session) get().loadFishboneCategories(session.id);
  },

  loadCorrectiveActions: async (incidentId) => {
    const actions = await invoke<CorrectiveAction[]>(
      "list_corrective_actions",
      { incidentId },
    );
    set({
      correctiveActions: actions,
      correctiveActionsIncidentId: incidentId,
    });
  },

  createCorrectiveAction: async (
    incidentId,
    description,
    assignedTo,
    dueDate,
    rcaSessionId,
  ) => {
    await invoke("create_corrective_action", {
      data: {
        incident_id: incidentId,
        rca_session_id: rcaSessionId,
        description,
        assigned_to: assignedTo,
        due_date: dueDate,
      },
    });
    get().loadCorrectiveActions(incidentId);
  },

  updateCorrectiveAction: async (id, data) => {
    await invoke("update_corrective_action", { id, data });
    const incidentId = get().correctiveActionsIncidentId;
    if (incidentId !== null) {
      await get().loadCorrectiveActions(incidentId);
    }
  },

  deleteCorrectiveAction: async (id) => {
    await invoke("delete_corrective_action", { id });
    const incidentId = get().correctiveActionsIncidentId;
    if (incidentId !== null) {
      await get().loadCorrectiveActions(incidentId);
    }
  },
}));
