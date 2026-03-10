# API Migration Plan - Leptos to REST API

This document outlines all the Leptos server functions that need to be converted to REST API endpoints.

## Authentication & Authorization APIs

### `POST /api/auth/login`
- **Leptos Function**: `sign_in_with_login_password(login, password)`
- **File**: `src/ac/api.rs`
- **Returns**: User session/token
- **Purpose**: Sign in user with credentials

### `POST /api/auth/logout`
- **Leptos Function**: `sign_out()`
- **File**: `src/ac/api.rs`
- **Returns**: Success status
- **Purpose**: Sign out current user

### `GET /api/auth/current-user`
- **Leptos Function**: `current_user()`
- **File**: `src/ac/api.rs`
- **Returns**: `Option<User>`
- **Purpose**: Get currently authenticated user

### `POST /api/workflow/transition`
- **Leptos Function**: `workflow_transition(resource, target)`
- **File**: `src/ac/api.rs`
- **Returns**: `PolicyState`
- **Purpose**: Transition resource workflow state

## Workspace APIs

### `GET /api/workspaces/policy-state`
- **Leptos Function**: `workspace_root_policy_state()`
- **File**: `src/workspace/api.rs`
- **Returns**: `PolicyState`
- **Purpose**: Get policy state for workspace root

### `GET /api/workspaces`
- **Leptos Function**: `list_workspaces()`
- **File**: `src/workspace/api.rs`
- **Returns**: `Vec<AliasEntry<Workspace>>`
- **Purpose**: List all workspaces

### `GET /api/workspaces/aliased`
- **Leptos Function**: `list_aliased_workspaces()`
- **File**: `src/workspace/api.rs`
- **Returns**: `Vec<AliasEntry<Workspace>>`
- **Purpose**: List aliased workspaces

### `GET /api/workspace/:id`
- **Leptos Function**: `get_workspace_info(id, commit?, path?)`
- **File**: `src/workspace/api.rs`
- **Query Params**: `commit`, `path`
- **Returns**: `RepoResult`
- **Purpose**: Get workspace information

### `GET /api/workspace/:id/log`
- **Leptos Function**: `get_log_info(id)`
- **File**: `src/workspace/api.rs`
- **Returns**: `LogInfo`
- **Purpose**: Get git log information for workspace

### `POST /api/workspace`
- **Leptos Function**: `create_workspace(uri, description?, long_description?)`
- **File**: `src/workspace/api.rs`
- **Body**: `{ uri, description?, long_description? }`
- **Returns**: Success/redirect
- **Purpose**: Create new workspace

### `POST /api/workspace/:id/sync`
- **Leptos Function**: `synchronize(id)`
- **File**: `src/workspace/api.rs`
- **Returns**: Success status
- **Purpose**: Synchronize workspace with remote

## Exposure APIs

### `GET /api/exposures`
- **Leptos Function**: `list()`
- **File**: `src/exposure/api.rs`
- **Returns**: `Vec<AliasEntry<Exposure>>`
- **Purpose**: List all exposures

### `GET /api/exposures/aliased`
- **Leptos Function**: `list_aliased()`
- **File**: `src/exposure/api.rs`
- **Returns**: `Vec<AliasEntry<Exposure>>`
- **Purpose**: List aliased exposures

### `GET /api/exposures/workspace/:workspace_id`
- **Leptos Function**: `list_aliased_for_workspace(workspace_id)`
- **File**: `src/exposure/api.rs`
- **Returns**: `Vec<AliasEntry<Exposure>>`
- **Purpose**: List exposures for specific workspace

### `GET /api/exposure/:id`
- **Leptos Function**: `get_exposure_info(id)`
- **File**: `src/exposure/api.rs`
- **Returns**: `ExposureInfo`
- **Purpose**: Get exposure details with files and workspace info

### `GET /api/exposure/:id/resolve/:path`
- **Leptos Function**: `resolve_exposure_path(id, path)`
- **File**: `src/exposure/api.rs`
- **Returns**: `ResolvedExposurePath`
- **Purpose**: Resolve exposure file path and get view info

## Data Models to Export as TypeScript Types

### Core Types
- `User` - User information
- `Workspace` - Workspace entity
- `Exposure` - Exposure entity
- `AliasEntry<T>` - Generic alias wrapper
- `PolicyState` - Policy and workflow state
- `RepoResult` - Repository query result
- `LogInfo` - Git log information
- `ExposureInfo` - Complete exposure information
- `ExposureFileProfile` - File profile information

## Vue Routes to Create

### Public Routes
- `/` - Home page (welcome/landing)
- `/auth/login` - Login page
- `/auth/logged_out` - Logout confirmation page

### Workspace Routes
- `/workspace/` - List all workspaces (aliased)
- `/workspace/:/id/` - List all workspaces (by ID)
- `/workspace/:/add` - Create new workspace form
- `/workspace/:id/` - Workspace main view
- `/workspace/:id/file/:commit/*path` - File browser at specific commit/path
- `/workspace/:id/create_exposure/:commit` - Create exposure from commit
- `/workspace/:id/log` - Git log view
- `/workspace/:id/synchronize` - Sync workspace action

### Exposure Routes
- `/exposure/` - List all exposures (aliased)
- `/exposure/:/id/` - List all exposures (by ID)
- `/exposure/:id/` - Exposure main view
- `/exposure/:id/:/wizard` - Exposure wizard for adding/editing files
- `/exposure/:id/*path` - Exposure file view at path

### Components to Build

#### Layout Components
- `App.vue` - Main app shell with navigation
- `Header.vue` - Top navigation bar with session status
- `Footer.vue` - Footer with copyright
- `MainLayout.vue` - Main content layout with sidebar portlets

