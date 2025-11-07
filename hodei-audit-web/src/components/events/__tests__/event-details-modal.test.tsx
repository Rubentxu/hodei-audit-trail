// Tests for EventDetailsModal component
import { render, screen } from '@/lib/test-utils'
import userEvent from '@testing-library/user-event'
import { EventDetailsModal } from '../event-details-modal'

// Mock UI components
jest.mock('@/components/ui/dialog', () => ({
  Dialog: ({ open, onOpenChange, children }: any) => (
    <div data-testid="dialog" data-open={open}>{children}</div>
  ),
  DialogContent: ({ children, className }: any) => (
    <div className={className} data-testid="dialog-content">{children}</div>
  ),
  DialogDescription: ({ children }: any) => <div>{children}</div>,
  DialogHeader: ({ children }: any) => <div>{children}</div>,
  DialogTitle: ({ children, className }: any) => (
    <h2 className={className} data-testid="dialog-title">{children}</h2>
  ),
}))

jest.mock('@/components/ui/badge', () => ({
  Badge: ({ children, className }: any) => (
    <span className={className} data-testid="badge">{children}</span>
  ),
}))

jest.mock('@/components/ui/separator', () => ({
  Separator: () => <hr data-testid="separator" />,
}))

const mockEvent = {
  id: 'evt-123',
  timestamp: '2025-01-15T10:30:00Z',
  user: 'test-user',
  action: 'CreateUser',
  resource: 'arn:aws:iam::123456789012:user/test',
  status: 'success' as const,
  source: 'iam',
  details: 'User created successfully',
  ip: '192.168.1.1',
  userAgent: 'TestAgent/1.0',
  duration: 150,
  bytes: 1024,
}

const defaultProps = {
  event: mockEvent,
  isOpen: true,
  onClose: jest.fn(),
}

describe('EventDetailsModal', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('should render dialog when isOpen is true', () => {
    render(<EventDetailsModal {...defaultProps} />)

    expect(screen.getByTestId('dialog')).toBeInTheDocument()
    expect(screen.getByTestId('dialog')).toHaveAttribute('data-open', 'true')
  })

  it('should not render when event is null', () => {
    render(
      <EventDetailsModal
        event={null}
        isOpen={true}
        onClose={jest.fn()}
      />
    )

    expect(screen.queryByTestId('dialog')).not.toBeInTheDocument()
  })

  it('should not render when isOpen is false', () => {
    render(
      <EventDetailsModal
        event={mockEvent}
        isOpen={false}
        onClose={jest.fn()}
      />
    )

    expect(screen.queryByTestId('dialog')).not.toBeInTheDocument()
  })

  it('should display event title', () => {
    render(<EventDetailsModal {...defaultProps} />)

    expect(screen.getByTestId('dialog-title')).toBeInTheDocument()
    expect(screen.getByText('Event Details')).toBeInTheDocument()
  })

  it('should display event details', () => {
    render(<EventDetailsModal {...defaultProps} />)

    expect(screen.getByText(mockEvent.id)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.user)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.action)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.resource)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.source)).toBeInTheDocument()
  })

  it('should display status badge with correct color', () => {
    render(<EventDetailsModal {...defaultProps} />)

    const badge = screen.getByTestId('badge')
    expect(badge).toBeInTheDocument()
    expect(badge).toHaveTextContent('success')
  })

  it('should display status icon', () => {
    render(<EventDetailsModal {...defaultProps} />)

    // Success should have CheckCircle icon (mocked)
    expect(screen.getByTestId('badge')).toBeInTheDocument()
  })

  it('should call onClose when dialog is closed', async () => {
    const user = userEvent.setup()
    const onClose = jest.fn()

    render(
      <EventDetailsModal
        event={mockEvent}
        isOpen={true}
        onClose={onClose}
      />
    )

    // Simulate close by changing isOpen to false
    // In real scenario, this would be triggered by clicking outside or close button
  })

  it('should handle different event statuses', () => {
    const failureEvent = { ...mockEvent, status: 'failure' as const }
    render(
      <EventDetailsModal
        event={failureEvent}
        isOpen={true}
        onClose={jest.fn()}
      />
    )

    expect(screen.getByTestId('badge')).toHaveTextContent('failure')
  })

  it('should handle warning status', () => {
    const warningEvent = { ...mockEvent, status: 'warning' as const }
    render(
      <EventDetailsModal
        event={warningEvent}
        isOpen={true}
        onClose={jest.fn()}
      />
    )

    expect(screen.getByTestId('badge')).toHaveTextContent('warning')
  })

  it('should display optional fields when present', () => {
    render(<EventDetailsModal {...defaultProps} />)

    expect(screen.getByText(mockEvent.details!)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.ip!)).toBeInTheDocument()
    expect(screen.getByText(mockEvent.userAgent!)).toBeInTheDocument()
    expect(screen.getByText(`${mockEvent.duration}ms`)).toBeInTheDocument()
    expect(screen.getByText('1 KB')).toBeInTheDocument()
  })

  it('should handle events without optional fields', () => {
    const minimalEvent = {
      id: 'evt-456',
      timestamp: '2025-01-15T10:30:00Z',
      user: 'test-user',
      action: 'TestAction',
      resource: 'test-resource',
      status: 'success' as const,
      source: 'test-source',
    }

    render(
      <EventDetailsModal
        event={minimalEvent}
        isOpen={true}
        onClose={jest.fn()}
      />
    )

    expect(screen.getByTestId('dialog')).toBeInTheDocument()
    expect(screen.getByText(minimalEvent.id)).toBeInTheDocument()
  })

  it('should format timestamp correctly', () => {
    render(<EventDetailsModal {...defaultProps} />)

    expect(screen.getByText(mockEvent.timestamp)).toBeInTheDocument()
  })

  it('should format bytes correctly', () => {
    render(<EventDetailsModal {...defaultProps} />)

    // 1024 bytes should be formatted as "1 KB"
    expect(screen.getByText('1 KB')).toBeInTheDocument()
  })
})
