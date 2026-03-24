import {
  BrowserRouter,
  Routes,
  Route,
  Navigate,
  useNavigate,
} from "react-router-dom";
import { lazy, Suspense, useEffect } from "react";
import { Layout } from "./components/ui/Layout";
import { ErrorBoundary } from "./components/ui/ErrorBoundary";
import { NetworkStatus } from "./components/ui/NetworkStatus";
import { ToastContainer } from "./components/ui/Toast";
import { SetupWizard } from "./components/settings/SetupWizard";
import { useSettingsStore } from "./stores/settingsStore";
import {
  useKeyboardShortcuts,
  createGlobalShortcuts,
} from "./hooks/useKeyboardShortcuts";
import "./index.css";

const DashboardPage = lazy(() =>
  import("./components/dashboard/DashboardPage").then((module) => ({
    default: module.DashboardPage,
  })),
);
const IncidentListPage = lazy(() =>
  import("./components/incidents/IncidentListPage").then((module) => ({
    default: module.IncidentListPage,
  })),
);
const IncidentCreatePage = lazy(() =>
  import("./components/incidents/IncidentCreatePage").then((module) => ({
    default: module.IncidentCreatePage,
  })),
);
const IncidentDetailPage = lazy(() =>
  import("./components/incidents/IncidentDetailPage").then((module) => ({
    default: module.IncidentDetailPage,
  })),
);
const IncidentEditPage = lazy(() =>
  import("./components/incidents/IncidentEditPage").then((module) => ({
    default: module.IncidentEditPage,
  })),
);
const RcaPage = lazy(() =>
  import("./components/rca/RcaPage").then((module) => ({
    default: module.RcaPage,
  })),
);
const OshaPage = lazy(() =>
  import("./components/osha/OshaPage").then((module) => ({
    default: module.OshaPage,
  })),
);
const SettingsPage = lazy(() =>
  import("./components/settings/SettingsPage").then((module) => ({
    default: module.SettingsPage,
  })),
);
const ImportPage = lazy(() =>
  import("./components/import/ImportPage").then((module) => ({
    default: module.ImportPage,
  })),
);
const ToolboxTalksPage = lazy(() =>
  import("./pages/ToolboxTalksPage").then((module) => ({
    default: module.ToolboxTalksPage,
  })),
);
const ToolboxCreatePage = lazy(() =>
  import("./pages/ToolboxCreatePage").then((module) => ({
    default: module.ToolboxCreatePage,
  })),
);
const ToolboxTalkDetailPage = lazy(() =>
  import("./pages/ToolboxTalkDetailPage").then((module) => ({
    default: module.ToolboxTalkDetailPage,
  })),
);
const JsaPage = lazy(() =>
  import("./pages/JsaPage").then((module) => ({ default: module.JsaPage })),
);

function RouteSkeleton() {
  return (
    <div className="flex items-center justify-center h-[60vh]">
      <div className="text-center">
        <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-safety-orange mx-auto" />
        <p className="mt-3 text-sm text-gray-600">Loading page...</p>
      </div>
    </div>
  );
}

function AppRoutes() {
  const navigate = useNavigate();

  // Enable global keyboard shortcuts with React Router navigation
  useKeyboardShortcuts(createGlobalShortcuts(navigate));

  return (
    <Layout>
      <Suspense fallback={<RouteSkeleton />}>
        <Routes>
          <Route path="/" element={<Navigate to="/dashboard" replace />} />
          <Route path="/dashboard" element={<DashboardPage />} />
          <Route path="/incidents" element={<IncidentListPage />} />
          <Route path="/incidents/new" element={<IncidentCreatePage />} />
          <Route path="/incidents/:id" element={<IncidentDetailPage />} />
          <Route path="/incidents/:id/edit" element={<IncidentEditPage />} />
          <Route path="/incidents/:id/rca" element={<RcaPage />} />
          <Route path="/osha" element={<OshaPage />} />
          <Route path="/import" element={<ImportPage />} />
          <Route path="/toolbox" element={<ToolboxTalksPage />} />
          <Route path="/toolbox/new" element={<ToolboxCreatePage />} />
          <Route path="/toolbox/:id" element={<ToolboxTalkDetailPage />} />
          <Route path="/jsa" element={<JsaPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </Suspense>
    </Layout>
  );
}

function App() {
  const { loadEstablishments, establishments, loading } = useSettingsStore();

  useEffect(() => {
    loadEstablishments();
  }, [loadEstablishments]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen bg-gray-50">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-safety-orange mx-auto" />
          <p className="mt-4 text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

  if (!loading && establishments.length === 0) {
    return <SetupWizard />;
  }

  return (
    <ErrorBoundary>
      <BrowserRouter>
        <NetworkStatus />
        <ToastContainer />
        <AppRoutes />
      </BrowserRouter>
    </ErrorBoundary>
  );
}

export default App;
