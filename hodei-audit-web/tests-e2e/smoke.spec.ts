// Basic smoke test
import { test, expect } from "@playwright/test";

test("should load home page", async ({ page }) => {
  await page.goto("/");
  const heading = page.locator("h1");
  await expect(heading).toContainText("Hodei Audit");
});

test("should load without errors", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("body")).toBeVisible();
});
