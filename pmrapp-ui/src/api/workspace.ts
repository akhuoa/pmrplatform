import apiClient from './client'
import type {
  Workspace,
  AliasEntry,
  PolicyState,
  RepoResult,
  LogInfo,
  EnforcedOk,
  CreateWorkspaceRequest,
} from '@/types/api'

export const workspaceApi = {
  // GET /api/workspaces/policy-state
  getPolicyState: async (): Promise<PolicyState> => {
    const { data } = await apiClient.get<PolicyState>('/workspaces/policy-state')
    return data
  },

  // GET /api/workspaces
  list: async (): Promise<EnforcedOk<AliasEntry<Workspace>[]>> => {
    const { data } = await apiClient.get<EnforcedOk<AliasEntry<Workspace>[]>>('/workspaces')
    return data
  },

  // GET /api/workspaces/aliased
  listAliased: async (): Promise<EnforcedOk<AliasEntry<Workspace>[]>> => {
    const { data } = await apiClient.get<EnforcedOk<AliasEntry<Workspace>[]>>('/workspaces/aliased')
    return data
  },

  // GET /api/workspace/:id
  get: async (id: string | number, commit?: string, path?: string): Promise<EnforcedOk<RepoResult>> => {
    const params = new URLSearchParams()
    if (commit) params.append('commit', commit)
    if (path) params.append('path', path)

    const { data } = await apiClient.get<EnforcedOk<RepoResult>>(
      `/workspace/${id}${params.toString() ? '?' + params.toString() : ''}`
    )
    return data
  },

  // GET /api/workspace/:id/log
  getLog: async (id: string | number): Promise<EnforcedOk<LogInfo>> => {
    const { data } = await apiClient.get<EnforcedOk<LogInfo>>(`/workspace/${id}/log`)
    return data
  },

  // POST /api/workspace
  create: async (request: CreateWorkspaceRequest): Promise<void> => {
    await apiClient.post('/workspace', request)
  },

  // POST /api/workspace/:id/sync
  sync: async (id: number): Promise<void> => {
    await apiClient.post(`/workspace/${id}/sync`)
  },
}
