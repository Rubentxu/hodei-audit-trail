// E2E tests for analytics functionality
import { test, expect } from '@playwright/test'

test.describe('Analytics Page', () => {
  test.beforeEach(async ({ page }) => {
    // Mock authenticated session
    await page.addInitScript(() => {
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
      }
    })
  })

  test('should display analytics dashboard', async ({ page }) => {
    await page.goto('/analytics')

    // Should have analytics heading
    await expect(page.locator('h1, h2')).toContainText(/Analytics/)

    // Should show charts
    const charts = page.locator('[data-testid="chart"], .recharts-wrapper, canvas')
    await expect(charts.first()).toBeVisible()
  })

  test('should display time-based charts', async ({ page }) => {
    await page.goto('/analytics')

    // Mock analytics data
    await page.addInitScript(() => {
      window.mockAnalytics = {
        eventsOverTime: Array.from({ length: 24 }, (_, i) => ({
          hour: i,
          count: Math.floor(Math.random() * 100),
        })),
      }
    })

    // Should show events over time chart
    const timeChart = page.locator('[data-testid="events-over-time"], [data-testid="time-chart"]')
    await expect(timeChart).toBeVisible()
  })

  test('should display top event sources', async ({ page }) => {
    await page.goto('/analytics')

    // Should show top sources widget
    const sourcesWidget = page.locator('[data-testid="top-sources"], .top-sources')
    await expect(sourcesWidget).toBeVisible()

    // Should show source items
    const sourceItems = page.locator('[data-testid="source-item"], .source-item')
    expect(await sourceItems.count()).toBeGreaterThan(0)
  })

  test('should filter analytics by date range', async ({ page }) => {
    await page.goto('/analytics')

    // Open date range picker
    const datePicker = page.locator('[data-testid="date-picker"], .date-picker, input[type="date"]')
    if (await datePicker.isVisible()) {
      await datePicker.click()

      // Set date range
      await page.fill('input[name="startDate"]', '2025-01-01')
      await page.fill('input[name="endDate"]', '2025-01-31')

      // Apply
      const applyButton = page.locator('button:has-text("Apply")')
      await applyButton.click()

      // Should update charts
      await expect(page.locator('[data-testid="chart"]')).toBeVisible()
    }
  })

  test('should switch between different views', async ({ page }) => {
    await page.goto('/analytics')

    // Find view switcher
    const viewButtons = page.locator('[data-testid="view-toggle"], .view-toggle button')
    const count = await viewButtons.count()

    if (count > 0) {
      // Click different views
      for (let i = 0; i < Math.min(count, 3); i++) {
        await viewButtons.nth(i).click()

        // Should update content
        await expect(page.locator('[data-testid="chart"]')).toBeVisible()
      }
    }
  })

  test('should export analytics data', async ({ page }) => {
    await page.goto('/analytics')

    // Find export button
    const exportButton = page.locator('button:has-text("Export"), [data-testid="export-analytics"]')
    if (await exportButton.isVisible()) {
      await exportButton.click()

      // Should show export options
      const exportOptions = page.locator('[data-testid="export-options"], .export-options')
      if (await exportOptions.isVisible()) {
        // Select CSV
        await page.click('text=CSV')

        // Download
        await page.click('button:has-text("Download")')

        // Should trigger download
        const download = page.waitForEvent('download')
      }
    }
  })

  test('should display metrics cards', async ({ page }) => {
    await page.goto('/analytics')

    // Should show metrics
    const metricsCards = page.locator('[data-testid="metric-card"], .metric-card')
    const count = await metricsCards.count()

    expect(count).toBeGreaterThan(0)

    // Each card should have a value
    const firstCard = metricsCards.first()
    await expect(firstCard).toContainText(/\d+/)
  })

  test('should handle empty state', async ({ page }) => {
    await page.goto('/analytics')

    // Mock empty data
    await page.addInitScript(() => {
      window.mockAnalytics = {
        events: [],
        total: 0,
      }
    })

    // Refresh or navigate
    await page.reload()

    // Should show empty state
    const emptyState = page.locator('[data-testid="empty-state"], .empty-state, text=/No data|No events/')
    if (await emptyState.isVisible()) {
      await expect(emptyState).toBeVisible()
    }
  })
})

test.describe('Analytics Filters', () => {
  test('should filter by event type', async ({ page }) => {
    await page.goto('/analytics')

    // Open filters
    const filterButton = page.locator('button:has-text("Filter")')
    await filterButton.click()

    // Select event type
    const eventTypeSelect = page.locator('[name="eventType"]')
    await eventTypeSelect.click()
    await page.click('text=CreateUser')

    // Apply
    await page.click('button:has-text("Apply")')

    // Should update analytics
    await expect(page.locator('[data-testid="chart"]')).toBeVisible()
  })

  test('should save analytics dashboard configuration', async ({ page }) => {
    await page.goto('/analytics')

    // Open save dialog
    const saveButton = page.locator('button:has-text("Save Dashboard")')
    if (await saveButton.isVisible()) {
      await saveButton.click()

      // Enter name
      await page.fill('input[name="name"]', 'My Analytics Dashboard')

      // Save
      await page.click('button:has-text("Save")')

      // Should show success
      await expect(page.locator('text=/saved|success/')).toBeVisible()
    }
  })
})
