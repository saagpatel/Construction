# Construction Safety Tracker

A desktop application for managing workplace safety incidents, conducting root cause analysis, generating OSHA compliance reports, and tracking corrective actions. Built for construction firms to maintain safety excellence and regulatory compliance.

**Status**: Release-candidate (core workflows) | **Rust Tests**: 17/17 passing

---

## ✨ Key Features

### 📋 Incident Management

- Complete incident CRUD with multi-step create wizard and edit route
- Employee information and injury/illness classification
- Auto-assign case numbers per establishment per year
- Privacy case handling (name masking)
- Attachment support (photos, audio, documents)
- Status tracking (open, in review, closed)

### 📊 OSHA Compliance

- **OSHA 300 Log** - Injury and illness record
- **OSHA 300A Summary** - Annual summary
- **OSHA 301 Report** - Individual incident details
- CSV export for OSHA 300, 300A, and 301 forms
- Auto-calculation of TRIR (Total Recordable Incident Rate)
- Annual statistics management (employee count, hours worked)

### 🔍 Root Cause Analysis

- **5 Whys Method** - Step-by-step guided analysis
- **Fishbone Diagram** - Multi-category cause mapping
  - Manpower, Methods, Materials, Machinery, Environment, Management
- Session management and completion tracking
- Visual diagram rendering

### 🛡️ Safety Programs

- **Toolbox Talks** - Pre-seeded safety topics with digital signatures
- **Job Safety Analysis (JSA/JHA)** - Reusable templates with review/approval workflow
- **Safety Inspections** - Data model + seed templates in place (app workflows pending)
- **Near Miss Reporting** - Data model in place (app workflows pending)

### 📚 Compliance & Training

- Training records schema + seeded OSHA courses in place (UI/commands pending)
- Equipment tracking schema in place (UI/commands pending)
- Trade-specific hazard library schema + seed data in place

### 📱 Field-Ready Features

- **Touch-optimized UI** - 44x44px minimum touch targets
- **Offline Support** - Auto-detection with visual indicator
- **Attachment Workflow** - Incident-level upload, grouping, and delete actions
- **Keyboard Shortcuts** - Cmd+N (new incident), Cmd+K (search), more
- **Error Recovery** - ErrorBoundary with reload button

### 👥 Multi-User Support

- Sync/auth/audit tables and role scaffolding are present in schema
- Runtime auth, RBAC enforcement, and sync execution are planned follow-on work

### 📈 Dashboard

- Incidents by month (line chart)
- Incidents by severity (bar chart)
- Incidents by location (bar chart)
- Top hazard categories (pie chart)
- Days since last injury counter
- Corrective action status (open vs closed)

---

## 🚀 Quick Start

### Prerequisites

- **macOS** (10.15+) or Linux/Windows
- **Node.js** 18+
- **Rust** 1.70+
- **pnpm** (npm install -g pnpm)

### Installation

```bash
# Clone repository
git clone https://github.com/yourusername/construction-safety-tracker.git
cd construction-safety-tracker

# Install dependencies
pnpm install

# Install Rust dependencies
cd src-tauri && cargo build && cd ..
```

### Development

```bash
# Normal dev mode (fastest incremental rebuilds, uses local build caches)
pnpm dev:normal

# Lean dev mode (minimal local disk growth, slower restarts)
pnpm dev:lean

# Tauri frontend URL
# http://localhost:1420
```

### Cleanup Commands

```bash
# Remove heavy build artifacts only (keeps dependencies for speed)
pnpm clean:heavy

# Remove all local reproducible caches (includes node_modules)
pnpm clean:all-local
```

### Normal vs Lean Dev Tradeoffs

- `pnpm dev:normal`: fastest hot reload and rebuilds, but can grow `src-tauri/target` and `node_modules/.vite`.
- `pnpm dev:lean`: uses temporary cache locations for Rust and Vite, then cleans heavy artifacts automatically when you exit; uses less disk but has slower startup/rebuild times.

### Build

```bash
# Build production version
pnpm tauri build

# Output: src-tauri/target/release/bundle/
# - macOS: .dmg and .app
# - Linux: .deb and AppImage
# - Windows: .msi and .exe
```

---

## 🧪 Testing

### Run All Tests

