import { test, expect } from "@playwright/test";

// Basic browser-based test for the Tauri frontend
// This tests the web view content directly via Playwright
test.describe("Hello World App", () => {
  test("should display Hello, World heading", async ({ page }) => {
    await page.goto("http://localhost:1420");

    // Wait for the heading to be visible
    const heading = page.locator("h1");
    await expect(heading).toBeVisible();
    await expect(heading).toHaveText("Hello, World");
  });

  test("should have correct page title structure", async ({ page }) => {
    await page.goto("http://localhost:1420");

    // Verify main container exists
    const container = page.locator("main.container");
    await expect(container).toBeVisible();
  });
});
