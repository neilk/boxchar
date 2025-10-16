import { writable, type Writable } from 'svelte/store';

interface SolveStats {
  totalReceived: number;
  totalCount: number;
  duration: number | null;
}

interface TopSolutionsByWordCount {
  [wordCount: number]: string[];
}

interface ParsedSolution {
  words: string;
  score: number;
  original: string;
}

interface WorkerMessage {
  type: string;
  solveId?: number;
  solutions?: string[];
  totalReceived?: number;
  totalCount?: number;
  duration?: number;
  error?: string;
}

export const solverReady: Writable<boolean> = writable(false);
export const solving: Writable<boolean> = writable(false);
export const solutions: Writable<string[]> = writable([]);
export const topSolutions: Writable<TopSolutionsByWordCount> = writable({}); // Top 3 per word count for quick display
export const solveStats: Writable<SolveStats> = writable({ totalReceived: 0, totalCount: 0, duration: null });
export const solverError: Writable<string | null> = writable(null); // Store for solver errors

let currentSolveId = 0;
let worker: Worker | null = null;
let allSolutions: string[] = []; // Store all solutions for final sorting

// Helper to parse solution string
function parseSolution(solutionStr: string): ParsedSolution {
  const parts = solutionStr.split(':');
  const words = parts[0] ?? '';
  const score = parseInt(parts[1] ?? '0') || 0;
  return { words, score, original: solutionStr };
}

// Helper to get word count from solution
function getWordCount(solutionStr: string): number {
  const { words } = parseSolution(solutionStr);
  return words.split('-').length;
}

// Helper to update top 3 for a segment
function updateTopSolutions(topSols: TopSolutionsByWordCount, newSolutions: string[]): TopSolutionsByWordCount {
  const updated = { ...topSols };

  newSolutions.forEach(sol => {
    const wordCount = getWordCount(sol);
    const parsed = parseSolution(sol);

    if (!updated[wordCount]) {
      updated[wordCount] = [];
    }

    // Add new solution
    updated[wordCount]!.push(sol);

    // Sort by score descending and keep top 3
    updated[wordCount]!.sort((a, b) => {
      const scoreA = parseSolution(a).score;
      const scoreB = parseSolution(b).score;
      return scoreB - scoreA;
    });

    updated[wordCount] = updated[wordCount]!.slice(0, 3);
  });

  return updated;
}

export function initializeSolverWorker(dictionaryData: Uint8Array): void {
  worker = new Worker(
    new URL('../workers/solver-worker.js', import.meta.url),
    { type: 'module' }
  );

  worker.addEventListener('message', (e: MessageEvent<WorkerMessage>) => {
    const { type, solveId, solutions: batchSolutions, totalReceived, totalCount, duration } = e.data;

    if (type === 'READY') {
      solverReady.set(true);
    }

    if (type === 'BATCH') {
      // Only process if this is the current solve
      if (solveId === currentSolveId && batchSolutions) {
        // Add to allSolutions array
        allSolutions.push(...batchSolutions);

        // Update top 3 for quick display
        topSolutions.update(top => updateTopSolutions(top, batchSolutions));

        if (totalReceived !== undefined) {
          solveStats.update(stats => ({ ...stats, totalReceived }));
        }
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
        if (totalCount !== undefined && duration !== undefined) {
          solveStats.update(stats => ({ ...stats, totalCount, duration }));
        }
      }
    }

    if (type === 'CANCELLED' && totalReceived !== undefined) {
      console.log(`Solve ${solveId} was cancelled. Received ${totalReceived} solutions.`);
    }

    if (type === 'ERROR') {
      console.error('Solver error:', e.data.error);
      solverError.set(e.data.error ?? 'Unknown error');
      solving.set(false);
    }
  });

  worker.postMessage({
    type: 'INIT',
    payload: { dictionaryData }
  });
}

export function solvePuzzle(sides: string[], maxSolutions = 10000): void {
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
  solverError.set(null); // Clear any previous errors

  worker.postMessage({
    type: 'SOLVE',
    solveId: currentSolveId,
    payload: { sides, maxSolutions }
  });
}

export function cancelSolve(): void {
  if (worker) {
    worker.postMessage({ type: 'CANCEL' });
    solving.set(false);
  }
}
