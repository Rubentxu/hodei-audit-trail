// Tests for QueryProvider component
import { render, screen } from '@/lib/test-utils'
import { QueryProvider } from '../query-provider'
import { QueryClientProvider } from '@tanstack/react-query'

// Mock dependencies
jest.mock('@/lib/api/hooks', () => ({
  queryClientConfig: {
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  },
}))

jest.mock('@tanstack/react-query-devtools', () => ({
  ReactQueryDevtools: ({ initialIsOpen }: { initialIsOpen: boolean }) => (
    <div data-testid="react-query-devtools">Devtools</div>
  ),
}))

describe('QueryProvider', () => {
  it('should render children correctly', () => {
    render(
      <QueryProvider>
        <div data-testid="child-component">Test Content</div>
      </QueryProvider>
    )

    expect(screen.getByTestId('child-component')).toBeInTheDocument()
    expect(screen.getByText('Test Content')).toBeInTheDocument()
  })

  it('should render devtools in development mode', () => {
    // Store original NODE_ENV
    const originalEnv = process.env.NODE_ENV
    process.env.NODE_ENV = 'development'

    render(
      <QueryProvider>
        <div>Test</div>
      </QueryProvider>
    )

    expect(screen.getByTestId('react-query-devtools')).toBeInTheDocument()

    // Restore NODE_ENV
    process.env.NODE_ENV = originalEnv
  })

  it('should not render devtools in production mode', () => {
    // Store original NODE_ENV
    const originalEnv = process.env.NODE_ENV
    process.env.NODE_ENV = 'production'

    render(
      <QueryProvider>
        <div>Test</div>
      </QueryProvider>
    )

    expect(screen.queryByTestId('react-query-devtools')).not.toBeInTheDocument()

    // Restore NODE_ENV
    process.env.NODE_ENV = originalEnv
  })

  it('should handle nested components', () => {
    render(
      <QueryProvider>
        <div>
          <h1>Parent Component</h1>
          <span>Child span</span>
        </div>
      </QueryProvider>
    )

    expect(screen.getByRole('heading', { name: 'Parent Component' })).toBeInTheDocument()
    expect(screen.getByText('Child span')).toBeInTheDocument()
  })
})
