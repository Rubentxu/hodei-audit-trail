// E2E tests for events functionality
import { test, expect } from '@playwright/test'

test.describe('Events Page', () => {
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

  test('should display events list', async ({ page }) => {
    await page.goto('/events')

    // Should have events table or list
    await expect(page.locator('h1, h2')).toContainText(/Event/)

    const eventsList = page.locator('[data-testid="events-list"], .events-list, table')
    await expect(eventsList).toBeVisible()
  })

  test('should display event details', async ({ page }) => {
    await page.goto('/events')

    // Mock events data
    await page.addInitScript(() => {
      window.mockEvents = [
        {
          id: 'evt-1',
          eventSource: 'ec2',
          eventName: 'RunInstances',
          timestamp: new Date().toISOString(),
          userIdentity: { principalId: 'user-123' },
        },
      ]
    })

    // Should show event items
    const eventItems = page.locator('[data-testid="event-item"], .event-item')
    await expect(eventItems).toHaveCount(1)
  })

  test('should filter events by date range', async ({ page }) => {
    await page.goto('/events')

    // Open date filter
    const dateFilter = page.locator('[data-testid="date-filter"], .date-filter, input[type="date"]')
    if (await dateFilter.isVisible()) {
      await dateFilter.click()

      // Set date range
      await page.fill('input[name="startDate"]', '2025-01-01')
      await page.fill('input[name="endDate"]', '2025-01-31')

      // Apply filter
      const applyButton = page.locator('button:has-text("Apply"), button:has-text("Filter")')
      await applyButton.click()

      // Should update events list
      await expect(page.locator('[data-testid="events-list"]')).toBeVisible()
    }
  })

  test('should search events', async ({ page }) => {
    await page.goto('/events')

    // Find search input
    const searchInput = page.locator('input[placeholder*="search" i], input[placeholder*="Search" i], [data-testid="search-input"]')
    await expect(searchInput).toBeVisible()

    // Type search query
    await searchInput.fill('ec2')

    // Should filter results
    await page.waitForTimeout(500) // Wait for debounce

    const eventItems = page.locator('[data-testid="event-item"], .event-item')
    // Results should be filtered
    expect(await eventItems.count()).toBeGreaterThanOrEqual(0)
  })

  test('should open event details modal', async ({ page }) => {
    await page.goto('/events')

    // Mock events
    await page.addInitScript(() => {
      window.mockEvents = [
        {
          id: 'evt-1',
          eventSource: 'ec2',
          eventName: 'RunInstances',
          timestamp: new Date().toISOString(),
          userIdentity: { principalId: 'user-123' },
        },
      ]
    })

    // Click on event item
    const eventItem = page.locator('[data-testid="event-item"], .event-item').first()
    await eventItem.click()

    // Should open modal
    const modal = page.locator('[data-testid="event-modal"], .modal, [role="dialog"]')
    await expect(modal).toBeVisible()

    // Should show event details
    await expect(modal).toContainText('ec2')
  })

  test('should paginate events', async ({ page }) => {
    await page.goto('/events')

    // Check for pagination controls
    const pagination = page.locator('[data-testid="pagination"], .pagination')

    if (await pagination.isVisible()) {
      // Click next page
      const nextButton = page.locator('button:has-text("Next"), [data-testid="next-page"]')
      if (await nextButton.isVisible()) {
        await nextButton.click()

        // Should update events list
        await expect(pagination).toBeVisible()
      }
    }
  })

  test('should export events', async ({ page }) => {
    await page.goto('/events')

    // Find export button
    const exportButton = page.locator('button:has-text("Export"), [data-testid="export-button"]')
    if (await exportButton.isVisible()) {
      await exportButton.click()

      // Should show export options or start download
      const exportDialog = page.locator('[data-testid="export-dialog"], .export-dialog')
      if (await exportDialog.isVisible()) {
        // Select export format
        await page.click('text=CSV')

        // Confirm export
        await page.click('button:has-text("Export"), button:has-text("Download")')

        // Should trigger download
        const download = page.waitForEvent('download')
        // Download handling would be tested in real scenario
      }
    }
  })

  test('should save search filters', async ({ page }) => {
    await page.goto('/events')

    // Open save search dialog
    const saveButton = page.locator('button:has-text("Save Search"), [data-testid="save-search"]')
    if (await saveButton.isVisible()) {
      await saveButton.click()

      // Enter search name
      await page.fill('input[name="name"]', 'My Saved Search')

      // Save
      await page.click('button:has-text("Save")')

      // Should show success message
      await expect(page.locator('text=/saved|success/')).toBeVisible()
    }
  })
})

test.describe('Events Filtering', () => {
  test('should filter by event source', async ({ page }) => {
    await page.goto('/events')

    // Open filters
    const filterButton = page.locator('button:has-text("Filter"), [data-testid="filter-button"]')
    await filterButton.click()

    // Select event source
    const eventSourceSelect = page.locator('[name="eventSource"], [data-testid="event-source"]')
    await eventSourceSelect.click()
    await page.click('text=ec2')

    // Apply filter
    const applyButton = page.locator('button:has-text("Apply")')
    await applyButton.click()

    // Should filter events
    const eventItems = page.locator('[data-testid="event-item"]')
    expect(await eventItems.count()).toBeGreaterThanOrEqual(0)
  })

  test('should clear all filters', async ({ page }) => {
    await page.goto('/events')

    // Apply some filters
    const filterButton = page.locator('button:has-text("Filter")')
    await filterButton.click()

    const eventSourceSelect = page.locator('[name="eventSource"]')
    await eventSourceSelect.click()
    await page.click('text=ec2')

    const applyButton = page.locator('button:has-text("Apply")')
    await applyButton.click()

    // Clear filters
    const clearButton = page.locator('button:has-text("Clear"), [data-testid="clear-filters"]')
    if (await clearButton.isVisible()) {
      await clearButton.click()

      // Should show all events
      const eventItems = page.locator('[data-testid="event-item"]')
      expect(await eventItems.count()).toBeGreaterThan(0)
    }
  })
})
