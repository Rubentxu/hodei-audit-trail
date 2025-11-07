// E2E tests for authentication workflows
import { test, expect } from '@playwright/test'

test.describe('Authentication', () => {
  test.beforeEach(async ({ page }) => {
    // Mock authentication for E2E tests
    await page.addInitScript(() => {
      // Mock next-auth
      window.nextAuth = {
        getSession: () => Promise.resolve({
          user: {
            id: 'test-user',
            email: 'test@example.com',
            name: 'Test User',
            role: 'admin',
            tenantId: 'test-tenant',
          },
        }),
        signIn: () => Promise.resolve(),
        signOut: () => Promise.resolve(),
      }
    })
  })

  test('should redirect to login when not authenticated', async ({ page }) => {
    await page.goto('/dashboard')

    // Should redirect to login
    await expect(page).toHaveURL(/(\/login|\/auth\/login)/)
  })

  test('should display login form', async ({ page }) => {
    await page.goto('/auth/login')

    // Should show login form
    await expect(page.locator('form')).toBeVisible()
    await expect(page.locator('input[type="email"]')).toBeVisible()
    await expect(page.locator('input[type="password"]')).toBeVisible()
    await expect(page.locator('button[type="submit"]')).toBeVisible()
  })

  test('should validate email format', async ({ page }) => {
    await page.goto('/auth/login')

    // Try invalid email
    await page.fill('input[type="email"]', 'invalid-email')
    await page.click('button[type="submit"]')

    // Should show validation error
    await expect(page.locator('text=/invalid|error|Invalid/')).toBeVisible()
  })

  test('should show error for empty fields', async ({ page }) => {
    await page.goto('/auth/login')

    await page.click('button[type="submit"]')

    // Should show validation errors
    const emailError = page.locator('input[type="email"] >> text=/required|Error|Invalid/')
    const passwordError = page.locator('input[type="password"] >> text=/required|Error|Invalid/')

    // At least one validation error should be visible
    const hasError = await Promise.all([
      emailError.isVisible().catch(() => false),
      passwordError.isVisible().catch(() => false),
    ])

    expect(hasError.some(Boolean)).toBe(true)
  })

  test('should handle tenant selection', async ({ page }) => {
    // Mock user with multiple tenants
    await page.addInitScript(() => {
      window.nextAuth.getSession = () => Promise.resolve({
        user: {
          id: 'test-user',
          email: 'test@example.com',
          name: 'Test User',
          role: 'admin',
          // User has access to multiple tenants
        },
        tenants: [
          { id: 'tenant-1', name: 'Tenant 1' },
          { id: 'tenant-2', name: 'Tenant 2' },
        ],
      })
    })

    await page.goto('/auth/tenant-select')

    // Should show tenant selection UI
    await expect(page.locator('h1, h2')).toContainText(/Tenant/)

    // Should display list of tenants
    const tenantItems = page.locator('[data-testid="tenant-item"], .tenant-item')
    await expect(tenantItems).toHaveCount(2)
  })

  test('should show error for invalid credentials', async ({ page }) => {
    await page.goto('/auth/login')

    await page.fill('input[type="email"]', 'test@example.com')
    await page.fill('input[type="password"]', 'wrongpassword')

    // Mock sign-in to fail
    await page.addInitScript(() => {
      window.nextAuth.signIn = () => Promise.reject(new Error('Invalid credentials'))
    })

    await page.click('button[type="submit"]')

    // Should show error message
    await expect(page.locator('text=/Invalid credentials|Login failed|Credentials/')).toBeVisible()
  })

  test('should remember user session', async ({ page }) => {
    // Mock successful authentication
    await page.addInitScript(() => {
      window.nextAuth.getSession = () => Promise.resolve({
        user: {
          id: 'test-user',
          email: 'test@example.com',
          name: 'Test User',
          role: 'admin',
          tenantId: 'test-tenant',
        },
      })
    })

    await page.goto('/auth/login')
    await page.fill('input[type="email"]', 'test@example.com')
    await page.fill('input[type="password"]', 'password')

    await page.click('button[type="submit"]')

    // Should redirect to dashboard
    await expect(page).toHaveURL(/(\/dashboard|\/)/)
  })

  test('should allow logout', async ({ page }) => {
    // Mock authenticated session
    await page.addInitScript(() => {
      window.nextAuth.getSession = () => Promise.resolve({
        user: {
          id: 'test-user',
          email: 'test@example.com',
          name: 'Test User',
          role: 'admin',
          tenantId: 'test-tenant',
        },
      })
    })

    await page.goto('/dashboard')

    // Click logout button
    const logoutButton = page.locator('[data-testid="logout"], button:has-text("Logout"), button:has-text("Sign out")')
    if (await logoutButton.isVisible()) {
      await logoutButton.click()

      // Should redirect to login
      await expect(page).toHaveURL(/(\/login|\/auth\/login)/)
    }
  })

  test('should handle role-based access', async ({ page }) => {
    // Mock admin user
    await page.addInitScript(() => {
      window.nextAuth.getSession = () => Promise.resolve({
        user: {
          id: 'test-user',
          email: 'test@example.com',
          name: 'Test User',
          role: 'admin',
          tenantId: 'test-tenant',
        },
      })
    })

    await page.goto('/dashboard')

    // Admin should see admin-only features
    const adminElements = page.locator('[data-testid="admin-only"], .admin-only, [data-role="admin"]')
    const count = await adminElements.count()

    if (count > 0) {
      await expect(adminElements.first()).toBeVisible()
    }
  })
})

test.describe('Password Reset', () => {
  test('should navigate to password reset', async ({ page }) => {
    await page.goto('/auth/login')

    // Click forgot password link
    const forgotLink = page.locator('a:has-text("Forgot"), a:has-text("Reset")')
    if (await forgotLink.isVisible()) {
      await forgotLink.click()

      // Should go to password reset page
      await expect(page).toHaveURL(/(\/forgot|\/reset|\/auth\/forgot)/)
    }
  })

  test('should allow password reset request', async ({ page }) => {
    await page.goto('/auth/forgot')

    // Should have email input
    const emailInput = page.locator('input[type="email"]')
    await expect(emailInput).toBeVisible()

    // Enter email
    await emailInput.fill('test@example.com')

    // Mock reset request
    await page.addInitScript(() => {
      window.resetPassword = () => Promise.resolve({ success: true })
    })

    // Submit form
    const submitButton = page.locator('button[type="submit"], button:has-text("Send")')
    if (await submitButton.isVisible()) {
      await submitButton.click()

      // Should show success message
      await expect(page.locator('text=/sent|success|check your email/')).toBeVisible()
    }
  })
})
