import { writable, derived } from 'svelte/store';

// Puzzle fields store - array of 12 individual letters
// Layout: [0-2: top, 3-5: right, 6-8: left, 9-11: bottom]
export const puzzleFields = writable(Array(12).fill(''));

// Solutions store - array of solution strings
export const solutions = writable([]);

// Solver state
export const solverReady = writable(false);
export const solving = writable(false);
export const solveTime = writable(null);

// Derived store - check if puzzle is complete
export const isPuzzleComplete = derived(
  puzzleFields,
  $fields => $fields.every(field => field.length === 1 && /^[A-Z]$/.test(field))
);

// Flag to control auto-saving
let autoSaveEnabled = false;

// Load puzzle from localStorage
export function loadPuzzleFromStorage() {
  try {
    const saved = localStorage.getItem('letterBoxedPuzzle');
    if (saved) {
      const puzzle = JSON.parse(saved);
      if (puzzle.fields && Array.isArray(puzzle.fields) && puzzle.fields.length === 12) {
        puzzleFields.set(puzzle.fields);
      }
    }
  } catch (error) {
    console.warn('Failed to load saved puzzle:', error);
  } finally {
    // Enable auto-save after loading is complete
    autoSaveEnabled = true;
  }
}

// Save puzzle to localStorage
export function savePuzzleToStorage(fields) {
  try {
    localStorage.setItem('letterBoxedPuzzle', JSON.stringify({ fields }));
  } catch (error) {
    console.warn('Failed to save puzzle:', error);
  }
}

// Subscribe to save changes automatically (only after initial load)
puzzleFields.subscribe(fields => {
  if (autoSaveEnabled) {
    savePuzzleToStorage(fields);
  }
});
