# Testing Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Testing Philosophy](#testing-philosophy)
3. [Testing Stack](#testing-stack)
4. [Test Types](#test-types)
5. [Running Tests](#running-tests)
6. [Writing Tests](#writing-tests)
7. [Test Organization](#test-organization)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)
10. [Resources](#resources)

---

## Introduction

This guide provides comprehensive documentation for testing the Hodei Audit Trail application. Our testing strategy follows a multi-layered approach to ensure code quality, functionality, performance, and security.

## Testing Philosophy

We follow the **Testing Pyramid** strategy:

```
                    E2E Tests (10%)
                   /             \
                  /               \
         Integration Tests (20%)  Performance
               /     \             Tests
              /       \            (5%)
     Unit Tests (65%) Security
                           Tests
```

### Core Principles

1. **Test Early, Test Often**: Write tests as you develop
2. **Fast Feedback**: Unit tests run in milliseconds
3. **Confidence**: E2E tests catch integration issues
4. **Quality Gates**: CI/CD enforces test standards
5. **Maintainable**: Tests are code, maintain them

## Testing Stack

### Frontend (Next.js/React)

| Tool | Purpose | Configuration |
|------|---------|---------------|
| **Jest** | Unit & Integration tests | `jest.config.js` |
| **React Testing Library** | Component testing | `jest.setup.js` |
| **Playwright** | E2E & E2E Performance | `playwright.config.ts` |
| **MSW** | API mocking | In test setup |
| **Vitest** | Alternative test runner | Configured |

### Backend (Rust)

| Tool | Purpose | Configuration |
|------|---------|---------------|
| **cargo test** | Unit & Integration | `Cargo.toml` |
| **cargo tarpaulin** | Code coverage | Configured in CI |
| **criterion** | Benchmarking | `benches/` |
| **mockall** | Mocking | Test utilities |

### Infrastructure

| Tool | Purpose |
|------|---------|
| **GitHub Actions** | CI/CD pipeline |
| **Codecov** | Coverage reports |
| **Trivy** | Security scanning |
| **ESLint/Clippy** | Linting |

## Test Types

### 1. Unit Tests

**Purpose**: Test individual functions, components, and hooks in isolation

**Scope**:
- Pure functions
- Component rendering
- Hook logic
- Utility functions
- Event handlers

**Backend (Rust)**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_validation() {
        let event = create_test_event();
        assert!(event.is_valid());
    }
}
```

**Frontend (React)**:
```typescript
describe('EventDetailsModal', () => {
  it('should render event information', () => {
    render(<EventDetailsModal eventId="1" />)
    expect(screen.getByText('Event Details')).toBeInTheDocument()
  })
})
```

**Time**: < 1s per test
**Location**: 
- Backend: `src/**/*/tests/*.rs`
- Frontend: `src/**/__tests__/*.test.tsx`

### 2. Integration Tests

**Purpose**: Test interaction between components, modules, and services

**Scope**:
- API endpoints
- Component + provider integration
- Database operations
- Authentication flow
- Hook + context integration

**Example**:
```typescript
describe('Events API Integration', () => {
  it('should fetch and display events', async () => {
    const events = await eventsAPI.query({ tenantId: 'test' })
    expect(events).toHaveLength(10)
    expect(events[0]).toHaveProperty('id')
  })
})
```

**Time**: 1-5s per test
**Location**: 
- Backend: `tests/**/*.rs`
- Frontend: `src/__tests__/integration/**/*.test.ts`

### 3. End-to-End (E2E) Tests

**Purpose**: Test complete user workflows in a real browser

**Scenarios**:
- User authentication
- Event creation and filtering
- Analytics dashboard
- Navigation
- Responsive design

**Example**:
```typescript
test('user can create a new event', async ({ page }) => {
  await page.goto('/events')
  await page.click('[data-testid="create-event-button"]')
  await page.fill('[data-testid="event-title"]', 'New Event')
  await page.click('[data-testid="save-button"]')
  await expect(page.locator('[data-testid="success-message"]'))
    .toContainText('Event created successfully')
})
```

**Browsers**: Chromium, Firefox, WebKit
**Time**: 10-30s per test
**Location**: `e2e/**/*.spec.ts`

### 4. Performance Tests

**Purpose**: Ensure application meets performance benchmarks

**Metrics**:
- API response times
- Page load times
- Core Web Vitals (TTI, FCP, CLS)
- Memory usage
- Concurrent request handling

**Benchmarks**:
```typescript
test('events API should respond within 500ms', async ({ page }) => {
  const startTime = Date.now()
  const response = await page.request.get('/api/events/query')
  const endTime = Date.now()
  expect(endTime - startTime).toBeLessThan(500)
})
```

**Time**: 5-15s per test
**Location**: `e2e/performance.spec.ts`

### 5. Security Tests

**Purpose**: Validate authentication, authorization, and security measures

**Areas**:
- Authentication
- Authorization (RBAC)
- Input validation
- XSS prevention
- CSRF protection
- Session management
- Rate limiting

**Example**:
```typescript
test('unauthenticated users cannot access protected routes', async ({ page }) => {
  await page.goto('/dashboard')
  expect(page.url()).toMatch(/\/login/)
})
```

**Time**: 2-5s per test
**Location**: `e2e/security.spec.ts`

### 6. API Tests

**Purpose**: Test API contracts and behavior

**Scope**:
- Request/response validation
- Error handling
- Authentication
- Rate limiting
- CORS
- Data serialization

**Tools**:
- Playwright request API
- node-mocks-http (unit tests)

## Running Tests

### Quick Reference

```bash
# Frontend
cd hodei-audit-web

# Run all tests
npm test

# Run in watch mode
npm run test:watch

# Run specific test file
npm test EventDetailsModal

# Run E2E tests
npx playwright test

# Run specific E2E file
npx playwright test auth.spec.ts

# Run performance tests
npx playwright test e2e/performance.spec.ts

# Run security tests
npx playwright test e2e/security.spec.ts

# Generate coverage
npm run test:coverage

# Generate coverage report
npm run test:coverage:html

# Run in CI mode
npm run test:ci

# Backend
cd hodei-audit-service

# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Run specific test
cargo test event_handler

# Run benchmarks
cargo bench
```

### Test Commands by Type

#### Unit Tests

```bash
# Frontend
npm test -- --testNamePattern="Unit Test"
# or
npm test src/components/EventCard.test.tsx

# Backend
cargo test --lib
```

#### Integration Tests

```bash
# Frontend
npm test -- --testPathPattern="integration"
# or
npm test src/__tests__/integration/events-api.test.ts

# Backend
cargo test --test '*'
```

#### E2E Tests

```bash
# All browsers
npx playwright test

# Specific browser
npx playwright test --project=chromium

# Debug mode
npx playwright test --debug

# Headed mode
npx playwright test --headed

# Retries
npx playwright test --retries=2
```

#### Performance Tests

```bash
# Run performance suite
npx playwright test e2e/performance.spec.ts

# Run with trace
npx playwright test e2e/performance.spec.ts --trace on

# Run specific test
npx playwright test e2e/performance.spec.ts -g "dashboard"
```

#### Security Tests

```bash
# Run security suite
npx playwright test e2e/security.spec.ts

# Run specific security test
npx playwright test e2e/security.spec.ts -g "authentication"
```

## Writing Tests

### Test File Naming

#### Frontend

| File Type | Pattern | Example |
|-----------|---------|---------|
| Component | `*.test.tsx` | `EventCard.test.tsx` |
| Hook | `use-*.test.ts` | `use-auth.test.ts` |
| Utils | `*-util.test.ts` | `date-util.test.ts` |
| Integration | `*.test.ts` in `__tests__/integration/` | `events-api.test.ts` |

#### Backend

| File Type | Pattern | Example |
|-----------|---------|---------|
| Unit | Inline `#[cfg(test)]` | In `lib.rs` |
| Integration | `tests/*.rs` | `tests/api_tests.rs` |

#### E2E

| File Type | Pattern | Example |
|-----------|---------|---------|
| Feature | `*.spec.ts` in `e2e/` | `events.spec.ts` |

### Test Structure

#### Unit Test Template (Jest)

```typescript
import { render, screen } from '@testing-library/react'
import { describe, it, expect, vi } from 'vitest'
import { ComponentName } from './ComponentName'

// Mock dependencies
vi.mock('@/lib/some-lib', () => ({
  someFunction: vi.fn(),
}))

describe('ComponentName', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should render correctly', () => {
    render(<ComponentName />)
    expect(screen.getByText('Expected Text')).toBeInTheDocument()
  })

  it('should handle user interaction', async () => {
    render(<ComponentName />)
    const button = screen.getByRole('button')
    fireEvent.click(button)
    expect(mockFunction).toHaveBeenCalledTimes(1)
  })

  it('should display error state', () => {
    const error = new Error('Test error')
    render(<ComponentName error={error} />)
    expect(screen.getByText(/error/i)).toBeInTheDocument()
  })

  it('should match snapshot', () => {
    const { container } = render(<ComponentName />)
    expect(container.firstChild).toMatchSnapshot()
  })
})
```

#### E2E Test Template (Playwright)

```typescript
import { test, expect } from '@playwright/test'

test.describe('Feature Name', () => {
  test.beforeEach(async ({ page }) => {
    // Login or setup
    await page.goto('/login')
    await page.fill('[data-testid="email"]', 'test@example.com')
    await page.fill('[data-testid="password"]', 'password123')
    await page.click('[data-testid="login-button"]')
  })

  test('should complete user workflow', async ({ page }) => {
    // Navigate to feature
    await page.goto('/feature')

    // Interact
    await page.click('[data-testid="action-button"]')
    await page.fill('[data-testid="input-field"]', 'value')
    await page.click('[data-testid="submit-button"]')

    // Assert
    await expect(page.locator('[data-testid="success-message"]'))
      .toContainText('Success')
  })

  test('should handle error state', async ({ page }) => {
    await page.goto('/feature')
    await page.click('[data-testid="error-trigger"]')

    await expect(page.locator('[data-testid="error-message"]'))
      .toContainText('Error')
  })

  test('should be accessible', async ({ page }) => {
    await page.goto('/feature')

    // Check for accessibility issues
    const accessibilityScanResults = await new AxePuppeteer(page).analyze()
    expect(accessibilityScanResults.violations).toEqual([])
  })
})
```

### Test Data Factories

#### Frontend

```typescript
// test-utils.tsx
export const createMockEvent = (overrides: Partial<Event> = {}): Event => ({
  id: '1',
  type: 'USER_ACTION',
  description: 'Test Event',
  timestamp: new Date().toISOString(),
  tenantId: 'test-tenant',
  ...overrides,
})

export const createMockUser = (overrides: Partial<User> = {}): User => ({
  id: '1',
  email: 'test@example.com',
  name: 'Test User',
  role: 'user',
  ...overrides,
})
```

#### Backend

```rust
pub fn create_test_event() -> Event {
    Event {
        id: Uuid::new_v4(),
        event_type: EventType::UserAction,
        description: "Test Event".to_string(),
        created_at: Utc::now(),
        tenant_id: "test-tenant".to_string(),
    }
}
```

### Mocking

#### React Testing Library

```typescript
// Mock next/router
vi.mock('next/router', () => ({
  useRouter: vi.fn(() => ({
    route: '/events',
    pathname: '/events',
    query: { id: '1' },
  })),
}))

// Mock next-auth
vi.mock('next-auth/react', () => ({
  useSession: vi.fn(() => ({
    data: {
      user: { id: '1', email: 'test@example.com' },
    },
    status: 'authenticated',
  })),
}))
```

#### Playwright

```typescript
// Mock API response
test('should display fetched data', async ({ page }) => {
  await page.route('/api/events', route => {
    route.fulfill({
      json: [{ id: '1', name: 'Test Event' }],
    })
  })

  await page.goto('/events')
  await expect(page.locator('[data-testid="event-item"]'))
    .toHaveCount(1)
})
```

## Test Organization

### Directory Structure

```
hodei-audit-web/
├── src/
│   ├── components/
│   │   ├── events/
│   │   │   ├── EventCard.tsx
│   │   │   └── __tests__/
│   │   │       └── EventCard.test.tsx
│   │   └── providers/
│   │       ├── AuthProvider.tsx
│   │       └── __tests__/
│   │           └── AuthProvider.test.tsx
│   ├── hooks/
│   │   ├── use-auth.ts
│   │   └── __tests__/
│   │       └── use-auth.test.ts
│   ├── lib/
│   │   ├── api/
│   │   │   └── __tests__/
│   │   │       └── events.test.ts
│   │   └── utils/
│   │       └── __tests__/
│   │           └── date-utils.test.ts
│   └── __tests__/
│       └── integration/
│           ├── events-api.test.ts
│           └── component-integration.test.ts
├── e2e/
│   ├── auth.spec.ts
│   ├── events.spec.ts
│   ├── analytics.spec.ts
│   ├── navigation.spec.ts
│   ├── performance.spec.ts
│   └── security.spec.ts
├── jest.config.js
├── jest.setup.js
└── playwright.config.ts
```

### Test Tags

Use tags to categorize tests:

```typescript
// Frontend
test.describe('Events API @unit', () => { ... })
test.describe('Authentication @integration', () => { ... })

// Playwright
test.describe('Performance Tests @performance', () => {
  test('... @slow', async ({ page }) => { ... })
})
```

Run with tags:
```bash
npx playwright test --grep '@unit'
npx playwright test --grep '@performance'
```

## Best Practices

### ✅ DO

1. **Write Tests First**: Follow TDD when possible
   ```typescript
   // 1. Write failing test
   test('should return filtered events', () => {
     const events = filterEvents(allEvents, 'error')
     expect(events.every(e => e.type === 'ERROR')).toBe(true)
   })
   
   // 2. Implement
   const filterEvents = (events, type) => events.filter(e => e.type === type)
   ```

2. **Use Descriptive Test Names**
   ```typescript
   ❌ test('event test', () => { ... })
   ✅ test('should display error message when API fails', () => { ... })
   ```

3. **Test Behavior, Not Implementation**
   ```typescript
   ❌ expect(component.state().loading).toBe(true)
   ✅ expect(screen.getByText(/loading/i)).toBeInTheDocument()
   ```

4. **Arrange-Act-Assert Pattern**
   ```typescript
   test('should update user profile', async () => {
     // Arrange
     const user = createMockUser()
     
     // Act
     await updateUserProfile(user, { name: 'New Name' })
     
     // Assert
     expect(screen.getByText('New Name')).toBeInTheDocument()
   })
   ```

5. **Keep Tests Isolated**
   ```typescript
   beforeEach(() => {
     // Reset state
     vi.clearAllMocks()
   })
   ```

6. **Use Data Test IDs**
   ```typescript
   // In component
   <button data-testid="save-button">Save</button>
   
   // In test
   await page.click('[data-testid="save-button"]')
   ```

7. **Mock External Dependencies**
   ```typescript
   vi.mock('@/lib/api', () => ({
     fetchEvents: vi.fn(),
   }))
   ```

8. **Test Edge Cases**
   ```typescript
   test('should handle empty events list', () => {
     const result = processEvents([])
     expect(result).toEqual([])
   })
   
   test('should handle null event', () => {
     const result = processEvent(null)
     expect(result).toBeNull()
   })
   ```

9. **Use Test Utilities**
   ```typescript
   // Create common test helpers
   const renderWithProviders = (component) => {
     return render(
       <QueryClientProvider client={queryClient}>
         <AuthProvider>{component}</AuthProvider>
       </QueryClientProvider>
     )
   }
   ```

10. **Clean Up After Tests**
    ```typescript
    afterEach(() => {
      cleanup()
      vi.clearAllMocks()
    })
    ```

### ❌ DON'T

1. **Don't Test Third-Party Libraries**
   ```typescript
   ❌ test('lodash should work', () => {
     expect(_.map([1, 2], n => n * 2)).toEqual([2, 4])
   })
   ```

2. **Don't Write Brittle Tests**
   ```typescript
   ❌ test('component', () => {
     const div = document.querySelector('div > div:nth-child(2)')
     expect(div).toBeTruthy()
   })
   ```

3. **Don't Test Implementation Details**
   ```typescript
   ❌ test('should call useState', () => {
     expect(MyComponent.prototype.setState).toHaveBeenCalled()
   })
   ```

4. **Don't Share State Between Tests**
   ```typescript
   ❌ let counter = 0
   test('increments', () => { counter++ })
   test('decrements', () => { counter-- })
   ```

5. **Don't Use Real APIs in Tests**
   ```typescript
   ❌ test('should fetch user', async () => {
     const user = await fetch('https://api.example.com/user/1')
     expect(user).toBeDefined()
   })
   ```

6. **Don't Ignore Failing Tests**
   - Fix failing tests immediately
   - Use `.skip()` temporarily with `@ticket` comment

7. **Don't Write Flaky Tests**
   ```typescript
   ❌ test('should load', async () => {
     await page.waitForTimeout(1000) // Time-based
     expect(await page.url()).toBe('/dashboard')
   })
   
   ✅ test('should load', async () => {
     await page.waitForSelector('[data-testid="dashboard"]')
     expect(await page.url()).toBe('/dashboard')
   })
   ```

## Troubleshooting

### Common Issues

#### 1. Test Timeouts

**Problem**: Tests timeout randomly

**Solutions**:
- Increase timeout for slow tests
- Mock slow operations
- Use `waitForSelector` instead of `waitForTimeout`

```typescript
test('should load data', async ({ page }) => {
  page.setDefaultTimeout(30000) // 30s
  await page.goto('/events')
  await page.waitForSelector('[data-testid="events-list"]')
})
```

#### 2. Flaky E2E Tests

**Problem**: Tests pass locally but fail in CI

**Solutions**:
- Increase wait times
- Use networkidle
- Check for race conditions
- Use retries

```typescript
test('should load data', async ({ page }) => {
  await page.goto('/events')
  await page.waitForLoadState('networkidle')
  // OR
  await expect(page.locator('[data-testid="events-list"]'))
    .toBeVisible({ timeout: 10000 })
})
```

#### 3. Mock Not Working

**Problem**: Mock returns undefined

**Solution**:
- Clear mocks in `beforeEach`
- Check mock implementation

```typescript
beforeEach(() => {
  vi.clearAllMocks()
})

test('should call API', async () => {
  const mockApi = vi.fn().mockResolvedValue({ data: [] })
  await subject()
  expect(mockApi).toHaveBeenCalled()
})
```

#### 4. Coverage Not Increasing

**Problem**: Added tests but coverage unchanged

**Solutions**:
- Check test file pattern in config
- Verify test runs (check test output)
- Add tests for uncovered branches

```bash
# Check which files are covered
npm run test:coverage -- --coverageReporters=html
# Open coverage/index.html
```

#### 5. Type Errors in Tests

**Problem**: TypeScript errors in test files

**Solutions**:
- Add `// @ts-ignore` temporarily
- Fix type definitions
- Update `tsconfig.json`

### Debugging Tests

#### Frontend Unit Tests

```bash
# Run in watch mode
npm test -- --watch

# Run specific test
npm test -- --testNamePattern="EventCard"

# Debug with console
console.log('Debug info:', value)
```

#### E2E Tests

```bash
# Run in debug mode
npx playwright test --debug

# Run headed
npx playwright test --headed

# Show browser
npx playwright test --show-browser

# Trace mode
npx playwright test --trace on
```

#### Backend Tests

```bash
# Run with output
cargo test -- --nocapture

# Run specific test
cargo test event_handler

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

## Resources

### Documentation

- [Jest Docs](https://jestjs.io/)
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/)
- [Playwright Docs](https://playwright.dev/)
- [Vitest Guide](https://vitest.dev/)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)

### Articles & Guides

- [Testing React Components](https://kentcdodds.com/blog/common-mistakes-with-react-testing-library)
- [Playwright Best Practices](https://playwright.dev/docs/best-practices)
- [Effective Unit Testing](https://github.com/goldbergyoni/javascript-testing-best-practices)

### Tools

- [Testing Library Queries](https://testing-library.com/docs/queries/bytestid)
- [Playwright Test Generator](https://playwright.dev/docs/codegen)
- [MSW (Mock Service Worker)](https://mswjs.io/)
- [Jest Coverage](https://jestjs.io/docs/coverage)

### IDE Extensions

- **VS Code**: Jest, Playwright Test Runner
- **IntelliJ**: Jest, Playwright plugins

### CI/CD Integration

- [GitHub Actions for Testing](.github/workflows/ci.yml)
- [Codecov Integration](./COVERAGE.md)
- [CI/CD Pipeline Documentation](../CI-CD-PIPELINE.md)

## Getting Help

### Support Channels

1. Check this guide first
2. Review test configuration files
3. Ask in team chat
4. Create issue with test details

### Reporting Test Issues

Include:
- Test file name
- Error message
- Steps to reproduce
- Environment (local/CI)
- Browser/version (for E2E)

Example:
```
File: e2e/events.spec.ts
Test: "should filter events by type"
Error: TimeoutError: Waiting for selector "[data-testid='event-item']" failed: Timeout 30000ms exceeded
Environment: CI, Chrome
```

## Contributing

### Adding New Tests

1. Follow naming conventions
2. Use appropriate test type (unit/integration/e2e)
3. Add to correct directory
4. Ensure coverage doesn't drop below 70%

### Updating Test Configuration

1. Jest config: `jest.config.js`
2. Playwright config: `playwright.config.ts`
3. Update documentation if needed

---

## Summary

This guide covers all aspects of testing for the Hodei Audit Trail project. Remember:

1. **Write tests as you code**
2. **Follow the testing pyramid**
3. **Keep tests fast, reliable, and maintainable**
4. **Aim for 70%+ coverage**
5. **Review and refactor tests regularly**

For questions or issues, refer to the troubleshooting section or create an issue.
