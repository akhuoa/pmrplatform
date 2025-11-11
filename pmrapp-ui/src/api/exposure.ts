import apiClient from './client'
import type {
  Exposure,
  AliasEntry,
  ExposureInfo,
  ResolvedExposurePath,
  EnforcedOk,
} from '@/types/api'

export const exposureApi = {
  // GET /api/exposures
  list: async (): Promise<EnforcedOk<AliasEntry<Exposure>[]>> => {
    const { data } = await apiClient.get<EnforcedOk<AliasEntry<Exposure>[]>>('/exposures')
    return data
  },

  // GET /api/exposures/aliased
  listAliased: async (): Promise<EnforcedOk<AliasEntry<Exposure>[]>> => {
    const { data } = await apiClient.get<EnforcedOk<AliasEntry<Exposure>[]>>('/exposures/aliased')
    return data
  },

  // GET /api/exposures/workspace/:workspace_id
  listForWorkspace: async (workspaceId: string | number): Promise<AliasEntry<Exposure>[]> => {
    const { data } = await apiClient.get<AliasEntry<Exposure>[]>(`/exposures/workspace/${workspaceId}`)
    return data
  },

  // GET /api/exposure/:id
  get: async (id: string | number): Promise<EnforcedOk<ExposureInfo>> => {
    const { data } = await apiClient.get<EnforcedOk<ExposureInfo>>(`/exposure/${id}`)
    return data
  },

  // GET /api/exposure/:id/resolve/:path
  resolvePath: async (id: number, path: string): Promise<EnforcedOk<ResolvedExposurePath>> => {
    const { data } = await apiClient.get<EnforcedOk<ResolvedExposurePath>>(`/exposure/${id}/resolve/${path}`)
    return data
  },
}
