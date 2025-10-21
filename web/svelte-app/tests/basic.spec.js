import { expect, test } from './fixture';

function getLetterBoxInputs(page) {
  return page.locator('.letter-box-container input[type="text"]');
}

test('page loads successfully', async ({ page }) => {

  // Check that the page title is set
  await expect(page).toHaveTitle(/Letter/);

  // Verify the page is visible and rendered
  const body = page.locator('body');
  await expect(body).toBeVisible();
});

test('letter box is present', async ({ page }) => {

  // Check that letter input fields are present (should be 12 fields)
  const inputs = getLetterBoxInputs(page);
  await expect(inputs).toHaveCount(12);
});

test('solving puzzle NUO,ERT,LCP,YIA finds neurotypical', async ({ page }) => {

  // Get all 12 input fields
  const inputs = getLetterBoxInputs(page);
  await expect(inputs).toHaveCount(12);

  // Enter the puzzle letters: NUO,ERT,LCP,YIA
  const letters = ['N', 'U', 'O', 'E', 'R', 'T', 'Y', 'I', 'A', 'L', 'C', 'P'];

  for (let i = 0; i < letters.length; i++) {
    await inputs.nth(i).fill(letters[i]);
  }

  // The shortest solution: "neurotypical"
  const solutionText = page.locator('text=/neurotypical/i');
  await expect(solutionText).toBeVisible({ timeout: 10000 });
});
