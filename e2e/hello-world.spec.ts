import { test, expect } from "@playwright/test";

// Basic browser-based test for the Tauri frontend
// This tests the web view content directly via Playwright
test.describe("GitHub PR List App", () => {
  test("should display login screen when not authenticated", async ({
    page,
  }) => {
    await page.goto("http://localhost:1420");

    // Wait for the welcome heading to be visible
    const heading = page.locator("h2");
    await expect(heading).toBeVisible();
    await expect(heading).toHaveText("Welcome to ghview");
  });

  test("should display sign in button", async ({ page }) => {
    await page.goto("http://localhost:1420");

    // Verify sign in button exists
    const signInButton = page.locator("button", {
      hasText: "Sign in with GitHub",
    });
    await expect(signInButton).toBeVisible();
  });
});
