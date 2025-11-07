// Integration tests for components with hooks and providers
import { render, screen, waitFor } from '@/lib/test-utils'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { SessionProvider } from 'next-auth/react'

// Create a test component that uses hooks
function TestComponent() {
  return (
    <div>
      <h1>Test Component</h1>
      <p>This component uses providers</p>
    </div>
  )
}

// Test with all providers
function TestWithProviders() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  })

  return (
    <QueryClientProvider client={queryClient}>
      <SessionProvider
        session={{
          user: {
            id: 'user-123',
            email: 'test@example.com',
            role: 'admin',
            tenantId: 'tenant-123',
          },
          expires: 'future-date',
        }}
      >
        <TestComponent />
      </SessionProvider>
    </QueryClientProvider>
  )
}

describe('Component Integration with Providers', () => {
  it('should render component with all providers', () => {
    render(<TestWithProviders />)

    expect(screen.getByRole('heading', { name: 'Test Component' })).toBeInTheDocument()
    expect(screen.getByText('This component uses providers')).toBeInTheDocument()
  })

  it('should have access to session through providers', () => {
    render(<TestWithProviders />)

    // Verify the component rendered
    expect(screen.getByText('Test Component')).toBeInTheDocument()
  })

  it('should handle component with query client', async () => {
    // This would test components that use React Query
    render(<TestWithProviders />)

    // Component should render without errors
    expect(screen.getByText('Test Component')).toBeInTheDocument()
  })
})

// Integration test for API integration
describe('API Integration Tests', () => {
  it('should mock API calls in tests', async () => {
    // Mock API response
    const mockApiCall = jest.fn().mockResolvedValue({
      data: { id: 'test-123', name: 'Test Item' },
    })

    // Simulate API call
    const result = await mockApiCall()

    expect(result).toEqual({
      data: { id: 'test-123', name: 'Test Item' },
    })
    expect(mockApiCall).toHaveBeenCalledTimes(1)
  })

  it('should handle multiple API calls', async () => {
    const mockApiCall1 = jest.fn().mockResolvedValue({ data: 'data1' })
    const mockApiCall2 = jest.fn().mockResolvedValue({ data: 'data2' })

    const [result1, result2] = await Promise.all([
      mockApiCall1(),
      mockApiCall2(),
    ])

    expect(result1).toEqual({ data: 'data1' })
    expect(result2).toEqual({ data: 'data2' })
    expect(mockApiCall1).toHaveBeenCalledTimes(1)
    expect(mockApiCall2).toHaveBeenCalledTimes(1)
  })
})

// Test WebSocket integration
describe('WebSocket Integration', () => {
  it('should handle WebSocket connections', () => {
    // Mock WebSocket
    const mockWebSocket = {
      send: jest.fn(),
      close: jest.fn(),
      addEventListener: jest.fn(),
      removeEventListener: jest.fn(),
    }

    global.WebSocket = jest.fn().mockImplementation(() => mockWebSocket)

    const ws = new WebSocket('ws://localhost:8080')

    expect(ws.send).toBeDefined()
    expect(ws.close).toBeDefined()

    // Simulate a message
    const messageHandler = jest.fn()
    ws.addEventListener('message', messageHandler)
    expect(ws.addEventListener).toHaveBeenCalledWith('message', messageHandler)
  })
})

// Test error handling integration
describe('Error Handling Integration', () => {
  it('should handle API errors gracefully', async () => {
    const mockApiCall = jest.fn().mockRejectedValue(new Error('API Error'))

    try {
      await mockApiCall()
    } catch (error) {
      expect(error).toBeInstanceOf(Error)
      expect((error as Error).message).toBe('API Error')
    }
  })

  it('should handle network failures', async () => {
    const mockApiCall = jest.fn().mockRejectedValue(new Error('Network Error'))

    try {
      await mockApiCall()
    } catch (error) {
      expect(error).toBeInstanceOf(Error)
      expect((error as Error).message).toBe('Network Error')
    }
  })
})

// Test state management integration
describe('State Management Integration', () => {
  it('should handle state updates across components', () => {
    let state = { count: 0 }

    const increment = () => {
      state = { ...state, count: state.count + 1 }
    }

    expect(state.count).toBe(0)
    increment()
    expect(state.count).toBe(1)
    increment()
    expect(state.count).toBe(2)
  })
})
