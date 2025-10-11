import { writable } from 'svelte/store';

export const solverReady = writable(false);
export const solving = writable(false);
export const solutions = writable([]);
export const topSolutions = writable({}); // Top 3 per word count for quick display
export const solveStats = writable({ totalReceived: 0, totalCount: 0, duration: null });

let currentSolveId = 0;
let worker = null;
let allSolutions = []; // Store all solutions for final sorting

// Helper to parse solution string
function parseSolution(solutionStr) {
  const parts = solutionStr.split(':');
  const words = parts[0];
  const score = parseInt(parts[1]) || 0;
  return { words, score, original: solutionStr };
}

// Helper to get word count from solution
function getWordCount(solutionStr) {
  const { words } = parseSolution(solutionStr);
  return words.split('-').length;
}

// Helper to update top 3 for a segment
function updateTopSolutions(topSols, newSolutions) {
  const updated = { ...topSols };

  newSolutions.forEach(sol => {
    const wordCount = getWordCount(sol);
    const parsed = parseSolution(sol);

    if (!updated[wordCount]) {
      updated[wordCount] = [];
    }

    // Add new solution
    updated[wordCount].push(sol);

    // Sort by score descending and keep top 3
    updated[wordCount].sort((a, b) => {
      const scoreA = parseSolution(a).score;
      const scoreB = parseSolution(b).score;
      return scoreB - scoreA;
    });

    updated[wordCount] = updated[wordCount].slice(0, 3);
  });

  return updated;
}

export function initializeSolverWorker(dictionaryData) {
  worker = new Worker(
    new URL('../workers/solver-worker.js', import.meta.url),
    { type: 'module' }
  );

  worker.addEventListener('message', (e) => {
    const { type, solveId, solutions: batchSolutions, totalReceived, totalCount, duration } = e.data;

    if (type === 'READY') {
      solverReady.set(true);
    }

    if (type === 'BATCH') {
      // Only process if this is the current solve
      if (solveId === currentSolveId) {
        // Add to allSolutions array
        allSolutions.push(...batchSolutions);

        // Update top 3 for quick display
        topSolutions.update(top => updateTopSolutions(top, batchSolutions));

        solveStats.update(stats => ({ ...stats, totalReceived }));
      }
    }

    if (type === 'COMPLETE') {
      if (solveId === currentSolveId) {
        // Final sort of all solutions by score
        allSolutions.sort((a, b) => {
          const scoreA = parseSolution(a).score;
          const scoreB = parseSolution(b).score;
          return scoreB - scoreA;
        });

        solutions.set(allSolutions);
        solving.set(false);
        solveStats.update(stats => ({ ...stats, totalCount, duration }));
      }
    }

    if (type === 'CANCELLED') {
      console.log(`Solve ${solveId} was cancelled. Received ${totalReceived} solutions.`);
    }

    if (type === 'ERROR') {
      console.error('Solver error:', e.data.error);
      solving.set(false);
    }
  });

  worker.postMessage({
    type: 'INIT',
    payload: { dictionaryData }
  });
}

export function solvePuzzle(sides, maxSolutions = 10000) {
  if (!worker) {
    console.error('Worker not initialized');
    return;
  }

  currentSolveId++;
  solving.set(true);
  solutions.set([]);
  topSolutions.set({});
  allSolutions = []; // Clear the solutions array
  solveStats.set({ totalReceived: 0, totalCount: 0, duration: null });

  worker.postMessage({
    type: 'SOLVE',
    solveId: currentSolveId,
    payload: { sides, maxSolutions }
  });
}

export function cancelSolve() {
  if (worker) {
    worker.postMessage({ type: 'CANCEL' });
    solving.set(false);
  }
}
