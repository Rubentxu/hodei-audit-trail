// Test utilities and helpers
import { ReactElement } from 'react'
import { render, RenderOptions } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

// Create a test query client
function createTestQueryClient() {
  return new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        gcTime: 0,
      },
      mutations: {
        retry: false,
      },
    },
  })
}

// Custom render function that includes providers
interface CustomRenderOptions extends Omit<RenderOptions, 'wrapper'> {
  queryClient?: QueryClient
  wrapper?: React.ComponentType<{ children: React.ReactNode }>
}

function customRender(
  ui: ReactElement,
  {
    queryClient = createTestQueryClient(),
    wrapper: Wrapper = ({ children }) => (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    ),
    ...renderOptions
  }: CustomRenderOptions = {}
) {
  return render(ui, { wrapper: Wrapper, ...renderOptions })
}

// Re-export everything
export * from '@testing-library/react'

// Override render method
export { customRender as render }

// Test data factories
export const createMockUser = (overrides = {}) => ({
  id: 'test-user-1',
  email: 'test@example.com',
  name: 'Test User',
  role: 'viewer',
  tenantId: 'test-tenant',
  ...overrides,
})

export const createMockEvent = (overrides = {}) => ({
  id: 'evt-123',
  eventSource: 'test-service',
  eventName: 'TestAction',
  awsRegion: 'us-east-1',
  userIdentity: {
    type: 'User',
    principalId: 'user-123',
    arn: 'arn:aws:iam::123456789012:user/test',
    accountId: '123456789012',
  },
  sourceIPAddress: '192.168.1.1',
  userAgent: 'TestAgent/1.0',
  eventTime: new Date().toISOString(),
  readOnly: true,
  resources: [],
  ...overrides,
})

export const createMockAnalytics = (overrides = {}) => ({
  totalEvents: 1000,
  uniqueUsers: 50,
  topEventSources: [
    { name: 'ec2', count: 300 },
    { name: 's3', count: 250 },
    { name: 'lambda', count: 200 },
  ],
  eventsOverTime: Array.from({ length: 24 }, (_, i) => ({
    hour: i,
    count: Math.floor(Math.random() * 100),
  })),
  ...overrides,
})

export const createMockCompliance = (overrides = {}) => ({
  id: 'report-123',
  name: 'Test Compliance Report',
  description: 'Test description',
  status: 'ready',
  generatedAt: new Date().toISOString(),
  findings: [
    {
      id: 'finding-1',
      severity: 'high',
      title: 'Test Finding',
      description: 'Test finding description',
      resource: 'arn:aws:s3:::test-bucket',
    },
  ],
  ...overrides,
})

// Mock API responses
export const mockApiResponse = {
  events: {
    events: Array.from({ length: 10 }, (_, i) => createMockEvent({ id: `evt-${i}` })),
    total: 100,
    page: 1,
    perPage: 10,
  },
  analytics: createMockAnalytics(),
  compliance: {
    reports: Array.from({ length: 5 }, (_, i) => createMockCompliance({ id: `report-${i}` })),
    total: 5,
  },
  auth: {
    user: createMockUser(),
    session: {
      accessToken: 'mock-token',
      expiresAt: Date.now() + 3600000,
    },
  },
}

// Mock API module
export const mockApi = {
  getEvents: jest.fn().mockResolvedValue(mockApiResponse.events),
  getAnalytics: jest.fn().mockResolvedValue(mockApiResponse.analytics),
  getComplianceReports: jest.fn().mockResolvedValue(mockApiResponse.compliance),
  login: jest.fn().mockResolvedValue(mockApiResponse.auth),
  logout: jest.fn().mockResolvedValue({ success: true }),
}

// Setup function for tests
export const setupTests = () => {
  // Suppress console errors in tests
  const originalError = console.error
  console.error = (...args) => {
    if (
      typeof args[0] === 'string' &&
      (args[0].includes('Warning:') || args[0].includes('ReactDOM.render'))
    ) {
      return
    }
    originalError.call(console, ...args)
  }

  // Mock IntersectionObserver
  global.IntersectionObserver = jest.fn().mockImplementation(() => ({
    observe: jest.fn(),
    unobserve: jest.fn(),
    disconnect: jest.fn(),
  }))

  // Mock matchMedia
  Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: jest.fn().mockImplementation((query) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: jest.fn(),
      removeListener: jest.fn(),
      addEventListener: jest.fn(),
      removeEventListener: jest.fn(),
      dispatchEvent: jest.fn(),
    })),
  })
}

// Cleanup function
export const cleanupTests = () => {
  jest.clearAllMocks()
}
