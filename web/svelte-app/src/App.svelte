<script>
  import { onMount } from 'svelte';
  import PuzzleLoader from './lib/PuzzleLoader.svelte';
  import LetterBox from './lib/LetterBox.svelte';
  import SolutionsDisplay from './lib/SolutionsDisplay.svelte';
  import ErrorDisplay from './lib/ErrorDisplay.svelte';
  import {
    puzzleFields,
    isPuzzleComplete,
    loadPuzzleFromStorage
  } from './stores/puzzle.js';
  import {
    initializeSolverWorker,
    solvePuzzle as workerSolvePuzzle,
    solverReady,
    solving,
    solutions,
    solveStats
  } from './stores/solver-worker.js';
  import { throttle } from './utils/throttle.js';

  let initError = null;

  onMount(async () => {
    // Load saved puzzle from localStorage
    loadPuzzleFromStorage();

    // Initialize solver worker with dictionary
    try {
      const response = await fetch('/dictionary.txt');
      const dictionaryText = await response.text();
      const dictionaryData = new TextEncoder().encode(dictionaryText);
      initializeSolverWorker(dictionaryData);
    } catch (error) {
      initError = error.message;
      console.error('Failed to initialize solver worker:', error);
    }
  });

  // Auto-solve with throttle when puzzle changes
  const throttledAutoSolve = throttle((fields) => {
    if (fields.every(f => f.length === 1 && /^[A-Z]$/.test(f))) {
      const sides = [
        fields.slice(0, 3).join(''),   // top
        fields.slice(3, 6).join(''),   // right
        fields.slice(9, 12).join(''),  // bottom
        fields.slice(6, 9).join('')    // left
      ].map(s => s.toLowerCase());

      workerSolvePuzzle(sides);
    }
  }, 300);

  // Subscribe to puzzle changes for auto-solve
  puzzleFields.subscribe(throttledAutoSolve);
</script>

<main>
  <h1>Letter Boxed Solver</h1>
  <p>Enter the letters for each side of the Letter Boxed puzzle. Solutions will appear automatically as you type.</p>

  {#if initError}
    <div class="error">Failed to initialize solver: {initError}</div>
  {/if}

  <div class="example">
    <PuzzleLoader />
  </div>

  <div class="container">
    <div class="game-input">
      <LetterBox />
    </div>

    {#if !$solverReady}
      <div class="status-message loading">Loading solver...</div>
    {/if}
  </div>

  <!-- Error display - always visible when there's an error -->
  <ErrorDisplay />

  <div class="container">
    <SolutionsDisplay />
  </div>
</main>

<style>
  :global(body) {
    font-family: Arial, sans-serif;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    line-height: 1.6;
  }

  main {
    width: 100%;
  }

  h1 {
    text-align: center;
  }

  .container {
    background: var(--color-bg-container);
    padding: 20px;
    border-radius: 8px;
    margin: 20px 0;
  }

  .example {
    background: var(--color-bg-example);
    padding: 15px;
    border-radius: 4px;
    margin: 20px 0;
  }

  .game-input {
    display: flex;
    justify-content: center;
    align-items: flex-start;
    margin: 20px 0;
  }

  .status-message {
    text-align: center;
    padding: 12px 20px;
    border-radius: 6px;
    font-size: 15px;
    margin: 15px 0;
    font-weight: 500;
    transition: all 0.3s ease;
  }

  .status-message.loading {
    background: var(--color-bg-light);
    color: var(--color-text-muted);
  }

  .error {
    color: var(--color-error);
    background: var(--color-error-bg);
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }
</style>