```bash
# Canonical verify contract (local + CI parity)
bash .codex/scripts/run_verify_commands.sh

# Rust tests only
cd src-tauri && cargo test

# Rust lint gate
cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings
```

### Test Coverage

- **Rust**: 17 tests covering DB operations, OSHA calculations, validation, and toolbox signature flow
- **Type Safety**: TypeScript strict mode, 100% coverage

### Notes

- Canonical quality gate commands live in `.codex/verify.commands`.
- CI quality gates are split by concern (`quality-gates`, `git-hygiene`, `lockfile-rationale`, `perf-*`) and align to the same release criteria.

---

## 📦 Tech Stack

| Layer              | Technology          | Version     |
| ------------------ | ------------------- | ----------- |
| **Framework**      | Tauri 2             | Latest      |
| **Frontend**       | React 19            | Latest      |
| **Language**       | TypeScript          | Strict mode |
| **Styling**        | Tailwind CSS 4      | Latest      |
| **State**          | Zustand             | 5.x         |
| **Charts**         | Recharts            | Latest      |
| **Backend**        | Rust                | 1.70+       |
| **Database**       | SQLite via rusqlite | 3.x         |
| **Error Handling** | thiserror + anyhow  | Latest      |
| **Testing**        | Rust unit tests     | 17 tests    |
| **Routing**        | React Router 7      | Latest      |

---

## 📂 Project Structure

```
construction-safety-tracker/
├── src/                          # React frontend
│   ├── components/
│   │   ├── dashboard/            # Safety dashboard
│   │   ├── incidents/            # Incident CRUD
│   │   ├── osha/                 # OSHA forms
│   │   ├── rca/                  # Root cause analysis
│   │   ├── settings/             # Establishment setup
│   │   ├── toolbox/              # Toolbox talks
│   │   └── ui/                   # Shared components
│   ├── hooks/                    # Custom hooks (keyboard, toast)
│   ├── stores/                   # Zustand state stores
│   ├── lib/                      # Types, constants, utilities
│   └── App.tsx
│
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # Tauri command handlers (50+)
│   │   ├── db/
│   │   │   ├── mod.rs            # DB connection, migrations
│   │   │   ├── migrations/       # 14 SQL migrations (49 tables)
│   │   │   ├── incidents.rs
│   │   │   ├── locations.rs
│   │   │   ├── osha.rs
│   │   │   ├── rca.rs
│   │   │   ├── toolbox.rs
│   │   │   └── jsa.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── errors.rs
│   │   └── validation.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── README.md                     # User-facing documentation
└── package.json
```

---

## 🔐 Security

### Current Security Controls

- ✅ **Database integrity** - Foreign key constraints and relational safeguards
- ✅ **Input validation** - Validation in command and domain layers
- ✅ **Path traversal prevention** - Filename sanitization on uploads
- ✅ **Parameterized SQL** - Query parameterization across DB operations
- ✅ **File upload limits** - Type validation and file-size cap (50MB)

### Security Notes

- Auth/session and audit-related schema scaffolding exists in migrations.
- Runtime auth/RBAC enforcement is planned follow-on work and is not currently active in the app runtime.

---

## 💾 Database

### Schema Overview

- **49 tables** across 14 migrations
- **SQLite** for local-first reliability
- **Foreign key constraints** for referential integrity
- **Indexes** for query performance
- **Seed data** for 50+ pre-populated records

### Key Tables

| Table              | Purpose                              |
| ------------------ | ------------------------------------ |
| establishments     | Company info (multi-location)        |
| locations          | Job sites                            |
| incidents          | Injury/illness records (OSHA 300)    |
| rca_sessions       | Root cause analysis sessions         |
| five_whys_steps    | 5 Whys analysis steps                |
| fishbone\_\*       | Fishbone diagram data                |
| corrective_actions | Corrective action tracking           |
| annual_stats       | Workforce data for OSHA 300A         |
| toolbox_talks      | Safety talks with attendance         |
| jsa\_\*            | Job Safety Analysis data             |
| inspections        | Safety inspection checklists         |
| near_miss_reports  | Near miss incidents                  |
| training_records   | Employee training history            |
| equipment\_\*      | Equipment tracking                   |
| users, sessions    | User auth (ready for implementation) |
| audit_log          | Compliance audit trail               |

---

## 📋 Usage Examples

### Creating an Incident

