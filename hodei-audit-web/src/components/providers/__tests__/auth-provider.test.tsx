// Tests for AuthProvider component
import { render, screen } from '@/lib/test-utils'
import { AuthProvider } from '../auth-provider'
import { SessionProvider } from 'next-auth/react'

// Mock SessionProvider to avoid NextAuth complexity in unit tests
jest.mock('next-auth/react', () => ({
  SessionProvider: ({ children }: { children: React.ReactNode }) => (
    <div data-testid="mock-session-provider">{children}</div>
  ),
}))

describe('AuthProvider', () => {
  it('should render children correctly', () => {
    render(
      <AuthProvider>
        <div data-testid="child-component">Test Child</div>
      </AuthProvider>
    )

    expect(screen.getByTestId('child-component')).toBeInTheDocument()
    expect(screen.getByText('Test Child')).toBeInTheDocument()
  })

  it('should render SessionProvider with children', () => {
    render(
      <AuthProvider>
        <span>Session Content</span>
      </AuthProvider>
    )

    expect(screen.getByTestId('mock-session-provider')).toBeInTheDocument()
    expect(screen.getByText('Session Content')).toBeInTheDocument()
  })

  it('should handle nested components', () => {
    render(
      <AuthProvider>
        <div>
          <h1>Parent</h1>
          <p>Child paragraph</p>
        </div>
      </AuthProvider>
    )

    expect(screen.getByRole('heading', { name: 'Parent' })).toBeInTheDocument()
    expect(screen.getByText('Child paragraph')).toBeInTheDocument()
  })

  it('should render multiple children', () => {
    render(
      <AuthProvider>
        <div>First</div>
        <div>Second</div>
        <div>Third</div>
      </AuthProvider>
    )

    expect(screen.getByText('First')).toBeInTheDocument()
    expect(screen.getByText('Second')).toBeInTheDocument()
    expect(screen.getByText('Third')).toBeInTheDocument()
  })
})
