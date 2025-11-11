import axios, type { AxiosInstance } from 'axios'

// Create axios instance with base configuration
const apiClient: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_URL || '/api',
  headers: {
    'Content-Type': 'application/json',
  },
  withCredentials: true, // Important for session cookies
})

// Request interceptor
apiClient.interceptors.request.use(
  (config) => {
    // You can add auth tokens here if needed
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// Response interceptor
apiClient.interceptors.response.use(
  (response) => {
    return response
  },
  (error) => {
    // Handle common errors
    if (error.response) {
      switch (error.response.status) {
        case 401:
          // Unauthorized - redirect to login
          window.location.href = '/auth/login'
          break
        case 403:
          console.error('Forbidden:', error.response.data)
          break
        case 404:
          console.error('Not found:', error.response.data)
          break
        case 500:
          console.error('Server error:', error.response.data)
          break
      }
    }
    return Promise.reject(error)
  }
)

export default apiClient
