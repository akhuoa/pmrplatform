import apiClient from './client'
import type {
  User,
  LoginRequest,
  LoginResponse,
  PolicyState,
  WorkflowTransitionRequest,
} from '@/types/api'

export const authApi = {
  // POST /api/auth/login
  login: async (credentials: LoginRequest): Promise<LoginResponse> => {
    const { data } = await apiClient.post<LoginResponse>('/auth/login', credentials)
    return data
  },

  // POST /api/auth/logout
  logout: async (): Promise<void> => {
    await apiClient.post('/auth/logout')
  },

  // GET /api/auth/current-user
  getCurrentUser: async (): Promise<User | null> => {
    const { data } = await apiClient.get<User | null>('/auth/current-user')
    return data
  },

  // POST /api/workflow/transition
  workflowTransition: async (request: WorkflowTransitionRequest): Promise<PolicyState> => {
    const { data } = await apiClient.post<PolicyState>('/workflow/transition', request)
    return data
  },
}
