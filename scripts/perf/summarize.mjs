import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";

const files = {
  bundle: ".perf-results/bundle.json",
  build: ".perf-results/build-time.json",
  memory: ".perf-results/memory.json",
  api: ".perf-results/api-summary.json",
};

const requiredMetrics = ["bundle", "build", "memory"];
const summary = {
  capturedAt: new Date().toISOString(),
  metrics: {},
  status: "pass",
};
for (const [key, file] of Object.entries(files)) {
  if (existsSync(file)) {
    summary.metrics[key] = JSON.parse(readFileSync(file, "utf8"));
  } else {
    summary.metrics[key] = { status: "not-run" };
  }
}

const missingRequired = requiredMetrics.filter(
  (metric) => summary.metrics[metric]?.status === "not-run",
);
if (missingRequired.length > 0) {
  summary.status = "not-run";
}

const failedMetric = Object.values(summary.metrics).some((metric) => {
  const status =
    typeof metric?.status === "string" ? metric.status.toLowerCase() : "";
  return status === "fail" || status === "failed" || status === "error";
});
if (failedMetric) {
  summary.status = "fail";
}

mkdirSync(".perf-results", { recursive: true });
writeFileSync(
  ".perf-results/summary.json",
  `${JSON.stringify(summary, null, 2)}\n`,
);
console.log("wrote .perf-results/summary.json");
