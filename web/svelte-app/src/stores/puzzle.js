import { writable, derived } from 'svelte/store';

// Puzzle sides store - array of 4 strings, each representing one side
export const puzzleSides = writable(['', '', '', '']);

// Solutions store - array of solution strings
export const solutions = writable([]);

// Solver state
export const solverReady = writable(false);
export const solving = writable(false);
export const solveTime = writable(null);

// Derived store - check if puzzle is complete
export const isPuzzleComplete = derived(
  puzzleSides,
  $sides => $sides.every(side => side.length === 3 && /^[A-Z]{3}$/.test(side))
);

// Load puzzle from localStorage
export function loadPuzzleFromStorage() {
  try {
    const saved = localStorage.getItem('letterBoxedPuzzle');
    if (saved) {
      const puzzle = JSON.parse(saved);
      if (puzzle.sides && Array.isArray(puzzle.sides)) {
        puzzleSides.set(puzzle.sides);
      }
    }
  } catch (error) {
    console.warn('Failed to load saved puzzle:', error);
  }
}

// Save puzzle to localStorage
export function savePuzzleToStorage(sides) {
  try {
    localStorage.setItem('letterBoxedPuzzle', JSON.stringify({ sides }));
  } catch (error) {
    console.warn('Failed to save puzzle:', error);
  }
}

// Subscribe to save changes automatically
puzzleSides.subscribe(sides => {
  savePuzzleToStorage(sides);
});
