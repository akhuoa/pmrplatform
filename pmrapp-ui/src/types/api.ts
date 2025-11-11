// Core API Types - matching Rust pmrcore models

export interface User {
  id: number
  login: string
  name?: string
  email?: string
}

export interface Workspace {
  id: number
  url: string
  description?: string
  long_description?: string
  created_ts: number
}

export interface Exposure {
  id: number
  workspace_id: number
  commit_id: string
  created_ts: number
  description?: string
}

export interface AliasEntry<T> {
  alias: string
  entity: T
}

export interface PolicyState {
  policy?: Policy
  state: WorkflowState
}

export interface Policy {
  agent: Agent
  resource: string
  roles: Role[]
  permissions: string[]
}

export type Agent =
  | { type: 'User', user: User }
  | { type: 'Anonymous' }

export type Role = 'Owner' | 'Manager' | 'Viewer'

export type WorkflowState = 'Private' | 'Pending' | 'Published'

export interface RepoResult {
  workspace: Workspace
  commit?: string
  path?: string
  target?: PathObjectInfo
}

export interface PathObjectInfo {
  name: string
  fullpath: string
  kind: 'File' | 'Dir' | 'Submodule'
  size?: number
  commit: string
  treeinfo?: TreeInfo
}

export interface TreeInfo {
  entries: PathObjectInfo[]
}

export interface LogInfo {
  entries: LogEntryInfo[]
  next_cursor?: string
}

export interface LogEntryInfo {
  commit: string
  author: string
  committer: string
  author_ts: number
  commit_ts: number
  summary: string
}

export interface ExposureInfo {
  exposure: Exposure
  exposure_alias?: string
  files: [string, boolean][]
  workspace: Workspace
  workspace_alias?: string
}

export interface ExposureFile {
  id: number
  exposure_id: number
  workspace_file_path: string
  created_ts: number
}

export interface ExposureFileView {
  id: number
  exposure_file_id: number
  view_key: string
  created_ts: number
}

export interface ExposureFileProfile {
  task_id: number
  description: string
  view_key: string
}

export type ResolvedExposurePath =
  | { type: 'Target', file: ExposureFile, view: ExposureFileViewResult }
  | { type: 'Redirect', path: string }

export type ExposureFileViewResult =
  | { type: 'Ok', view: ExposureFileView, viewPath?: string }
  | { type: 'Err', availableViews: string[] }

// API Request/Response types

export interface LoginRequest {
  login: string
  password: string
}

export interface LoginResponse {
  message: string
}

export interface CreateWorkspaceRequest {
  uri: string
  description?: string
  long_description?: string
}

export interface WorkflowTransitionRequest {
  resource: string
  target: WorkflowState
}

// Enforced response wrapper
export interface EnforcedOk<T> {
  inner: T
  policy_state: PolicyState
}