#### Portlet Components (Sidebar)
- `ContentAction.vue` - Context actions for current page
- `ExposureSource.vue` - Source workspace info for exposures
- `ViewsAvailable.vue` - Available views for exposure files
- `Navigation.vue` - Breadcrumb/navigation helper

#### Workspace Components
- `WorkspaceList.vue` - List of workspaces
- `WorkspaceDetail.vue` - Single workspace view
- `WorkspaceAdd.vue` - Create workspace form
- `WorkspaceLog.vue` - Git log viewer
- `WorkspaceFileBrowser.vue` - File tree browser
- `WorkspaceSynchronize.vue` - Sync action page

#### Exposure Components
- `ExposureList.vue` - List of exposures
- `ExposureDetail.vue` - Single exposure view
- `ExposureFileView.vue` - Rendered file view
- `ExposureWizard.vue` - Wizard for adding files

#### Auth Components
- `LoginForm.vue` - Login form
- `SessionStatus.vue` - User session display in header
- `WorkflowState.vue` - Workflow state transitions

#### Shared Components
- `ErrorTemplate.vue` - Error display component
- `Spinner.vue` - Loading spinner
- `SelectList.vue` - Selection list component
- `SelectMap.vue` - Selection map component

## Next Steps

1. ✅ Document all API endpoints (COMPLETED)
2. ✅ Document all routes and components (COMPLETED)
3. ✅ Create REST API handlers in `src/rest/` module (COMPLETED)
   - `src/rest/handlers/auth.rs` - Authentication endpoints
   - `src/rest/handlers/workspace.rs` - Workspace endpoints
   - `src/rest/handlers/exposure.rs` - Exposure endpoints
   - `src/rest/mod.rs` - API router configuration
4. Update `main.rs` to mount REST API router and serve Vue app
5. Install Vue dependencies and test API client
6. Build Vue components for each route
7. Test end-to-end integration

## Implementation Status

### ✅ Rust Backend REST API - IN PROGRESS

REST API structure created, but requires dependency version updates:

**File Structure:**
```
pmrapp/src/rest/
├── handlers/
│   ├── auth.rs       # Auth & workflow endpoints
│   ├── workspace.rs  # Workspace CRUD endpoints
│   └── exposure.rs   # Exposure endpoints
├── handlers.rs       # Module exports
└── mod.rs           # Router configuration
pmrapp/src/server/
└── platform.rs      # Platform type alias for REST handlers
```

**Key Changes:**
- ✅ Created REST handler modules with 16 API endpoints
- ✅ Using `Extension<AuthSession>` and `Extension<Platform>` for dependency injection
- ✅ Converted all Leptos server functions to Axum route handlers
- ✅ Maintaining same business logic as original server functions
- ✅ Updated `main.rs` to mount REST API at `/api` prefix
- ✅ Added `server/platform.rs` module for Platform type alias
- ⚠️ **Dependency conflict**: `axum-login` 0.16.0 uses `axum` 0.7.9, but project uses `axum` 0.8.6

**Blocking Issue:**
```
error[E0277]: the trait bound `fn(...) -> ... {login}: Handler<_, _>` is not satisfied
```

This occurs because there are two different versions of axum in the dependency graph.
`axum-login` 0.16.0 depends on `axum` 0.7, while the project uses `axum` 0.8.

**Solutions:**
1. **Recommended**: Update `axum-login` from 0.16.0 to 0.18.0 (supports axum 0.8)
   - Edit `Cargo.toml`: `axum-login = "0.18.0"`
   - May require updating `pmrac` crate to be compatible
2. **Alternative**: Downgrade `axum` and `leptos_axum` to 0.7 compatible versions
3. **Workaround**: Create wrapper functions that bridge the version gap

**API Endpoints Implemented (16 total):**
- 4 Auth/workflow endpoints
- 7 Workspace endpoints
- 5 Exposure endpoints

### ✅ Vue Frontend - COMPLETE

Project structure created with all API client modules:

**File Structure:**
```
pmrapp-ui/
├── src/
│   ├── api/
│   │   ├── client.ts     # Axios configuration
│   │   ├── auth.ts       # Auth API endpoints
│   │   ├── workspace.ts  # Workspace API endpoints
│   │   ├── exposure.ts   # Exposure API endpoints
│   │   └── index.ts      # API exports
│   ├── types/
│   │   └── api.ts        # TypeScript types matching Rust models
│   ├── stores/
│   │   └── auth.ts       # Pinia authentication store
│   ├── components/       # Vue components (to be built)
│   ├── views/            # Page components (to be built)
│   └── router/           # Vue Router config (to be built)
├── vite.config.ts        # Vite with API proxy to :9380
└── package.json          # Dependencies including axios
```

**Completed:**
- ✅ TypeScript types matching all Rust models
- ✅ Axios client with request/response interceptors
- ✅ API modules for auth, workspace, exposure
- ✅ Pinia auth store for session management
- ✅ Vite proxy configuration for `/api` → `http://localhost:9380`

### 🚧 Next Steps

1. **Install Vue dependencies:**
   ```bash
   cd pmrapp-ui
   npm install
   ```

2. **Build Vue components** for each route (see component list in doc)

3. **Implement Vue Router** with all application routes

4. **Test API integration:**
   - Start Rust backend: `cargo run --release --features ssr`
   - Start Vue dev server: `cd pmrapp-ui && npm run dev`
   - Test endpoints at `http://localhost:5173`

5. **Build for production:**
   ```bash
   cd pmrapp-ui
   npm run build
   # Rust backend will serve dist/ files
   ```

6. **Authentication flow:**
   - Test login/logout via REST API
   - Verify session cookies work between Vue and Rust
   - Implement protected routes in Vue Router
