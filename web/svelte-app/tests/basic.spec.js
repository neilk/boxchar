import { expect, test } from './fixture';

function getLetterInputs(page) {
  return page.locator('.letter-box-container input[type="text"]');
}

async function enterPuzzle(page, letters) {
  const inputs = getLetterInputs(page);
  for (let i = 0; i < letters.length; i++) {
    await inputs.nth(i).fill(letters[i]);
  }
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
  const inputs = getLetterInputs(page);
  await expect(inputs).toHaveCount(12);
});

test('solve a puzzle', async ({ page }) => {

  await enterPuzzle(page, ['N', 'U', 'O', 'E', 'R', 'T', 'Y', 'I', 'A', 'L', 'C', 'P']);
  // The shortest solution: "neurotypical"
  const solutionText = page.locator('text=/neurotypical/i');
  await expect(solutionText).toBeVisible({ timeout: 10000 });
});

test('puzzle with no solutions', async ({ page }) => {

  // Enter a puzzle with no solutions - it has no vowels
  await enterPuzzle(page, ['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P']);

  // Wait for solving to complete and check for the "No solutions found!" message
  const noSolutionsMessage = page.locator('text=/No solutions found!/i');
  await expect(noSolutionsMessage).toBeVisible({ timeout: 10000 });
});