1. Click **"New Incident"** or press **Cmd+N**
2. Step 1: Enter date, location, employee info
3. Step 2: Describe what happened
4. Step 3: Select injury/illness type and severity
5. Step 4: Add healthcare facility info (optional)
6. After save, add attachments (photos, audio notes, or documents) from the incident detail page

### Running Root Cause Analysis

1. Open incident → click **"Root Cause Analysis"**
2. Choose **5 Whys** or **Fishbone Diagram**
3. Answer guided questions or map causes
4. Mark root causes
5. Create corrective actions
6. Mark the RCA session complete and track corrective actions

### Generating OSHA Reports

1. Go to **OSHA Forms** tab
2. Select year and form (300, 300A, or 301)
3. Review data
4. Click **"Export CSV"** on OSHA 300 log
5. Open in Excel or upload to OSHA website

### Conducting Toolbox Talk

1. Go to **Toolbox Talks**
2. Click **"Schedule Talk"**
3. Select topic (pre-populated list)
4. Set date and conductor
5. Add attendees
6. Have each person **sign** using signature pad
7. Mark **Completed** when done

---

## 🐛 Troubleshooting

### App won't start

```bash
# Clear cache and rebuild
rm -rf src-tauri/target
cargo clean
pnpm tauri dev
```

### Database locked error

- Close all instances of the app
- Check for orphaned database processes
- Ensure disk has write permissions

### Permission errors on macOS

```bash
# Grant execution permission
chmod +x ./target/release/construction-safety-tracker
```

### Type errors in React

```bash
# Ensure strict mode is enabled
cat tsconfig.json | grep '"strict"'
# Should show: "strict": true
```

---

## 📝 Documentation

- **[README.md](./README.md)** - Setup, features, and project structure
- **[docs/source-of-truth/README.md](./docs/source-of-truth/README.md)** - Canonical release and operations references
- **[docs/release/go-no-go-checklist.md](./docs/release/go-no-go-checklist.md)** - Final ship decision gate
- **[docs/runbooks/release-and-rollback.md](./docs/runbooks/release-and-rollback.md)** - Release and rollback procedure

---

## 🎯 Roadmap

### Completed

- ✅ Touch-optimized UI and offline support
- ✅ File attachments (photos, voice, documents)
- ✅ Safety programs in app runtime (toolbox talks, JSA)
- ✅ Multi-user schema foundation (auth, sync, audit tables)
- ✅ Trade-specific hazards seed data in schema

### In Progress / Planned

- [ ] Safety inspections command + UI workflows
- [ ] Near miss reporting command + UI workflows
- [ ] Training records command + UI workflows
- [ ] Equipment tracking command + UI workflows
- [ ] Runtime auth/RBAC enforcement
- [ ] AI-powered incident classification (Claude API)
- [ ] Predictive analytics for high-risk areas
- [ ] Mobile app (iOS/Android via Tauri)
- [ ] Cloud sync for multi-device runtime
- [ ] Advanced reporting with custom dashboards
- [ ] Integration with OSHA reporting systems

---

## 🤝 Contributing

Contributions welcome! Please:

1. Create a branch with the required prefix: `codex/<type>/<slug>`
2. Run the canonical verify contract: `bash .codex/scripts/run_verify_commands.sh`
3. Commit with conventional commits: `git commit -m "feat: ..."`
4. Push and create a pull request

### Code Standards

- **Rust**: No `unwrap()` in production code
- **React**: Functional components only
- **TypeScript**: Strict mode, no `any` types
- **Verification**: Ensure `.codex/verify.commands` passes before PR

---

## 📄 License

MIT - See LICENSE file for details

---

## 👤 Support

For questions or issues:

- Check the [Documentation](#documentation) section
- Review [Troubleshooting](#troubleshooting) guide
- Open a GitHub issue

---

## 📊 Statistics

- **Lines of Code**: 20,000+
- **Rust Backend**: ~5,000 lines
- **React Frontend**: ~8,000 lines
- **Database Migrations**: 14 files (49 tables)
- **Tauri Commands**: 50+
- **Rust Tests**: 17 passing
- **Development Time**: ~3 weeks
- **Security Posture**: Core controls enabled; auth/RBAC runtime pending

---

**Built with ❤️ for construction safety professionals**

Last updated: 2026-03-01 | Version: 1.0.0
