import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '@/api'
import type { User, LoginRequest, PolicyState, WorkflowState } from '@/types/api'

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const policyState = ref<PolicyState | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isAuthenticated = computed(() => user.value !== null)
  const currentUser = computed(() => user.value)
  const currentPolicyState = computed(() => policyState.value)

  // Actions
  async function login(credentials: LoginRequest): Promise<boolean> {
    loading.value = true
    error.value = null

    try {
      await authApi.login(credentials)
      await fetchCurrentUser()
      return true
    } catch (err: any) {
      error.value = err.response?.data?.message || 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function logout(): Promise<void> {
    try {
      await authApi.logout()
    } catch (err) {
      console.error('Logout error:', err)
    } finally {
      user.value = null
      policyState.value = null
    }
  }

  async function fetchCurrentUser(): Promise<void> {
    try {
      const currentUser = await authApi.getCurrentUser()
      user.value = currentUser
    } catch (err) {
      console.error('Failed to fetch current user:', err)
      user.value = null
    }
  }

  async function transitionWorkflow(resource: string, target: WorkflowState): Promise<PolicyState | null> {
    try {
      const newState = await authApi.workflowTransition({ resource, target })
      policyState.value = newState
      return newState
    } catch (err) {
      console.error('Workflow transition failed:', err)
      return null
    }
  }

  function setPolicyState(state: PolicyState | null): void {
    policyState.value = state
  }

  function clearPolicyState(): void {
    policyState.value = null
  }

  // Initialize - check if user is already logged in
  async function initialize(): Promise<void> {
    await fetchCurrentUser()
  }

  return {
    // State
    user,
    policyState,
    loading,
    error,

    // Getters
    isAuthenticated,
    currentUser,
    currentPolicyState,

    // Actions
    login,
    logout,
    fetchCurrentUser,
    transitionWorkflow,
    setPolicyState,
    clearPolicyState,
    initialize,
  }
})
