import { writable } from 'svelte/store';

export const solverReady = writable(false);
export const solving = writable(false);
export const solutions = writable([]);
export const solveStats = writable({ totalReceived: 0, totalCount: 0, duration: null });

let currentSolveId = 0;
let worker = null;

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
        solutions.update(arr => [...arr, ...batchSolutions]);
        solveStats.update(stats => ({ ...stats, totalReceived }));
      }
    }

    if (type === 'COMPLETE') {
      if (solveId === currentSolveId) {
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
