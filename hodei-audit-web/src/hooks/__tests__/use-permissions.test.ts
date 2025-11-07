// Tests for usePermission and related hooks
import { renderHook } from '@testing-library/react'
import { usePermission, useAnyPermission, useAllPermissions, useUserRole, useIsAdmin, useIsAuditorOrAbove, useIsAnalystOrAbove } from '../use-permissions'
import { useSession } from 'next-auth/react'
import { UserRole } from '@/types/auth'

// Mock dependencies
jest.mock('next-auth/react', () => ({
  useSession: jest.fn(),
}))

jest.mock('@/lib/auth/permissions', () => ({
  hasPermission: jest.fn(),
  hasAnyPermission: jest.fn(),
  hasAllPermissions: jest.fn(),
}))

const mockHasPermission = jest.mocked(require('@/lib/auth/permissions').hasPermission)
const mockHasAnyPermission = jest.mocked(require('@/lib/auth/permissions').hasAnyPermission)
const mockHasAllPermissions = jest.mocked(require('@/lib/auth/permissions').hasAllPermissions)

const mockSessionData = {
  user: {
    id: 'user-123',
    email: 'test@example.com',
    role: 'admin' as UserRole,
    tenantId: 'tenant-123',
  },
}

beforeEach(() => {
  jest.clearAllMocks()
  ;(useSession as jest.Mock).mockReturnValue({
    data: mockSessionData,
    status: 'authenticated',
  })
})

describe('usePermission', () => {
  it('should return true when user has the permission', () => {
    mockHasPermission.mockReturnValue(true)

    const { result } = renderHook(() => usePermission('manage:users' as any))

    expect(result.current).toBe(true)
    expect(mockHasPermission).toHaveBeenCalledWith('admin', 'manage:users')
  })

  it('should return false when user does not have the permission', () => {
    mockHasPermission.mockReturnValue(false)

    const { result } = renderHook(() => usePermission('manage:users' as any))

    expect(result.current).toBe(false)
    expect(mockHasPermission).toHaveBeenCalledWith('admin', 'manage:users')
  })

  it('should return false when user role is undefined', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => usePermission('manage:users' as any))

    expect(result.current).toBe(false)
    expect(mockHasPermission).not.toHaveBeenCalled()
  })

  it('should return false when session is null', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'authenticated',
    })

    const { result } = renderHook(() => usePermission('manage:users' as any))

    expect(result.current).toBe(false)
    expect(mockHasPermission).not.toHaveBeenCalled()
  })
})

describe('useAnyPermission', () => {
  it('should return true when user has at least one permission', () => {
    mockHasAnyPermission.mockReturnValue(true)

    const { result } = renderHook(() => useAnyPermission(['perm1', 'perm2', 'perm3'] as any))

    expect(result.current).toBe(true)
    expect(mockHasAnyPermission).toHaveBeenCalledWith('admin', ['perm1', 'perm2', 'perm3'])
  })

  it('should return false when user has none of the permissions', () => {
    mockHasAnyPermission.mockReturnValue(false)

    const { result } = renderHook(() => useAnyPermission(['perm1', 'perm2', 'perm3'] as any))

    expect(result.current).toBe(false)
    expect(mockHasAnyPermission).toHaveBeenCalledWith('admin', ['perm1', 'perm2', 'perm3'])
  })

  it('should return false when user role is undefined', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useAnyPermission(['perm1', 'perm2'] as any))

    expect(result.current).toBe(false)
    expect(mockHasAnyPermission).not.toHaveBeenCalled()
  })
})

describe('useAllPermissions', () => {
  it('should return true when user has all permissions', () => {
    mockHasAllPermissions.mockReturnValue(true)

    const { result } = renderHook(() => useAllPermissions(['perm1', 'perm2', 'perm3'] as any))

    expect(result.current).toBe(true)
    expect(mockHasAllPermissions).toHaveBeenCalledWith('admin', ['perm1', 'perm2', 'perm3'])
  })

  it('should return false when user is missing at least one permission', () => {
    mockHasAllPermissions.mockReturnValue(false)

    const { result } = renderHook(() => useAllPermissions(['perm1', 'perm2', 'perm3'] as any))

    expect(result.current).toBe(false)
    expect(mockHasAllPermissions).toHaveBeenCalledWith('admin', ['perm1', 'perm2', 'perm3'])
  })

  it('should return false when user role is undefined', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useAllPermissions(['perm1', 'perm2'] as any))

    expect(result.current).toBe(false)
    expect(mockHasAllPermissions).not.toHaveBeenCalled()
  })
})

describe('useUserRole', () => {
  it('should return the user role', () => {
    const { result } = renderHook(() => useUserRole())

    expect(result.current).toBe('admin')
  })

  it('should return null when role is undefined', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useUserRole())

    expect(result.current).toBeNull()
  })

  it('should return null when session is null', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: null,
      status: 'authenticated',
    })

    const { result } = renderHook(() => useUserRole())

    expect(result.current).toBeNull()
  })
})

describe('useIsAdmin', () => {
  it('should return true when user is admin', () => {
    mockHasPermission.mockReturnValue(true)

    const { result } = renderHook(() => useIsAdmin())

    expect(result.current).toBe(true)
    expect(mockHasPermission).toHaveBeenCalledWith('admin', 'manage:tenants')
  })

  it('should return false when user is not admin', () => {
    mockHasPermission.mockReturnValue(false)

    const { result } = renderHook(() => useIsAdmin())

    expect(result.current).toBe(false)
    expect(mockHasPermission).toHaveBeenCalledWith('admin', 'manage:tenants')
  })
})

describe('useIsAuditorOrAbove', () => {
  it('should return true when user is admin', () => {
    const { result } = renderHook(() => useIsAuditorOrAbove())

    expect(result.current).toBe(true)
  })

  it('should return true when user is auditor', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'auditor' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAuditorOrAbove())

    expect(result.current).toBe(true)
  })

  it('should return false when user is analyst', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'analyst' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAuditorOrAbove())

    expect(result.current).toBe(false)
  })

  it('should return false when user is viewer', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'viewer' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAuditorOrAbove())

    expect(result.current).toBe(false)
  })

  it('should return false when user role is null', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAuditorOrAbove())

    expect(result.current).toBe(false)
  })
})

describe('useIsAnalystOrAbove', () => {
  it('should return true when user is admin', () => {
    const { result } = renderHook(() => useIsAnalystOrAbove())

    expect(result.current).toBe(true)
  })

  it('should return true when user is auditor', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'auditor' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAnalystOrAbove())

    expect(result.current).toBe(true)
  })

  it('should return true when user is analyst', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'analyst' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAnalystOrAbove())

    expect(result.current).toBe(true)
  })

  it('should return false when user is viewer', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123', role: 'viewer' as UserRole } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAnalystOrAbove())

    expect(result.current).toBe(false)
  })

  it('should return false when user role is null', () => {
    ;(useSession as jest.Mock).mockReturnValue({
      data: { user: { id: 'user-123' } },
      status: 'authenticated',
    })

    const { result } = renderHook(() => useIsAnalystOrAbove())

    expect(result.current).toBe(false)
  })
})
