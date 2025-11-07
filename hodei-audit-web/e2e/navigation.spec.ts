// E2E tests for navigation and basic functionality
import { test, expect } from '@playwright/test'

test.describe('Navigation', () => {
  test('should navigate to dashboard', async ({ page }) => {
    await page.goto('/')

    // Should redirect to dashboard or login
    expect(page.url()).toMatch(/\/(dashboard|login|auth)/)
  })

  test('should navigate to events page', async ({ page }) => {
    await page.goto('/events')

    await expect(page.locator('h1')).toContainText(/Events|Event/)
  })

  test('should navigate to analytics page', async ({ page }) => {
    await page.goto('/analytics')

    await expect(page.locator('h1')).toContainText(/Analytics/)
  })

  test('should navigate to compliance page', async ({ page }) => {
    await page.goto('/compliance')

    await expect(page.locator('h1')).toContainText(/Compliance/)
  })

  test('should navigate between pages using navigation menu', async ({ page }) => {
    await page.goto('/')

    // Click on Analytics link
    await page.click('text=Analytics')
    await expect(page).toHaveURL(/analytics/)

    // Click on Events link
    await page.click('text=Events')
    await expect(page).toHaveURL(/events/)

    // Click on Compliance link
    await page.click('text=Compliance')
    await expect(page).toHaveURL(/compliance/)
  })

  test('should show page title correctly', async ({ page }) => {
    await page.goto('/')

    await expect(page).toHaveTitle(/Hodei Audit/)
  })

  test('should handle 404 for non-existent pages', async ({ page }) => {
    await page.goto('/non-existent-page')

    // Should show 404 or redirect
    expect(page.url()).toMatch(/\/(404|login|auth)/)
  })
})

test.describe('Layout and UI', () => {
  test('should display header navigation', async ({ page }) => {
    await page.goto('/')

    // Check if header is visible
    const header = page.locator('header, nav')
    await expect(header).toBeVisible()
  })

  test('should display footer', async ({ page }) => {
    await page.goto('/')

    // Footer should be present
    const footer = page.locator('footer')
    await expect(footer).toBeVisible()
  })

  test('should have responsive navigation on mobile', async ({ page, isMobile }) => {
    await page.setViewportSize({ width: 375, height: 667 })
    await page.goto('/')

    // Mobile menu should be visible
    const mobileMenuButton = page.locator('[data-testid="mobile-menu"], .mobile-menu, [aria-label="Toggle menu"]')
    await expect(mobileMenuButton).toBeVisible()
  })

  test('should display loading states', async ({ page }) => {
    await page.goto('/')

    // Loading indicator should appear and disappear
    const loadingIndicator = page.locator('[data-testid="loading"], .loading, .spinner')
    if (await loadingIndicator.isVisible()) {
      await expect(loadingIndicator).toBeHidden({ timeout: 5000 })
    }
  })
})

test.describe('Accessibility', () => {
  test('should have proper heading hierarchy', async ({ page }) => {
    await page.goto('/')

    // Should have h1 as main heading
    const h1 = page.locator('h1')
    await expect(h1).toBeVisible()

    // Should not skip heading levels
    const headings = page.locator('h1, h2, h3, h4, h5, h6')
    const count = await headings.count()
    expect(count).toBeGreaterThan(0)
  })

  test('should have alt text for images', async ({ page }) => {
    await page.goto('/')

    const images = page.locator('img')
    const count = await images.count()

    for (let i = 0; i < count; i++) {
      const img = images.nth(i)
      const alt = await img.getAttribute('alt')
      expect(alt).toBeTruthy()
    }
  })

  test('should have proper ARIA labels', async ({ page }) => {
    await page.goto('/')

    // Interactive elements should have labels
    const buttons = page.locator('button, [role="button"]')
    const buttonCount = await buttons.count()

    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i)
      const hasLabel = await button.evaluate((el) => {
        return !!(el.getAttribute('aria-label') ||
                 el.getAttribute('aria-labelledby') ||
                 el.getAttribute('title') ||
                 el.textContent?.trim())
      })
      expect(hasLabel).toBe(true)
    }
  })
})

test.describe('Performance', () => {
  test('should load page within reasonable time', async ({ page }) => {
    const startTime = Date.now()
    await page.goto('/')
    await page.waitForLoadState('networkidle')
    const loadTime = Date.now() - startTime

    // Should load within 5 seconds
    expect(loadTime).toBeLessThan(5000)
  })

  test('should have optimized images', async ({ page }) => {
    await page.goto('/')

    const images = page.locator('img')
    const count = await images.count()

    for (let i = 0; i < count; i++) {
      const img = images.nth(i)
      const src = await img.getAttribute('src')
      if (src && !src.startsWith('data:')) {
        // Images should be optimized (have dimensions or use next/image)
        const hasDimensions = await img.evaluate((el) => {
          return !!(el.getAttribute('width') && el.getAttribute('height'))
        })
        // If not using Next.js Image, should at least have loading attribute
        if (!hasDimensions) {
          const loading = await img.getAttribute('loading')
          expect(loading).toBeTruthy()
        }
      }
    }
  })
})
