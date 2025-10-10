import { config } from '@/config'
import { http as mockHttp } from '@/api/mockClient'

// Real HTTP client (axios or fetch)
const realHttp = {
  async get<T = any>(url: string): Promise<{ data: T }> {
    const response = await fetch(`${config.apiUrl}${url}`)
    const data = await response.json()
    return { data }
  },
  
  async post<T = any>(url: string, body: any): Promise<{ data: T }> {
    const response = await fetch(`${config.apiUrl}${url}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    })
    const data = await response.json()
    return { data }
  },
  
  async put<T = any>(url: string, body: any): Promise<{ data: T }> {
    const response = await fetch(`${config.apiUrl}${url}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    })
    const data = await response.json()
    return { data }
  },
  
  async delete<T = any>(url: string): Promise<{ data: T }> {
    const response = await fetch(`${config.apiUrl}${url}`, {
      method: 'DELETE'
    })
    const data = await response.json()
    return { data }
  }
}

// Export the appropriate client based on config
export const http = config.mockMode ? mockHttp : realHttp
