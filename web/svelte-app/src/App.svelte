<script>
  import { onMount } from 'svelte';
  import PuzzleLoader from './lib/PuzzleLoader.svelte';
  import LetterBox from './lib/LetterBox.svelte';
  import SolutionsDisplay from './lib/SolutionsDisplay.svelte';
  import {
    puzzleFields,
    solutions,
    isPuzzleComplete,
    solveTime,
    solving,
    loadPuzzleFromStorage
  } from './stores/puzzle.js';
  import { initializeWasm, dictionaryInitialized, initializationError } from './stores/wasm.js';

  let solveFunction = null;
  let initError = null;

  onMount(async () => {
    // Load saved puzzle from localStorage
    loadPuzzleFromStorage();

    // Initialize WASM
    try {
      const { solve_game } = await initializeWasm();
      solveFunction = solve_game;
    } catch (error) {
      initError = error.message;
    }
  });

  async function solvePuzzle() {
    if (!solveFunction || !$dictionaryInitialized) {
      alert('Solver not ready yet. Please wait and try again.');
      return;
    }

    if (!$isPuzzleComplete) {
      alert('Please fill in all four sides of the puzzle.');
      return;
    }

    solving.set(true);
    solutions.set([]);
    solveTime.set(null);

    // Use setTimeout to allow UI to update
    setTimeout(() => {
      try {
        const startTime = performance.now();
        // Convert fields to sides format for WASM (top, right, bottom, left - clockwise)
        const sides = [
          $puzzleFields.slice(0, 3).join(''),   // top
          $puzzleFields.slice(3, 6).join(''),   // right
          $puzzleFields.slice(9, 12).join(''),  // bottom
          $puzzleFields.slice(6, 9).join('')    // left
        ].map(s => s.toLowerCase());
        const maxSolutions = 10000;

        console.dir({sides, solveFunction});
        const result = solveFunction(sides, maxSolutions);
        const endTime = performance.now();

        solutions.set(result);
        solveTime.set(Math.round(endTime - startTime));
      } catch (error) {
        solutions.set([`Error: ${error.message}`]);
      } finally {
        solving.set(false);
      }
    }, 10);
  }
</script>

<main>
  <h1>Letter Boxed Solver</h1>
  <p>Enter the letters for each side of the Letter Boxed puzzle. The solver will find word combinations that use all letters.</p>

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

    <button
      class="solve-btn"
      on:click={solvePuzzle}
      disabled={$solving || !$dictionaryInitialized}
    >
      {$solving ? 'Solving...' : !$dictionaryInitialized ? 'Loading...' : 'Solve Puzzle'}
    </button>
  </div>

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
    background: #f5f5f5;
    padding: 20px;
    border-radius: 8px;
    margin: 20px 0;
  }

  .example {
    background: #e7f3ff;
    padding: 15px;
    border-radius: 4px;
    margin: 20px 0;
  }

  .game-input {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 20px 0;
  }

  .solve-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 4px;
    font-size: 16px;
    cursor: pointer;
    margin: 10px 0;
    width: 100%;
  }

  .solve-btn:hover:not(:disabled) {
    background: #0056b3;
  }

  .solve-btn:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .error {
    color: #dc3545;
    background: #f8d7da;
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }
</style>
