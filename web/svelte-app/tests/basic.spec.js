import { test, expect } from '@playwright/test';

test('page loads successfully', async ({ page }) => {
  await page.goto('/');

  // Wait for the page to load
  await page.waitForLoadState('networkidle');

  // Check that the page title is set
  await expect(page).toHaveTitle(/Letter/);

  // Verify the page is visible and rendered
  const body = page.locator('body');
  await expect(body).toBeVisible();
});

test('letter box is present', async ({ page }) => {
  await page.goto('/');

  // Wait for the page to load
  await page.waitForLoadState('networkidle');

  // Check that letter input fields are present (should be 12 fields)
  const inputs = page.locator('input[type="text"]');
  await expect(inputs).toHaveCount(12);
});
