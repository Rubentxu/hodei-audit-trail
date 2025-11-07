// Test to verify Jest configuration
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

describe('Jest Configuration', () => {
  it('should render without crashing', () => {
    render(<div>Test Component</div>)
    expect(screen.getByText('Test Component')).toBeInTheDocument()
  })

  it('should handle user events', async () => {
    const user = userEvent.setup()
    let counter = 0

    const handleClick = () => {
      counter++
    }

    render(
      <button onClick={handleClick} data-testid="button">
        Click me
      </button>
    )

    await user.click(screen.getByTestId('button'))
    expect(counter).toBe(1)
  })

  it('should match snapshot', () => {
    const { container } = render(<div>Snapshot Test</div>)
    expect(container.firstChild).toMatchSnapshot()
  })
})

describe('Test Utilities', () => {
  it('should have testing library available', () => {
    expect(typeof render).toBe('function')
    expect(typeof screen).toBe('object')
  })

  it('should have jest-dom matchers', () => {
    const element = document.createElement('div')
    document.body.appendChild(element)
    element.textContent = 'Test'

    expect(element).toBeInTheDocument()
    expect(element).toHaveTextContent('Test')
  })
})
