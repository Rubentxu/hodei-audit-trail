// Integration tests for API routes
import { createMocks } from 'node-mocks-http'
import type { NextApiRequest, NextApiResponse } from 'next'
import handler from '@/app/api/events/query/route'

// Mock the database and external services
jest.mock('@/lib/db', () => ({
  db: {
    events: {
      findMany: jest.fn(),
      count: jest.fn(),
    },
  },
}))

jest.mock('@/lib/auth', () => ({
  getServerSession: jest.fn(),
}))

describe('Events API Integration', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('should return events for valid request', async () => {
    const mockEvents = [
      {
        id: 'evt-1',
        eventSource: 'ec2',
        eventName: 'RunInstances',
        timestamp: new Date(),
        userIdentity: { type: 'User', principalId: 'user-123' },
        sourceIPAddress: '192.168.1.1',
      },
      {
        id: 'evt-2',
        eventSource: 's3',
        eventName: 'PutObject',
        timestamp: new Date(),
        userIdentity: { type: 'User', principalId: 'user-456' },
        sourceIPAddress: '192.168.1.2',
      },
    ]

    const { db } = require('@/lib/db')
    db.events.findMany.mockResolvedValue(mockEvents)
    db.events.count.mockResolvedValue(2)

    const { req, res } = createMocks({
      method: 'GET',
      query: {
        tenantId: 'tenant-123',
        page: '1',
        perPage: '10',
      },
    })

    await handler(req as NextApiRequest, res as NextApiResponse)

    expect(res._getStatusCode()).toBe(200)

    const data = JSON.parse(res._getData())
    expect(data.events).toHaveLength(2)
    expect(data.total).toBe(2)
    expect(data.page).toBe(1)
    expect(data.perPage).toBe(10)

    expect(db.events.findMany).toHaveBeenCalled()
  })

  it('should handle filter parameters', async () => {
    const { db } = require('@/lib/db')
    db.events.findMany.mockResolvedValue([])
    db.events.count.mockResolvedValue(0)

    const { req, res } = createMocks({
      method: 'GET',
      query: {
        tenantId: 'tenant-123',
        eventSource: 'ec2',
        eventName: 'RunInstances',
        user: 'user-123',
        startDate: '2025-01-01',
        endDate: '2025-01-31',
      },
    })

    await handler(req as NextApiRequest, res as NextApiResponse)

    expect(res._getStatusCode()).toBe(200)

    const data = JSON.parse(res._getData())
    expect(data.events).toHaveLength(0)

    // Verify that filters were applied
    expect(db.events.findMany).toHaveBeenCalledWith(
      expect.objectContaining({
        where: expect.objectContaining({
          tenantId: 'tenant-123',
        }),
      })
    )
  })

  it('should return 401 for unauthenticated request', async () => {
    const { getServerSession } = require('@/lib/auth')
    getServerSession.mockResolvedValue(null)

    const { req, res } = createMocks({
      method: 'GET',
      query: {
        tenantId: 'tenant-123',
      },
    })

    await handler(req as NextApiRequest, res as NextApiResponse)

    expect(res._getStatusCode()).toBe(401)
  })

  it('should handle invalid query parameters', async () => {
    const { db } = require('@/lib/db')
    db.events.findMany.mockResolvedValue([])
    db.events.count.mockResolvedValue(0)

    const { req, res } = createMocks({
      method: 'GET',
      query: {
        tenantId: 'tenant-123',
        page: 'invalid', // Invalid page number
        perPage: '10',
      },
    })

    await handler(req as NextApiRequest, res as NextApiResponse)

    // Should handle gracefully
    expect(res._getStatusCode()).toBe(200)

    const data = JSON.parse(res._getData())
    expect(data.events).toHaveLength(0)
  })

  it('should handle POST method for creating events', async () => {
    const { db } = require('@/lib/db')
    db.events.create = jest.fn().mockResolvedValue({
      id: 'evt-new',
      eventSource: 'ec2',
      eventName: 'CreateInstance',
    })

    const { req, res } = createMocks({
      method: 'POST',
      body: {
        tenantId: 'tenant-123',
        eventSource: 'ec2',
        eventName: 'CreateInstance',
        eventData: {},
      },
    })

    await handler(req as NextApiRequest, res as NextApiResponse)

    expect(res._getStatusCode()).toBe(201)

    const data = JSON.parse(res._getData())
    expect(data.event).toBeDefined()
    expect(data.event.id).toBe('evt-new')
  })
})

// Mock createMocks for Node.js environment
function createMocks(options: {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE'
  query?: Record<string, string>
  body?: any
}) {
  const req = {
    method: options.method,
    query: options.query || {},
    body: options.body,
    headers: {},
    cookies: {},
  } as any

  const res = {
    _statusCode: 200,
    _data: '',
    status(code: number) {
      this._statusCode = code
      return this
    },
    json(data: any) {
      this._data = JSON.stringify(data)
      return this
    },
    end() {
      return this
    },
  } as any

  return { req, res }
}
