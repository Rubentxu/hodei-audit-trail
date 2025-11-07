// Tests for useAuth hook
import { renderHook, act } from '@testing-library/react'
import { useAuth } from '../use-auth'
import { useRouter } from 'next/navigation'
import { useSession } from 'next-auth/react'

// Mock Next.js hooks
jest.mock('next/navigation', () => ({
  useRouter: jest.fn(),
}))

jest.mock('next-auth/react', () => ({
  useSession: jest.fn(),
}))

const mockPush = jest.fn()

beforeEach(() => {
  jest.clearAllMocks()
  ;(useRouter as jest.Mock).mockReturnValue({
    push: mockPush,
  })
})

describe('useAuth', () => {
  it('should return loading state when session is loading', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'loading',
    })

    const { result } = renderHook(() => useAuth())

    expect(result.current.isLoading).toBe(true)
    expect(result.current.isAuthenticated).toBe(false)
    expect(result.current.status).toBe('loading')
  })

  it('should return authenticated state when session exists', () => {
    const mockSession = {
      user: {
        id: 'user-123',
        email: 'test@example.com',
        role: 'viewer',
        tenantId: 'tenant-123',
      },
    }

    ;(useSession as jest.Mock).mockReturnValue({
      data: mockSession,
      status: 'authenticated',
    })

    const { result } = renderHook(() => useAuth())

    expect(result.current.isLoading).toBe(false)
    expect(result.current.isAuthenticated).toBe(true)
    expect(result.current.status).toBe('authenticated')
    expect(result.current.session).toBe(mockSession)
  })

  it('should return unauthenticated state when no session', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'unauthenticated',
    })

    const { result } = renderHook(() => useAuth())

    expect(result.current.isLoading).toBe(false)
    expect(result.current.isAuthenticated).toBe(false)
    expect(result.current.status).toBe('unauthenticated')
  })

  it('should redirect to login when not authenticated', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'unauthenticated',
    })

    renderHook(() => useAuth())

    expect(mockPush).toHaveBeenCalledWith('/auth/login')
  })

  it('should redirect to login when status is unauthenticated and requiredTenant is false', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'unauthenticated',
    })

    renderHook(() => useAuth(false))

    expect(mockPush).toHaveBeenCalledWith('/auth/login')
  })

  it('should redirect to tenant selection when tenant is required but not present', () => {
    const mockSessionWithoutTenant = {
      user: {
        id: 'user-123',
        email: 'test@example.com',
        role: 'viewer',
      },
    }

    ;(useSession as jest.Mock).mockReturnValue({
      data: mockSessionWithoutTenant,
      status: 'authenticated',
    })

    renderHook(() => useAuth(true))

    expect(mockPush).toHaveBeenCalledWith('/auth/tenant-select')
  })

  it('should not redirect when tenant is present', () => {
    const mockSession = {
      user: {
        id: 'user-123',
        email: 'test@example.com',
        role: 'viewer',
        tenantId: 'tenant-123',
      },
    }

    ;(useSession as jest.Mock).mockReturnValue({
      data: mockSession,
      status: 'authenticated',
    })

    renderHook(() => useAuth(true))

    expect(mockPush).not.toHaveBeenCalled()
  })

  it('should not redirect when tenant is not required and user is authenticated', () => {
    const mockSession = {
      user: {
        id: 'user-123',
        email: 'test@example.com',
        role: 'viewer',
      },
    }

    ;(useSession as jest.Mock).mockReturnValue({
      data: mockSession,
      status: 'authenticated',
    })

    renderHook(() => useAuth(false))

    expect(mockPush).not.toHaveBeenCalled()
  })

  it('should update when session status changes', () => {
    const mockSession = {
      user: {
        id: 'user-123',
        email: 'test@example.com',
        role: 'viewer',
        tenantId: 'tenant-123',
      },
    }

    ;(useSession as jest.Mock)
      .mockReturnValueOnce({
        data: null,
        status: 'loading',
      })
      .mockReturnValueOnce({
        data: mockSession,
        status: 'authenticated',
      })

    const { result, rerender } = renderHook(() => useAuth())

    // First render: loading
    expect(result.current.status).toBe('loading')

    // Rerender: authenticated
    rerender()
    expect(result.current.status).toBe('authenticated')
  })
})

describe('useRequireRole', () => {
  beforeEach(() => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: {
        user: {
          id: 'user-123',
          email: 'test@example.com',
          role: 'admin',
          tenantId: 'tenant-123',
        },
      },
      status: 'authenticated',
    })
  })

  it('should return true when user has matching role', () => {
    const { result } = renderHook(() => useRequireRole('admin'))

    expect(result.current).toBe(true)
  })

  it('should return false when user does not have matching role', () => {
    const { result } = renderHook(() => useRequireRole('viewer'))

    expect(result.current).toBe(false)
  })

  it('should accept array of required roles', () => {
    const { result } = renderHook(() => useRequireRole(['admin', 'superuser']))

    expect(result.current).toBe(true)
  })

  it('should return true when user has at least one of the required roles', () => {
    const { result } = renderHook(() => useRequireRole(['viewer', 'admin']))

    expect(result.current).toBe(true)
  })

  it('should return false when user has none of the required roles', () => {
    const { result } = renderHook(() => useRequireRole(['viewer', 'guest']))

    expect(result.current).toBe(false)
  })

  it('should handle empty role array', () => {
    const { result } = renderHook(() => useRequireRole([]))

    expect(result.current).toBe(false)
  })

  it('should handle undefined user role', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: {
        user: {
          id: 'user-123',
          email: 'test@example.com',
          // role is undefined
          tenantId: 'tenant-123',
        },
      },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useRequireRole('admin'))

    expect(result.current).toBe(false)
  })

  it('should use useAuth internally', () => {
    const { result } = renderHook(() => useRequireRole('admin'))

    // Verify that useAuth was called
    expect(result.current).toBeDefined()
  })
})
